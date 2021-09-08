use std::{
    error,
    fmt::{self, Debug, Display, Formatter},
};

#[derive(Debug)]
pub enum Error {
    Save(SaveError),
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self {
            Error::Save(save_error) => Some(save_error),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            Error::Save(save_error) => {
                write!(f, "Error when saving an atlas data file: {}", save_error)
            }
        }
    }
}

#[derive(Debug)]
pub enum SaveError {
    Serialize(serde_json::error::Error),
}

impl error::Error for SaveError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self {
            SaveError::Serialize(json_error) => Some(json_error),
        }
    }
}

impl Display for SaveError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "An error occured when trying to save a config file:")?;

        match &self {
            SaveError::Serialize(json_error) => write!(
                f,
                "Error when serializing into a json file => {}",
                json_error
            ),
        }
    }
}

impl From<SaveError> for Error {
    fn from(error: SaveError) -> Self {
        Error::Save(error)
    }
}
