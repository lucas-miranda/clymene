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

use crate::processors::image::{
    self,
    format_handlers::FormatHandlerError
} ;

#[derive(Debug)]
pub enum Error {
    FileExpected,
    WrongFileType,
    PathAlreadyExists,
    ExternalProgramFail(Vec<u8>),
    InvalidInputPath(String),
    FormatHandlerFailed(FormatHandlerError),
    IO(io::Error)
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self {
            Error::FileExpected => None,
            Error::WrongFileType => None,
            Error::PathAlreadyExists => None,
            Error::ExternalProgramFail(_) => None,
            Error::InvalidInputPath(_) => None,
            Error::FormatHandlerFailed(handler_error) => Some(handler_error),
            Error::IO(io_error) => Some(io_error)
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            Error::FileExpected => write!(f, "File expected at path."),
            Error::WrongFileType => write!(f, "Wrong file type."),
            Error::PathAlreadyExists => write!(f, "Supplied path already exists."),
            Error::ExternalProgramFail(stderr) => {
                let msg = match std::str::from_utf8(stderr) {
                    Ok(err_msg) => err_msg,
                    Err(_) => "Can't display, malformed message."
                };

                write!(f, "An external program call has failed. Collected err:\n{}", msg)
            },
            Error::InvalidInputPath(path) => write!(f, "Invalid input path '{}'. Expected a valid folder.", path),
            Error::FormatHandlerFailed(handler_error) => write!(f, "Format handler has failed: {}", handler_error),
            Error::IO(io_error) => write!(f, "IO error: {}", io_error)
        }
    }
}

impl Into<image::Error> for Error {
    fn into(self) -> image::Error {
        image::Error::FormatHandler(self)
    }
}

