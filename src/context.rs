use anyhow::{anyhow, Context, Result};
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    hash::Hash,
    time::SystemTime,
};

use crate::{
    expirable::Expirable,
    mesh::Mesh,
    pipeline::{IOType, Input, Pipeline, Stage},
    preprocessor::preprocess_shader,
    shader::ShaderProgram,
    texture::Texture,
};

pub struct Ctx {
    pub pipeline: Expirable<Pipeline>,

    pub textures: HashMap<String, Expirable<Texture>>,
    pub shaders: HashMap<String, Expirable<ShaderProgram>>,

    pub default_shader: ShaderProgram,
    pub default_mesh: Mesh,
    pub reversed_mesh: Mesh,
}

const DEFAULT_VERTEX_SHADER: &str = include_str!("shaders/default.vert");
const DEFAULT_FRAGMENT_SHADER: &str = include_str!("shaders/default.frag");

impl Ctx {
    pub fn load_pipeline(file: &str, dir: &str) -> Result<Self> {
        let pipeline = Pipeline::load_from_file(&format!("{dir}/{file}"))?;

        let mut ctx = Self {
            pipeline: Expirable::now(pipeline),
            textures: HashMap::new(),
            shaders: HashMap::new(),
            default_shader: ShaderProgram::new(DEFAULT_VERTEX_SHADER, DEFAULT_FRAGMENT_SHADER)?,
            default_mesh: Mesh::default_plain(false),
            reversed_mesh: Mesh::default_plain(true),
        };

        ctx.refresh_stages(dir)?;

        Ok(ctx)
    }

    pub fn refresh_pipeline(&mut self, file: &str, dir: &str) -> Result<bool> {
        let path = format!("{dir}/{file}");
        let modified = file_modified(&path)?;

        let mut changed = false;

        if self.pipeline.expired(modified) {
            changed = true;
            println!("pipeline file expired");
            self.pipeline = Expirable::now(Pipeline::load_from_file(&path)?);
        }

        changed |= self.refresh_stages(dir)?;

        Ok(changed)
    }

    fn refresh_stages(&mut self, dir: &str) -> Result<bool> {
        let mut textures = HashSet::new();
        let mut shaders = HashSet::new();

        let mut changed = false;

        let pipeline = self.pipeline.data().clone();
        for stage in pipeline.pipeline.iter() {
            changed |= self.refresh_shader(stage, dir)?;
            shaders.insert(stage.shader.clone());

            for input in stage.inputs.iter() {
                changed |= self.refresh_input(input, dir, &mut textures)?;
            }

            match stage.output.typ {
                IOType::Memory => {
                    textures.insert(stage.output.name.clone());
                }
                IOType::File => (),
            }
        }

        drain_filter(&mut self.textures, |it| textures.contains(it));
        drain_filter(&mut self.shaders, |it| shaders.contains(it));

        Ok(changed)
    }

    fn refresh_shader(&mut self, stage: &Stage, dir: &str) -> Result<bool> {
        let fname = format!("{dir}/{}", stage.shader);
        let modified = file_modified(&fname)?;

        let shader = self.shaders.get(&stage.shader);
        if let Some(shader) = shader {
            if !shader.expired(modified) {
                return Ok(false);
            }
        }

        println!("Shader `{}` expired", stage.shader);

        let shader = preprocess_shader(
            &fname,
            &stage.debug_shader.as_ref().map(|path| format!("{dir}/{path}")),
        )?;

        let shader = ShaderProgram::new(DEFAULT_VERTEX_SHADER, &shader)
            .with_context(|| format!("Failed to create shader program: {fname}"))?;

        self.shaders
            .insert(stage.shader.to_string(), Expirable::now(shader));

        Ok(true)
    }

    fn refresh_input(&mut self, input: &Input, dir: &str, r: &mut HashSet<String>) -> Result<bool> {
        match input.typ {
            IOType::File => {
                let fname = format!("{dir}/{}", input.name);
                r.insert(input.name.clone());
                Ok(self.refresh_image(&fname, &input.name)?)
            }
            IOType::Memory => {
                if !r.contains(&input.name) {
                    return Err(anyhow!("Unknown resource in input: {}", input.name));
                }
                Ok(false)
            }
        }
    }

    fn refresh_image(&mut self, fname: &str, name: &str) -> Result<bool> {
        if let Some(texture) = self.textures.get(name) {
            let modified = file_modified(fname)?;

            if !texture.expired(modified) {
                return Ok(false);
            }
        }
        println!("Image `{}` expired", fname);

        let texture = Expirable::now(Texture::from_file(fname)?);
        self.textures.insert(name.to_string(), texture);

        Ok(true)
    }
}

fn file_modified(path: &str) -> Result<SystemTime> {
    let time = File::open(path)?.metadata()?.modified()?;
    Ok(time)
}

fn drain_filter<K, V, F>(map: &mut HashMap<K, V>, f: F)
where
    K: Clone + Eq + Hash,
    F: Fn(&K) -> bool,
{
    let textures_to_delete: Vec<K> = map
        .iter()
        .filter(|it| !f(it.0))
        .map(|it| it.0.clone())
        .collect();
    for t in textures_to_delete {
        map.remove(&t);
    }
}
