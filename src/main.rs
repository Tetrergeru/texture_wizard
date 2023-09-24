pub mod pipeline;

fn main() {
    let pipe = pipeline::Pipeline::load_from_file("examples/project.tw.yaml").unwrap();
    println!("pipe = {pipe:?}");
}
