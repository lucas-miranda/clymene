use std::{
    error,
    fmt::{
        self,
        Debug,
        Display,
        Formatter
    }
};

use super::{
    config,
    cache,
    image
};

#[derive(Debug)]
pub enum Error {
    ConfigProcessor(config::Error),
    CacheProcessor(cache::Error),
    ImageProcessor(image::Error)
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self {
            Error::ConfigProcessor(config_error) => Some(config_error),
            Error::CacheProcessor(cache_error) => Some(cache_error),
            Error::ImageProcessor(image_error) => Some(image_error)
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            Error::ConfigProcessor(config_error) => write!(f, "An error occurs at config processor: {}", config_error),
            Error::CacheProcessor(cache_error) => write!(f, "An error occurs at cache processor: {}", cache_error),
            Error::ImageProcessor(image_error) => write!(f, "An error occurs at image processor: {}", image_error)
        }
    }
}
