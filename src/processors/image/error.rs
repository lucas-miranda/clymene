use std::{
    error,
    fmt::{
        self,
        Debug,
        Display,
        Formatter
    },
    io
};

use crate::processors::{
    self,
    image::format_handlers
};

#[derive(Debug)]
pub enum Error {
    InvalidInputPath(String),
    InvalidOutputPath(String),
    IO(io::Error),
    FormatHandler(format_handlers::Error)
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self {
            Error::InvalidInputPath(_) => None,
            Error::InvalidOutputPath(_) => None,
            Error::IO(io_error) => Some(io_error),
            Error::FormatHandler(handler_error) => Some(handler_error)
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            Error::InvalidInputPath(path) => write!(f, "Invalid input path '{}'. Expected a valid folder.", path),
            Error::InvalidOutputPath(path) => write!(f, "Invalid output path '{}'. Expected a valid folder.", path),
            Error::IO(io_error) => write!(f, "IO error: {}", io_error),
            Error::FormatHandler(handler_error) => write!(f, "Format handler error: {}", handler_error)
        }
    }
}

impl Into<processors::Error> for Error {
    fn into(self) -> processors::Error {
        processors::Error::ImageProcessor(self)
    }
}

