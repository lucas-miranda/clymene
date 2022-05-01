use std::path::{Path, PathBuf};

mod atlas_output_stats;
mod error;
mod output_processor;
mod output_stats;

pub use atlas_output_stats::AtlasOutputStats;
pub use error::Error;
pub use output_processor::OutputProcessor;
pub use output_stats::OutputStats;

pub struct Output<'a> {
    pub atlas_width: u32,
    pub atlas_height: u32,
    files: Vec<OutputFile<'a>>,
}

impl<'a> Output<'a> {
    pub fn new(atlas_width: u32, atlas_height: u32) -> Self {
        Self {
            atlas_width,
            atlas_height,
            files: Vec::new(),
        }
    }

    pub fn register_file(&mut self, output_file: OutputFile<'a>) -> eyre::Result<()> {
        let metadata = output_file.path.metadata().map_err(eyre::Error::from)?;

        if !metadata.is_file() {
            return Err(Error::FileExpected.into());
        }

        self.files.push(output_file);

        Ok(())
    }

    pub fn files(&self) -> &Vec<OutputFile> {
        &self.files
    }

    pub fn set_atlas_size(&mut self, atlas_size: u32) {
        self.atlas_width = atlas_size;
        self.atlas_height = atlas_size;
    }
}

pub struct OutputFile<'a> {
    path: PathBuf,
    stats: Option<Box<dyn OutputStats + 'a>>,
}

impl<'a> OutputFile<'a> {
    pub fn new(path: PathBuf) -> Self {
        Self { path, stats: None }
    }

    pub fn with_stats<T: OutputStats + 'a>(path: PathBuf, stats: T) -> Self {
        Self {
            path,
            stats: Some(Box::new(stats)),
        }
    }

    pub fn path(&self) -> &Path {
        self.path.as_ref()
    }

    pub fn stats(&self) -> Option<&(dyn OutputStats + 'a)> {
        self.stats.as_deref()
    }
}
