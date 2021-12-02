use std::{
    error,
    fmt::{self, Debug, Display, Formatter},
};

#[derive(Debug)]
pub enum PackerError {
    EmptyTargetSize,
    OutOfSpace,
}

impl error::Error for PackerError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self {
            PackerError::EmptyTargetSize => None,
            PackerError::OutOfSpace => None,
        }
    }
}

impl Display for PackerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            PackerError::EmptyTargetSize => write!(f, "Target atlas size is empty"),
            PackerError::OutOfSpace => write!(f, "Atlas image is out of space"),
        }
    }
}
