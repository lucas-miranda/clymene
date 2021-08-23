use std::{
    error,
    fmt::{self, Debug, Display, Formatter},
};

use crate::processors::image::format_handlers;

#[derive(Debug)]
pub enum Error {
    FormatHandler(format_handlers::Error),
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self {
            Error::FormatHandler(handler_error) => Some(handler_error),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            Error::FormatHandler(handler_error) => {
                write!(f, "Format handler error: {}", handler_error)
            }
        }
    }
}
