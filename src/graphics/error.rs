use std::{
    error,
    fmt::{self, Debug, Display, Formatter},
    path::PathBuf,
};

use crate::processors::image::format_handlers;

#[derive(Debug)]
pub enum Error {
    FileExpected(PathBuf),
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self {
            Error::FileExpected(_path) => None,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            Error::FileExpected(path) => {
                write!(f, "Expecting a valid file at path '{}'.", path.display())
            }
        }
    }
}

impl From<Error> for format_handlers::Error {
    fn from(error: Error) -> Self {
        match error {
            Error::FileExpected(path) => format_handlers::Error::FileExpected(path),
        }
    }
}
