use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct PackerRetryConfig {
    #[serde(default)]
    pub enable: bool,

    #[serde(default)]
    pub max_retries: u32,

    #[serde(default)]
    pub until_atlas_size: u32,
}
