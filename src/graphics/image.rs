use crate::graphics::{Error, Graphic, GraphicSource};
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
    ) -> Result<Self, Error> {
        let metadata = source_path.metadata().unwrap();

        if !metadata.is_file() {
            return Err(Error::FileExpected(source_path));
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

impl From<Image> for Graphic {
    fn from(image: Image) -> Self {
        Graphic::Image(image)
    }
}
