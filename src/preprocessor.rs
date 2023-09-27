use std::{collections::HashMap, fs};

use anyhow::{Context, Result};

use lazy_static::lazy_static;

pub fn preprocess_shader(name: &str) -> Result<String> {
    let mut p = Preproseccor::new();
    
    let src = p.load_sorce(name)?.to_owned();

    Ok(src)
}

#[derive(Debug)]
struct Preproseccor {
    processed: HashMap<String, String>,
}

lazy_static! {
    static ref STANDARD_SHADERS: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("hash.glsl", include_str!("shaders/hash.glsl"));
        map.insert("random.glsl", include_str!("shaders/random.glsl"));
        map
    };
}

impl Preproseccor {
    fn new() -> Self {
        Self {
            processed: HashMap::new(),
        }
    }

    fn preprocess_shader(&mut self, name: &str, src: &str) -> Result<String> {
        let mut res = String::new();

        for l in src.lines() {
            if l.trim_start().starts_with("#include") {
                let fst = l.find('"').expect("Broken include syntax: missing `\"`");
                let lst = l.rfind('"').expect("Broken include syntax: missing `\"`");
                if fst == lst {
                    panic!("Broken include syntax: required two `\"`");
                }

                let incl_name = &l[fst + 1..lst];

                let included = self.load_sorce(incl_name)?;

                res.push_str(included);
            } else {
                res.push_str(l);
                res.push('\n');
            }
        }

        self.processed.insert(name.to_string(), res.clone());
        Ok(res)
    }

    fn load_sorce(&mut self, name: &str) -> Result<&'_ str> {
        if self.processed.contains_key(name) {
            return Ok(&self.processed[name]);
        }

        let processed = if STANDARD_SHADERS.contains_key(name) {
            let file = STANDARD_SHADERS[name];
            self.preprocess_shader(name, file)?
        } else {
            let file = fs::read_to_string(name);
            let file = file.with_context(|| format!("Loading shader file '{name}'"))?;
            self.preprocess_shader(name, &file)?
        };

        self.processed.insert(name.into(), processed);

        Ok(&self.processed[name])
    }
}
