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

use crate::processors;

#[derive(Debug)]
pub enum Error {
    InvalidOutputPath(String),
    IO(io::Error)
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self {
            Error::InvalidOutputPath(_) => None,
            Error::IO(io_error) => Some(io_error)
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            Error::InvalidOutputPath(path) => write!(f, "Invalid output path '{}'. Expected a valid folder.", path),
            Error::IO(io_error) => write!(f, "IO error: {}", io_error)
        }
    }
}

impl Into<processors::Error> for Error {
    fn into(self) -> processors::Error {
        processors::Error::ConfigProcessor(self)
    }
}
