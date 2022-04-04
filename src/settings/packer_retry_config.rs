use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PackerRetryConfig {
    #[serde(default)]
    pub enable: bool,

    #[serde(default)]
    pub max_retries: u32,

    #[serde(default)]
    pub until_atlas_size: u32,
}

impl Default for PackerRetryConfig {
    fn default() -> Self {
        Self {
            enable: true,
            max_retries: 3,
            until_atlas_size: 0
        }
    }
}
