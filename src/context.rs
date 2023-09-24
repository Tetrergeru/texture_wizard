use anyhow::Context;
use image::RgbaImage;
use std::{collections::HashMap, fs};

use crate::pipeline::{self, IOType};

pub struct Ctx {
    pub images: HashMap<String, RgbaImage>,
    pub shaders: HashMap<String, String>,
}

impl Ctx {
    pub fn new(pipe: &pipeline::Pipeline, dir: &str) -> anyhow::Result<Self> {
        let mut ctx = Self {
            images: HashMap::new(),
            shaders: HashMap::new(),
        };

        for stage in pipe.pipeline.iter() {
            ctx.load_shader(&format!("{dir}/{}", stage.shader))?;

            for input in stage.inputs.iter() {
                match input.typ {
                    IOType::File => ctx.load_image(&format!("{dir}/{}", input.name))?,
                    IOType::Memory => (),
                }
            }
        }

        Ok(ctx)
    }

    fn load_shader(&mut self, fname: &str) -> anyhow::Result<()> {
        let shader = fs::read_to_string(fname)
            .with_context(|| format!("Failed to read shader from '{}'", fname))?;

        self.shaders.insert(fname.to_string(), shader);

        Ok(())
    }

    fn load_image(&mut self, fname: &str) -> anyhow::Result<()> {
        if self.images.contains_key(fname) {
            return Ok(());
        }

        let image = image::open(fname)
            .with_context(|| format!("Failed to read image from '{}'", fname))?
            .into_rgba8();
        self.images.insert(fname.to_string(), image);

        Ok(())
    }
}
