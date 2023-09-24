use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Pipeline {
    pub pipeline: Vec<Stage>,
}

impl Pipeline {
    pub fn load_from_file(fname: &str) -> anyhow::Result<Self> {
        let pipeline = fs::read_to_string(fname)?;
        let pipeline = serde_yaml::from_str(&pipeline)?;
        Ok(pipeline)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Stage {
    pub shader: String,
    pub inputs: Vec<Input>,
    pub output: Output,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Input {
    #[serde(rename = "type")]
    pub typ: IOType,
    pub name: String,
    pub uniform: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Output {
    #[serde(rename = "type")]
    pub typ: IOType,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum IOType {
    #[serde(rename = "file")]
    File,
    #[serde(rename = "memory")]
    Memory
}