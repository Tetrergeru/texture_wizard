use std::time::SystemTime;

use anyhow::Result;
use chrono::{DateTime, Utc};

use crate::{
    context::Ctx,
    expirable::Expirable,
    pipeline::{Input, Pipeline, Preview, Profiling, Source, Stage},
    shader::ShaderProgram,
    texture::Texture,
};

pub fn execute_pipeline(ctx: &mut Ctx, pipe: &mut Expirable<Pipeline>, force: bool) -> Result<()> {
    if !ctx.refresh_pipeline(pipe)? && !force {
        let mut e = Executor { ctx };

        e.draw_previews(pipe.data());

        return Ok(());
    }

    if ctx.logs_enabled {
        let datetime: DateTime<Utc> = SystemTime::now().into();
        println!("Reexecuting pipeline {}", datetime.format("%Y.%m.%d/ %T"));
    }

    let mut e = Executor { ctx };

    for stage in pipe.data().pipeline.iter() {
        e.execute_stage(stage)?;
    }

    e.draw_previews(pipe.data());

    Ok(())
}

struct Executor<'a> {
    ctx: &'a mut Ctx,
}

impl<'a> Executor<'a> {
    fn draw_previews(&mut self, pipe: &Pipeline) {
        let mut preview = 0;

        for stage in pipe.pipeline.iter() {
            match stage.output.preview {
                Preview::Disabled => (),
                Preview::Simple => {
                    let texture = &self.ctx.textures[&stage.output.name];
                    self.draw_simple_preview(texture.data(), preview);
                    preview += 1;
                }
            }
        }
    }

    fn execute_stage(&mut self, stage: &Stage) -> Result<()> {
        let texture = Texture::from_size(stage.output.width, stage.output.height)?;
        texture.bind_as_canvas();

        let shader = &self.ctx.shaders[&stage.shader];
        shader.data().bind();

        let mut idx = 0;
        for input in stage.inputs.iter() {
            self.handle_input(input, &mut idx, shader.data())?;
        }

        let start = SystemTime::now();

        self.ctx.reversed_mesh.draw();

        let elapsed = start.elapsed()?;

        match stage.profiling {
            Profiling::Disabled => (),
            Profiling::Clock => {
                println!(
                    "Shader {} executed in {} sec",
                    stage.shader,
                    elapsed.as_secs_f64()
                );
            }
        }

        texture.unbind_as_canvas();

        self.handle_output(stage, texture)?;

        Ok(())
    }

    fn draw_simple_preview(&self, texture: &Texture, idx: isize) {
        self.ctx.default_shader.bind();
        texture.activate_bind(0);
        unsafe {
            gl::Viewport(idx as i32 * 200, 0, 200, 200);
        }
        self.ctx.default_mesh.draw();
    }

    fn handle_input(&self, input: &Input, idx: &mut i32, shader: &ShaderProgram) -> Result<()> {
        match input {
            Input::File { name, uniform } => {
                let texture = self.ctx.textures.get(name).unwrap();
                texture.data().activate_bind(*idx as u32);
                shader.uniform_1i(uniform, *idx)?;
            }
            Input::Memory { name, uniform } => {
                let texture = self.ctx.textures.get(name);
                if let Some(texture) = texture {
                    texture.data().activate_bind(*idx as u32);
                    shader.uniform_1i(uniform, *idx)?;

                    return Ok(());
                }

                let expr = self.ctx.variables.get(name).unwrap();
                shader.uniform_expr(uniform, expr)?;
            }
            Input::Expr { uniform, expr } => {
                shader.uniform_expr(uniform, expr)?;
            }
        }

        Ok(())
    }

    fn handle_output(&mut self, stage: &Stage, texture: Texture) -> Result<()> {
        match stage.output.dst {
            Source::File => {
                let fname = self.ctx.project_path.path(&stage.output.name);
                texture.save_to_file(&fname)?;
            }
            Source::Memory => (),
        }

        self.ctx
            .textures
            .insert(stage.output.name.clone(), Expirable::now(texture));

        Ok(())
    }
}
