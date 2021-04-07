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

use crate::processors::image::format_handlers;

#[derive(Debug)]
pub enum Error {
    FileExpected,
    DirectoryExpected,
    IO(io::Error),
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self {
            Error::FileExpected => None,
            Error::DirectoryExpected => None,
            Error::IO(io_error) => Some(io_error),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            Error::FileExpected => write!(f, "Expecting a valid file at path."),
            Error::DirectoryExpected => write!(f, "Expecting a valid directory at path."),
            Error::IO(io_error) => write!(f, "IO error: {}", io_error),
        }
    }
}

impl From<io::Error> for Error {
    fn from(io_error: io::Error) -> Self {
        Error::IO(io_error)
    }
}

impl Into<format_handlers::Error> for Error {
    fn into(self) -> format_handlers::Error {
        match self {
            Error::FileExpected => format_handlers::Error::FileExpected,
            Error::DirectoryExpected => format_handlers::Error::FileExpected,
            Error::IO(io_error) => format_handlers::Error::IO(io_error)
        }
    }
}
