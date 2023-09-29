use serde::{Deserialize, Serialize};

use super::Input;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Stage {
    pub shader: String,
    pub inputs: Vec<Input>,
    pub output: Output,
    #[serde(default)]
    pub debug_shader: Option<String>,
    #[serde(default)]
    pub profiling: Profiling,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Profiling {
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "clock")]
    Clock,
}

impl Default for Profiling {
    fn default() -> Self {
        Self::Disabled
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Output {
    pub dst: Source,
    pub name: String,
    pub width: u32,
    pub height: u32,
    #[serde(default)]
    pub preview: Preview,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Source {
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
