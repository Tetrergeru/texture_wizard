#![allow(clippy::single_match)]

use crate::preprocessor::Preproseccor;

pub mod context;
pub mod executor;
pub mod mesh;
pub mod pipeline;
pub mod shader;
pub mod texture;
pub mod preprocessor;

fn main() {
    let pipe = pipeline::Pipeline::load_from_file("examples/project.tw.yaml").unwrap();
    let previews = pipe.number_of_previews();

    let width = previews * 200;
    let height = 200_usize;

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

    unsafe {
        gl::Viewport(0, 0, width as gl::types::GLint, height as gl::types::GLint);
        // gl::ClearColor(0.3, 0.3, 0.5, 1.0);
        gl::Enable(gl::DEPTH_TEST);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    }

    executor::execute_pipeline(&pipe, "examples").unwrap();

    if previews == 0 {
        return
    }

    let mut event_pump = sdl.event_pump().unwrap();
    loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => return,
                _ => (),
            }
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            // gl::ClearColor(0.3, 0.3, 0.5, 1.0);
        }

        executor::execute_pipeline(&pipe, "examples").unwrap();

        window.gl_swap_window();
    }
}
