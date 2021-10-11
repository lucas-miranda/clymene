use std::{
    error,
    fmt::{self, Debug, Display, Formatter},
    io,
};

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    FileExpected,
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self {
            Error::Io(io_error) => Some(io_error),
            Error::FileExpected => None,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            Error::Io(io_error) => write!(f, "Io Error => {}", io_error),
            Error::FileExpected => write!(f, "File was expected at provided path"),
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        match e.kind() {
            io::ErrorKind::NotFound => Error::FileExpected,
            _ => Error::Io(e),
        }
    }
}
