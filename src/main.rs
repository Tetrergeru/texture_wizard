pub mod pipeline;
pub mod context;

fn main() {
    let pipe = pipeline::Pipeline::load_from_file("examples/project.tw.yaml").unwrap();
    let ctx = context::Ctx::new(&pipe, "examples").unwrap();
    println!("pipe = {pipe:?}");
    println!("ctx.images.len() = {}, ctx.shaders.len() = {}", ctx.images.len(), ctx.shaders.len());
}
