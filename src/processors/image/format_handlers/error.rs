use std::{
    error,
    fmt::{self, Debug, Display, Formatter},
    path::PathBuf,
};

use crate::processors::image::{self, format_handlers::FormatHandlerError};

#[derive(Debug)]
pub enum Error {
    FileExpected(PathBuf),
    DirectoryExpected,
    WrongFileType,
    ExternalProgramFail(Vec<u8>),
    FormatHandlerFailed(FormatHandlerError),
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self {
            Error::FileExpected(_path) => None,
            Error::DirectoryExpected => None,
            Error::WrongFileType => None,
            Error::ExternalProgramFail(_) => None,
            Error::FormatHandlerFailed(handler_error) => Some(handler_error),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            Error::FileExpected(path) => write!(f, "File expected at path '{}'.", path.display()),
            Error::DirectoryExpected => write!(f, "Directory expected at path."),
            Error::WrongFileType => write!(f, "Wrong file type."),
            Error::ExternalProgramFail(stderr) => {
                let msg =
                    std::str::from_utf8(stderr).unwrap_or("Can't display, malformed message.");

                write!(
                    f,
                    "An external program call has failed. Collected err:\n{}",
                    msg
                )
            }
            Error::FormatHandlerFailed(handler_error) => {
                write!(f, "Format handler has failed: {}", handler_error)
            }
        }
    }
}

impl From<Error> for image::Error {
    fn from(error: Error) -> Self {
        image::Error::FormatHandler(error)
    }
}
