use anyhow::Result;

use crate::{
    context::Ctx,
    mesh::Mesh,
    pipeline::{IOType, Pipeline, Stage},
    texture::Texture,
};

pub fn execute_pipeline(pipe: &Pipeline, dir: &str) -> Result<()> {
    let mut ctx = Ctx::new(pipe, dir)?;
    for stage in pipe.pipeline.iter() {
        execute_stage(&mut ctx, stage, dir)?;
    }
    Ok(())
}

fn execute_stage(ctx: &mut Ctx, stage: &Stage, dir: &str) -> Result<()> {
    let shader = ctx.shaders.get(&stage.shader).unwrap_or_else(|| {
        panic!(
            "unexpected shader name {}, all shaders: {:?}",
            stage.shader, ctx.shaders
        )
    });
    shader.bind();

    // println!("attrib = {}", shader.get_attrib_location("Position")?);

    let mesh = Mesh::default_plain();

    let texture = Texture::from_size(stage.output.width, stage.output.height)?;
    // if let IOType::Memory = stage.output.typ {
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
    // }

    for (idx, input) in stage.inputs.iter().enumerate() {
        let texture = ctx.textures.get(&input.name).unwrap();
        texture.activate_bind(idx as u32);
        shader.uniform_1i(&input.uniform, idx as i32)?;
    }

    // unsafe {
    //     gl::Viewport(0, 0, 500, 500);
    // }

    mesh.draw();

    match stage.output.typ {
        IOType::File => texture.save_to_file(&format!("{dir}/{}", stage.output.name))?,
        IOType::Memory => {
            ctx.textures.insert(stage.output.name.clone(), texture);
        }
    }

    Ok(())
}
