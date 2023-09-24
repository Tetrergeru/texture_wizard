use anyhow::{anyhow, Context, Result};
use std::{
    collections::{HashMap, HashSet},
    fs,
};

use crate::{
    pipeline::{self, IOType},
    shader::ShaderProgram,
    texture::Texture,
};

pub struct Ctx {
    pub textures: HashMap<String, Texture>,
    pub shaders: HashMap<String, ShaderProgram>,
}

const DEFAULT_VERTEX_SHADER: &str = include_str!("shaders/default.vert");

impl Ctx {
    pub fn new(pipe: &pipeline::Pipeline, dir: &str) -> Result<Self> {
        let mut ctx = Self {
            textures: HashMap::new(),
            shaders: HashMap::new(),
        };

        let mut results = HashSet::new();

        for stage in pipe.pipeline.iter() {
            ctx.load_shader(&format!("{dir}/{}", stage.shader))?;

            for input in stage.inputs.iter() {
                match input.typ {
                    IOType::File => ctx.load_image(&format!("{dir}/{}", input.name))?,
                    IOType::Memory => {
                        if !results.contains(&input.name) {
                            return Err(anyhow!("Unknown resource in input: {}", input.name));
                        }
                    }
                }
            }

            match stage.output.typ {
                IOType::Memory => {
                    results.insert(stage.output.name.clone());
                }
                IOType::File => (),
            }
        }

        Ok(ctx)
    }

    fn load_shader(&mut self, fname: &str) -> Result<()> {
        let shader = fs::read_to_string(fname)
            .with_context(|| format!("Failed to read shader from '{}'", fname))?;

        self.shaders.insert(
            fname.to_string(),
            ShaderProgram::new(DEFAULT_VERTEX_SHADER, &shader)
                .with_context(|| format!("Failed to create shader program: {fname}"))?,
        );

        Ok(())
    }

    fn load_image(&mut self, fname: &str) -> Result<()> {
        if self.textures.contains_key(fname) {
            return Ok(());
        }

        let texture = Texture::from_file(fname)?;
        self.textures.insert(fname.to_string(), texture);

        Ok(())
    }
}
