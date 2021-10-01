use serde::{Deserialize, Serialize};

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct CacheMetadata {
    pub version: String,
    pub data_prettified: bool,
}

impl Default for CacheMetadata {
    fn default() -> Self {
        Self {
            version: env!("CARGO_PKG_VERSION").to_owned(),
            data_prettified: false,
        }
    }
}

impl CacheMetadata {
    pub fn new(data_prettified: bool) -> Self {
        Self {
            version: env!("CARGO_PKG_VERSION").to_owned(),
            data_prettified,
        }
    }
}
