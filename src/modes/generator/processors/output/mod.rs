use std::path::{Path, PathBuf};

mod error;
mod output_processor;

pub use error::Error;
pub use output_processor::OutputProcessor;

pub struct Output {
    pub atlas_width: u32,
    pub atlas_height: u32,
    files: Vec<PathBuf>,
}

impl Output {
    pub fn new(atlas_width: u32, atlas_height: u32) -> Self {
        Self {
            atlas_width,
            atlas_height,
            files: Vec::new(),
        }
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

    pub fn set_atlas_size(&mut self, atlas_size: u32) {
        self.atlas_width = atlas_size;
        self.atlas_height = atlas_size;
    }
}
