use std::{
    error,
    fmt::{self, Debug, Display, Formatter},
};

#[derive(Debug)]
pub enum SaveError {
    Serialize(toml::ser::Error),
}

impl error::Error for SaveError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self {
            SaveError::Serialize(toml_error) => Some(toml_error),
        }
    }
}

impl Display for SaveError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "An error occured when trying to save a config file:")?;

        match &self {
            SaveError::Serialize(toml_error) => write!(
                f,
                "Error when serializing into a toml file => {}",
                toml_error
            ),
        }
    }
}
