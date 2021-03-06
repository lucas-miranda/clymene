use std::{
    error,
    fmt::{self, Debug, Display, Formatter},
    path::PathBuf,
};

#[derive(Debug)]
pub enum Error {
    Load(LoadError),
    Save(SaveError),
    DirectoryExpected(PathBuf),
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self {
            Error::Load(load_error) => Some(load_error),
            Error::Save(save_error) => Some(save_error),
            Error::DirectoryExpected(_path) => None,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            Error::Load(load_error) => write!(f, "Error when loading a cache file: {}", load_error),
            Error::Save(save_error) => write!(f, "Error when saving a cache file: {}", save_error),
            Error::DirectoryExpected(path) => {
                write!(f, "Directory expected at path '{}'", path.display())
            }
        }
    }
}

#[derive(Debug)]
pub enum LoadError {
    Deserialize(serde_json::error::Error),
    FileNotFound(PathBuf),
}

impl error::Error for LoadError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self {
            LoadError::Deserialize(json_error) => Some(json_error),
            LoadError::FileNotFound(_path) => None,
        }
    }
}

impl Display for LoadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            LoadError::Deserialize(json_error) => write!(
                f,
                "Error when deserializing from a json file => {}",
                json_error
            ),
            LoadError::FileNotFound(path) => write!(f, "File not found at '{}'", path.display()),
        }
    }
}

impl From<LoadError> for Error {
    fn from(error: LoadError) -> Self {
        Error::Load(error)
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
