#![allow(clippy::single_match)]

pub mod context;
pub mod pipeline;
pub mod shader;

fn main() {
    let width = 500_usize;
    let height = 500_usize;

    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let window = video_subsystem
        .window("Texture Wizard", width as u32, height as u32)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();

    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    let pipe = pipeline::Pipeline::load_from_file("examples/project.tw.yaml").unwrap();
    let ctx = context::Ctx::new(&pipe, "examples").unwrap();
    println!("pipe = {pipe:?}");
    println!(
        "ctx.images.len() = {}, ctx.shaders.len() = {}",
        ctx.images.len(),
        ctx.shaders.len()
    );

    let mut event_pump = sdl.event_pump().unwrap();
    loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => return,
                _ => (),
            }
        }
    }
}
