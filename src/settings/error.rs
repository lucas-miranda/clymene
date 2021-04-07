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
pub enum Error {
    Load(LoadError),
    Save(SaveError)
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self {
            Error::Load(load_error) => Some(load_error),
            Error::Save(save_error) => Some(save_error)
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            Error::Load(load_error) => write!(f, "Error when loading a config file => {}", load_error),
            Error::Save(save_error) => write!(f, "Error when saving a config file => {}", save_error)
        }
    }
}


#[derive(Debug)]
pub enum LoadError {
    IO(io::Error),
    Deserialize(toml::de::Error)
}

impl error::Error for LoadError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self {
            LoadError::IO(io_error) => Some(io_error),
            LoadError::Deserialize(toml_error) => Some(toml_error)
        }
    }
}

impl Display for LoadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "An error occured when trying to load a config file:")?;

        match &self {
            LoadError::IO(io_error) => write!(f, "IO error => {}", io_error),
            LoadError::Deserialize(toml_error) => write!(f, "Error when deserializing from a toml file => {}", toml_error)
        }
    }
}



#[derive(Debug)]
pub enum SaveError {
    IO(io::Error),
    Serialize(toml::ser::Error)
}

impl error::Error for SaveError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self {
            SaveError::IO(io_error) => Some(io_error),
            SaveError::Serialize(toml_error) => Some(toml_error)
        }
    }
}

impl Display for SaveError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "An error occured when trying to save a config file:")?;

        match &self {
            SaveError::IO(io_error) => write!(f, "IO error => {}", io_error),
            SaveError::Serialize(toml_error) => write!(f, "Error when serializing into a toml file => {}", toml_error)
        }
    }
}
