use std::time::SystemTime;

use anyhow::Result;

use crate::{
    context::Ctx,
    pipeline::{IOType, Preview, Stage},
    texture::Texture, expirable::Expirable,
};

pub fn execute_pipeline(ctx: &mut Ctx, fname: &str, dir: &str, force: bool) -> Result<()> {
    if !ctx.refresh_pipeline(fname, dir)? && !force {
        return Ok(())
    }

    println!("Reexecuting pipeline {:?}", SystemTime::now());

    let mut e = Executor {
        dir: dir.to_string(),
        ctx,
        preview: 0,
    };

    let pipeline = e.ctx.pipeline.data().clone();
    for stage in pipeline.pipeline.iter() {
        e.execute_stage(stage)?;
    }
    Ok(())
}

struct Executor<'a> {
    dir: String,
    ctx: &'a mut Ctx,
    preview: usize,
}

impl<'a> Executor<'a> {
    fn execute_stage(&mut self, stage: &Stage) -> Result<()> {
        let texture = Texture::from_size(stage.output.width, stage.output.height)?;
        texture.bind_as_canvas();

        let shader = &self.ctx.shaders[&stage.shader];
        shader.data().bind();

        for (idx, input) in stage.inputs.iter().enumerate() {
            let texture = self.ctx.textures.get(&input.name).unwrap();
            texture.data().activate_bind(idx as u32);
            shader.data().uniform_1i(&input.uniform, idx as i32)?;
        }

        self.ctx.reversed_mesh.draw();

        texture.unbind_as_canvas();

        match stage.output.preview {
            Preview::Disabled => (),
            Preview::Simple => self.draw_simple_preview(&texture),
        }

        self.handle_output(stage, texture)?;

        Ok(())
    }

    fn draw_simple_preview(&mut self, texture: &Texture) {
        self.ctx.default_shader.bind();
        texture.activate_bind(0);
        unsafe {
            gl::Viewport(self.preview as i32 * 200, 0, 200, 200);
        }
        self.ctx.default_mesh.draw();

        self.preview += 1;
    }

    fn handle_output(&mut self, stage: &Stage, texture: Texture) -> Result<()> {
        match stage.output.typ {
            IOType::File => {
                let fname = format!("{}/{}", self.dir, stage.output.name);
                texture.save_to_file(&fname)?;
            }
            IOType::Memory => {
                self.ctx
                    .textures
                    .insert(stage.output.name.clone(), Expirable::now(texture));
            }
        }

        Ok(())
    }
}
