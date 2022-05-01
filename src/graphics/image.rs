use crate::graphics::{Error, GraphicSource};
use std::{ffi::OsString, path::PathBuf};

#[derive(Debug)]
pub struct Image {
    /// File name without extension.
    pub source_name: OsString,

    /// Source file path which yields this.
    pub source_path: PathBuf,

    /// Graphical source
    pub graphic_source: GraphicSource,
}

impl Image {
    pub fn with_graphic_source(
        graphic_source: GraphicSource,
        source_path: PathBuf,
    ) -> eyre::Result<Self> {
        let metadata = source_path.metadata().map_err(eyre::Report::from)?;

        if !metadata.is_file() {
            return Err(Error::FileExpected(source_path).into());
        }

        let source_name = source_path
            .file_stem()
            .ok_or_else(|| Error::FileExpected(source_path.clone()))?;

        Ok(Self {
            source_name: source_name.to_owned(),
            source_path,
            graphic_source,
        })
    }
}
