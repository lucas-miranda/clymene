use std::{
    error,
    fmt::{
        self,
        Debug,
        Display,
        Formatter
    },
    io,
    path::PathBuf
};

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    InvalidOutputDirectoryPath(PathBuf),
    OutputFilepathAlreadyInUse(PathBuf)
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self {
            Error::IO(io_error) => Some(io_error),
            _ => None
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            Error::IO(io_error) => write!(f, "IO Error: {}", io_error),
            Error::InvalidOutputDirectoryPath(path) => write!(f, "Expecting a valid directory at path '{}'.", path.display()),
            Error::OutputFilepathAlreadyInUse(path) => write!(f, "Path '{}' is already in use.", path.display())
        }
    }
}
