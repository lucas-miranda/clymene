use std::path::{Path, PathBuf};

mod error;
mod output_processor;

pub use error::Error;
pub use output_processor::OutputProcessor;

pub struct Output {
    files: Vec<PathBuf>,
}

impl Output {
    pub fn new() -> Self {
        Self { files: Vec::new() }
    }

    pub fn register_file(&mut self, path: &Path) -> Result<(), Error> {
        let metadata = path.metadata().map_err(Error::from)?;

        if !metadata.is_file() {
            return Err(Error::FileExpected);
        }

        self.files.push(path.to_owned());

        Ok(())
    }

    pub fn files(&self) -> &Vec<PathBuf> {
        &self.files
    }
}
