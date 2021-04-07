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

#[derive(Debug)]
pub enum FormatHandlerError {
    Deserialize(serde_json::error::Error),
    IO(io::Error)
}

impl error::Error for FormatHandlerError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self {
            FormatHandlerError::Deserialize(json_error) => Some(json_error),
            FormatHandlerError::IO(io_error) => Some(io_error)
        }
    }
}

impl Display for FormatHandlerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            FormatHandlerError::Deserialize(json_error) => write!(f, "Error when deserializing from a json file => {}", json_error),
            FormatHandlerError::IO(io_error) => write!(f, "IO error: {}", io_error)
        }
    }
}

impl Into<super::Error> for FormatHandlerError {
    fn into(self) -> super::Error {
        super::Error::FormatHandlerFailed(self)
    }
}
