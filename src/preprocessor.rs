use std::{
    collections::HashMap,
    fs,
};

use anyhow::Result;

#[derive(Debug)]
pub struct Preproseccor {
    processed: HashMap<String, String>,
}

const HASH_GLSL: &str = include_str!("shaders/hash.glsl");
const TEST_GLSL: &str = include_str!("shaders/test.glsl");

impl Preproseccor {
    pub fn new() -> Result<Self> {
        let mut slf = Self {
            processed: HashMap::new(),
        };

        slf.preprocess_shader("hash.glsl", &|| Ok(HASH_GLSL.to_string()))?;
        slf.preprocess_shader("test.glsl", &|| Ok(TEST_GLSL.to_string()))?;

        Ok(slf)
    }

    pub fn preprocess_shader(&mut self, name: &str, src: &dyn Fn() -> Result<String>) -> Result<String>
    {
        if self.processed.contains_key(name) {
            return Ok(self.processed[name].clone());
        }


        println!("name = '{name}'");
        let mut res = String::new();
        let src = src()?;

        for l in src.lines() {
            if l.trim_start().starts_with("#include") {
                let fst = l.find('"').expect("Broken include syntax: missing '\"'");
                let lst = l.rfind('"').expect("Broken include syntax: missing '\"'");
                if fst == lst {
                    panic!("Broken include syntax: missing ");
                }

                let incl_name = &l[fst+1..lst];
                println!("incl_name = '{incl_name}'");

                res.push_str(
                    &self.preprocess_shader(incl_name, &|| Ok(fs::read_to_string(incl_name)?))?,
                )
            } else {
                res.push_str(l);
                res.push('\n');
            }
        }

        self.processed.insert(name.to_string(), res.clone());
        Ok(res)
    }
}

