pub struct ProjectPath {
    dir: String,
    fname: String,
}

impl ProjectPath {
    pub fn new(dir: &str, fname: &str) -> Self {
        Self {
            dir: dir.into(),
            fname: fname.into(),
        }
    }

    pub fn path(&self, fname: &str) -> String {
        format!("{}/{fname}", self.dir)
    }

    pub fn main(&self) -> String {
        self.path(&self.fname)
    }
}
