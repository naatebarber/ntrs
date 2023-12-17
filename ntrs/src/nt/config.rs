#[derive(Debug, PartialEq)]
pub struct Path {
    pub src: String,
    pub dest: String,
}

#[derive(Debug)]
pub struct NTConfig {
    pub host: String,
    pub port: usize,
    paths: Vec<Path>,
}

impl NTConfig {
    pub fn new(host: String, port: usize) -> NTConfig {
        NTConfig {
            host,
            port,
            paths: vec![],
        }
    }

    pub fn with(&mut self, path: Path) -> &mut Self {
        self.paths.push(path);
        self
    }

    pub fn without(&mut self, path: Path) -> &mut Self {
        self.paths.retain(|x| *x != path);
        self
    }

    pub fn get_dest(&self, src: String) -> Option<String> {
        match self.paths.iter().find(|&x| x.src == src) {
            Some(path) => Some(String::from(&path.dest)),
            _ => None,
        }
    }
}