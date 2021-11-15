use std::{
    error,
    fmt::{self, Debug, Display, Formatter},
    io,
};

use image::error::ImageError;

#[derive(Debug)]
pub enum ValidationError {
    AtlasImageLoadFailed(ImageError),
    AtlasImageNotFound,
    AtlasImageIoError(io::Error),
    CacheNotUpdated,
    PreviousFileImageSizeMismatch,
}

impl error::Error for ValidationError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self {
            ValidationError::AtlasImageLoadFailed(image_error) => Some(image_error),
            ValidationError::AtlasImageNotFound => None,
            ValidationError::AtlasImageIoError(io_error) => Some(io_error),
            ValidationError::CacheNotUpdated => None,
            ValidationError::PreviousFileImageSizeMismatch => None,
        }
    }
}

impl Display for ValidationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            ValidationError::AtlasImageLoadFailed(image_error) => {
                write!(f, "Failed to load image: {}", image_error)
            }
            ValidationError::AtlasImageNotFound => write!(f, "Atlas image file not found"),
            ValidationError::AtlasImageIoError(io_error) => {
                write!(f, "Atlas image io error: {}", io_error)
            }
            ValidationError::CacheNotUpdated => write!(f, "Cache isn't updated"),
            ValidationError::PreviousFileImageSizeMismatch => write!(
                f,
                "Previous output file image size differs from current size"
            ),
        }
    }
}
