use std::{
    error,
    fmt::{self, Debug, Display, Formatter},
};

#[derive(Debug)]
pub enum FormatHandlerError {
    Deserialize(serde_json::error::Error),
}

impl error::Error for FormatHandlerError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self {
            FormatHandlerError::Deserialize(json_error) => Some(json_error),
        }
    }
}

impl Display for FormatHandlerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            FormatHandlerError::Deserialize(json_error) => write!(
                f,
                "Error when deserializing from a json file => {}",
                json_error
            ),
        }
    }
}

impl From<FormatHandlerError> for super::Error {
    fn from(error: FormatHandlerError) -> Self {
        super::Error::FormatHandlerFailed(error)
    }
}
