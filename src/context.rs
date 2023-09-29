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
    pipeline::{IOType, Input, Pipeline, Preview, Stage},
    preprocessor::preprocess_shader,
    project_path::ProjectPath,
    shader::ShaderProgram,
    texture::Texture,
};

pub struct Ctx {
    pub project_path: ProjectPath,

    // pub pipeline: Expirable<Pipeline>,
    pub textures: HashMap<String, Expirable<Texture>>,
    pub shaders: HashMap<String, Expirable<ShaderProgram>>,

    pub default_shader: ShaderProgram,
    pub default_mesh: Mesh,
    pub reversed_mesh: Mesh,

    pub logs_enabled: bool,
}

const DEFAULT_VERTEX_SHADER: &str = include_str!("shaders/default.vert");
const DEFAULT_FRAGMENT_SHADER: &str = include_str!("shaders/default.frag");

impl Ctx {
    pub fn load(project_path: ProjectPath, pipe: &Expirable<Pipeline>) -> Result<Self> {
        let mut ctx = Self {
            project_path,
            textures: HashMap::new(),
            shaders: HashMap::new(),
            default_shader: ShaderProgram::new(DEFAULT_VERTEX_SHADER, DEFAULT_FRAGMENT_SHADER)?,
            default_mesh: Mesh::default_plain(false),
            reversed_mesh: Mesh::default_plain(true),
            logs_enabled: true,
        };

        ctx.refresh_stages(pipe)?;

        Ok(ctx)
    }

    pub fn refresh_pipeline(&mut self, pipe: &mut Expirable<Pipeline>) -> Result<bool> {
        let path = self.project_path.main();
        let modified = file_modified(&path)?;

        let mut changed = false;

        if pipe.expired(modified) {
            changed = true;
            if self.logs_enabled {
                println!("pipeline file expired");
            }
            *pipe = Expirable::now(Pipeline::load_from_file(&self.project_path)?);
        }

        changed |= self.refresh_stages(pipe)?;

        Ok(changed)
    }

    fn refresh_stages(&mut self, pipe: &Expirable<Pipeline>) -> Result<bool> {
        let mut textures = HashSet::new();
        let mut shaders = HashSet::new();

        let mut changed = false;

        for stage in pipe.data().pipeline.iter() {
            changed |= self.refresh_shader(stage)?;
            shaders.insert(stage.shader.clone());

            for input in stage.inputs.iter() {
                changed |= self.refresh_input(input, &mut textures)?;
            }

            match stage.output.typ {
                IOType::Memory => {
                    textures.insert(stage.output.name.clone());
                }
                IOType::File => (),
            }

            match stage.output.preview {
                Preview::Disabled => (),
                Preview::Simple => {
                    textures.insert(stage.output.name.clone());
                }
            }
        }

        drain_filter(&mut self.textures, |it| textures.contains(it));
        drain_filter(&mut self.shaders, |it| shaders.contains(it));

        Ok(changed)
    }

    fn refresh_shader(&mut self, stage: &Stage) -> Result<bool> {
        let fname = self.project_path.path(&stage.shader);
        let modified = file_modified(&fname)?;

        let shader = self.shaders.get(&stage.shader);
        if let Some(shader) = shader {
            if !shader.expired(modified) {
                return Ok(false);
            }
        }

        if self.logs_enabled {
            println!("Shader `{}` expired", stage.shader);
        }
        let shader = preprocess_shader(
            &fname,
            &stage
                .debug_shader
                .as_ref()
                .map(|path| self.project_path.path(path)),
        )?;

        let shader = ShaderProgram::new(DEFAULT_VERTEX_SHADER, &shader)
            .with_context(|| format!("Failed to create shader program: {fname}"))?;

        self.shaders
            .insert(stage.shader.to_string(), Expirable::now(shader));

        Ok(true)
    }

    fn refresh_input(&mut self, input: &Input, r: &mut HashSet<String>) -> Result<bool> {
        match input.typ {
            IOType::File => {
                let fname = self.project_path.path(&input.name);
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

        if self.logs_enabled {
            println!("Image `{}` expired", fname);
        }
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
