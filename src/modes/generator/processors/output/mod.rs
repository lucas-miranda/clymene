use std::path::{Path, PathBuf};

mod output_processor;
pub use output_processor::OutputProcessor;

pub struct Output {
    files: Vec<PathBuf>,
}

impl Output {
    pub fn new() -> Self {
        Self { files: Vec::new() }
    }

    pub fn register_file(&mut self, path: &Path) {
        if !path.is_file() {
            panic!("There is no file at '{}'", path.display())
        }

        self.files.push(path.to_owned());
    }

    pub fn files(&self) -> &Vec<PathBuf> {
        &self.files
    }
}
