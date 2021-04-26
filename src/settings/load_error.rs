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
pub enum LoadError {
    Deserialize(toml::de::Error),
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
        write!(f, "An error occured when trying to load a config file:")?;

        match &self {
            LoadError::Deserialize(toml_error) => write!(f, "Error when deserializing from a toml file => {}", toml_error),
            LoadError::FileNotFound(path) => write!(f, "File not found at '{}'", path.display())
        }
    }
}
