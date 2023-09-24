use anyhow::Result;

use crate::{
    context::Ctx,
    mesh::Mesh,
    pipeline::{IOType, Pipeline, Preview, Stage},
    texture::Texture,
};

pub fn execute_pipeline(pipe: &Pipeline, dir: &str) -> Result<()> {
    let ctx = Ctx::new(pipe, dir)?;

    let mut e = Executor {
        dir: dir.to_string(),
        ctx,
        preview: 0,
    };

    for stage in pipe.pipeline.iter() {
        e.execute_stage(stage)?;
    }
    Ok(())
}

struct Executor {
    dir: String,
    ctx: Ctx,
    preview: usize,
}

impl Executor {
    fn execute_stage(&mut self, stage: &Stage) -> Result<()> {
        let shader = self.ctx.shaders.get(&stage.shader).unwrap_or_else(|| {
            panic!(
                "unexpected shader name {}, all shaders: {:?}",
                stage.shader, self.ctx.shaders
            )
        });
        shader.bind();

        let mesh = Mesh::default_plain();

        let texture = Texture::from_size(stage.output.width, stage.output.height)?;
        let mut framebuffer: u32 = 0;
        unsafe {
            gl::CreateFramebuffers(1, &mut framebuffer);
            gl::BindFramebuffer(gl::FRAMEBUFFER, framebuffer);
            gl::FramebufferTexture2D(
                gl::FRAMEBUFFER,
                gl::COLOR_ATTACHMENT0,
                gl::TEXTURE_2D,
                texture.get_id(),
                0,
            );
            gl::Viewport(0, 0, stage.output.width as i32, stage.output.height as i32);
        };

        for (idx, input) in stage.inputs.iter().enumerate() {
            let texture = self.ctx.textures.get(&input.name).unwrap();
            texture.activate_bind(idx as u32);
            shader.uniform_1i(&input.uniform, idx as i32)?;
        }

        mesh.draw();

        match stage.output.typ {
            IOType::File => texture.save_to_file(&format!("{}/{}", self.dir, stage.output.name))?,
            IOType::Memory => {
                self.ctx.textures.insert(stage.output.name.clone(), texture);
            }
        }

        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }

        match stage.output.preview {
            Preview::Disabled => (),
            Preview::Simple => unsafe {
                gl::Viewport(
                    self.preview as i32 * 200,
                    0,
                    200,
                    200,
                );
                self.ctx.default_shader.bind();
                mesh.draw();
                self.preview += 1;
            },
        }

        Ok(())
    }
}
