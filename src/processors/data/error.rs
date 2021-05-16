use std::{
    error,
    fmt::{
        self,
        Debug,
        Display,
        Formatter
    },
    path::PathBuf
};

#[derive(Debug)]
pub enum Error {
    Load(LoadError),
    Save(SaveError),
    DirectoryExpected(PathBuf),
    InvalidVersion { version: String, expected: String }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self {
            Error::Load(load_error) => Some(load_error),
            Error::Save(save_error) => Some(save_error),
            Error::DirectoryExpected(_path) => None,
            Error::InvalidVersion { .. } => None
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            Error::Load(load_error) => write!(f, "Error when loading an atlas data file: {}", load_error),
            Error::Save(save_error) => write!(f, "Error when saving an atlas data file: {}", save_error),
            Error::DirectoryExpected(path) => write!(f, "Directory expected at path '{}'", path.display()),
            Error::InvalidVersion { version, expected } => write!(f, "Invalid cache version '{}', expected version '{}'", version, expected)
        }
    }
}



#[derive(Debug)]
pub enum LoadError {
    Deserialize(serde_json::error::Error),
    FileNotFound(PathBuf)
}

impl error::Error for LoadError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self {
            LoadError::Deserialize(toml_error) => Some(toml_error),
            LoadError::FileNotFound(_path) => None
        }
    }
}

impl Display for LoadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "An error occured when trying to load a atlas data file:")?;

        match &self {
            LoadError::Deserialize(toml_error) => write!(f, "Error when deserializing from a toml file => {}", toml_error),
            LoadError::FileNotFound(path) => write!(f, "File not found at '{}'", path.display())
        }
    }
}

impl Into<Error> for LoadError {
    fn into(self) -> Error {
        Error::Load(self)
    }
}



#[derive(Debug)]
pub enum SaveError {
    Serialize(serde_json::error::Error)
}

impl error::Error for SaveError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self {
            SaveError::Serialize(json_error) => Some(json_error)
        }
    }
}

impl Display for SaveError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "An error occured when trying to save a config file:")?;

        match &self {
            SaveError::Serialize(json_error) => write!(f, "Error when serializing into a json file => {}", json_error)
        }
    }
}

impl Into<Error> for SaveError {
    fn into(self) -> Error {
        Error::Save(self)
    }
}
