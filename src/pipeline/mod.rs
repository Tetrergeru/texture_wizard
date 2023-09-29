mod input;
#[cfg(test)]
pub mod input_test;
mod stage;

use std::{collections::HashMap, fs};

pub use input::{Expr, Input};

use serde::{Deserialize, Serialize};
pub use stage::*;

use crate::project_path::ProjectPath;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Pipeline {
    pub pipeline: Vec<Stage>,
    pub variables: HashMap<String, Expr>,
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
                continue;
            }
            res += 1;
        }
        res
    }
}
