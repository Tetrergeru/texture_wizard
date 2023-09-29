use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "src")]
pub enum Input {
    #[serde(rename = "file")]
    File { name: String, uniform: String },
    #[serde(rename = "memory")]
    Memory { name: String, uniform: String },
    #[serde(rename = "expr")]
    Expr { uniform: String, expr: Expr },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum Expr {
    Float(f32),
    Vec2([f32; 2]),
    Vec3([f32; 3]),
    Vec4([f32; 4]),
    String(String),
}
