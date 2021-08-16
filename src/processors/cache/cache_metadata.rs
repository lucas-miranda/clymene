use serde::{ 
    Serialize, 
    Deserialize 
};

use super::Error;

#[derive(Serialize, Deserialize)]
pub struct CacheMetadata {
    version: String
}

impl Default for CacheMetadata {
    fn default() -> Self {
        CacheMetadata {
            version: env!("CARGO_PKG_VERSION").to_owned()
        }
    }
}

impl CacheMetadata {
    pub fn expect_version(&self, expected_version: &str) -> Result<(), Error> {
        match self.version.eq(expected_version) {
            true => Ok(()),
            false => Err(
                Error::InvalidVersion {
                    version: self.version.to_owned(),
                    expected: expected_version.to_owned()
                }
            )
        }
    }
}
