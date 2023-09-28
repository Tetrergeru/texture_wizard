use std::fs;

use serde::{Deserialize, Serialize};

use crate::project_path::ProjectPath;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Pipeline {
    pub pipeline: Vec<Stage>,
}

impl Pipeline {
    pub fn load_from_file(path: &ProjectPath) -> anyhow::Result<Self> {
        let pipeline = fs::read_to_string(path.main())?;
        let pipeline = serde_yaml::from_str(&pipeline)?;
        Ok(pipeline)
    }

    pub fn number_of_previews(&self) -> usize {
        let mut res = 0;
        for stage in self.pipeline.iter() {
            if let Preview::Disabled = stage.output.preview {
                continue
            }
            res += 1;
        }
        res
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Stage {
    pub shader: String,
    pub inputs: Vec<Input>,
    pub output: Output,
    #[serde(default)]
    pub debug_shader: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Input {
    #[serde(rename = "type")]
    pub typ: IOType,
    pub name: String,
    pub uniform: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Output {
    #[serde(rename = "type")]
    pub typ: IOType,
    pub name: String,
    pub width: u32,
    pub height: u32,
    #[serde(default)]
    pub preview: Preview,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum IOType {
    #[serde(rename = "file")]
    File,
    #[serde(rename = "memory")]
    Memory,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Preview {
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "simple")]
    Simple,
}

impl Default for Preview {
    fn default() -> Self {
        Self::Disabled
    }
}