use std::{
    error,
    fmt::{self, Debug, Display, Formatter},
    io,
    path::Path,
};

use crate::{
    graphics::GraphicSource, math::Rectangle, modes::generator::processors::data::FrameData,
};

pub struct GraphicSourceData {
    pub source: GraphicSource,
    pub frame_index: u32,
}

impl GraphicSourceData {
    pub fn try_from_path(
        path: &Path,
        frames_data: &[FrameData],
    ) -> Result<Self, GraphicSourceDataError> {
        let metadata = path.metadata().map_err(GraphicSourceDataError::from)?;

        if !metadata.is_file() {
            return Err(GraphicSourceDataError::FileExpected);
        }

        let frame_index = match try_retrieve_frame_index(path) {
            Some(index) => index,
            None => return Err(GraphicSourceDataError::FrameIndexNotFound),
        };

        let (source_region, atlas_region) = get_regions(frame_index, frames_data);

        let buffer = {
            let dyn_image = image::open(path).map_err(GraphicSourceDataError::GraphicLoadError)?;

            match dyn_image {
                image::DynamicImage::ImageRgba8(rgba_image) => Ok(rgba_image),
                _ => Err(GraphicSourceDataError::UnsupportedGraphicFormat),
            }
        }?;

        Ok(GraphicSourceData {
            source: GraphicSource {
                atlas_region,
                buffer,
                region: source_region,
            },
            frame_index,
        })
    }
}

fn try_retrieve_frame_index(path: &Path) -> Option<u32> {
    if let Some(stem) = path.file_stem() {
        if let Some(stem_str) = stem.to_str() {
            if let Ok(index) = stem_str.parse() {
                return Some(index);
            }
        }
    }

    None
}

fn get_regions(
    frame_index: u32,
    frames_data: &[FrameData],
) -> (Rectangle<u32>, Option<Rectangle<u32>>) {
    let result_source_region;
    let result_atlas_region;

    match frames_data.get(frame_index as usize) {
        Some(frame_data) => match frame_data {
            FrameData::Empty => {
                result_source_region = Rectangle::default();
                result_atlas_region = None;
            }
            FrameData::Contents {
                atlas_region,
                source_region,
                ..
            } => {
                result_source_region = Rectangle::with(
                    source_region.x,
                    source_region.y,
                    source_region.width,
                    source_region.height,
                )
                .unwrap_or_else(Rectangle::default);

                result_atlas_region = if !atlas_region.is_empty() {
                    Some(
                        Rectangle::with(
                            atlas_region.x,
                            atlas_region.y,
                            atlas_region.width,
                            atlas_region.height,
                        )
                        .unwrap_or_else(Rectangle::default),
                    )
                } else {
                    None
                };
            }
        },
        None => {
            result_source_region = Rectangle::default();
            result_atlas_region = None;
        }
    }

    (result_source_region, result_atlas_region)
}

#[derive(Debug)]
pub enum GraphicSourceDataError {
    IO(io::Error),
    FileExpected,
    FrameIndexNotFound,
    GraphicLoadError(image::ImageError),
    UnsupportedGraphicFormat,
}

impl error::Error for GraphicSourceDataError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self {
            GraphicSourceDataError::IO(io_error) => Some(io_error),
            GraphicSourceDataError::GraphicLoadError(image_error) => Some(image_error),
            _ => None,
        }
    }
}

impl Display for GraphicSourceDataError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            GraphicSourceDataError::IO(io_error) => write!(f, "IO Error: {}", io_error),
            GraphicSourceDataError::FileExpected => write!(f, "File expected"),
            GraphicSourceDataError::FrameIndexNotFound => write!(f, "Frame index was not found"),
            GraphicSourceDataError::GraphicLoadError(image_error) => {
                write!(f, "Graphic load error: {}", image_error)
            }
            GraphicSourceDataError::UnsupportedGraphicFormat => {
                write!(f, "Supplied graphic format isn't supported.")
            }
        }
    }
}

impl From<io::Error> for GraphicSourceDataError {
    fn from(e: io::Error) -> Self {
        GraphicSourceDataError::IO(e)
    }
}
