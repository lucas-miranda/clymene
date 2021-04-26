use serde::{ 
    Deserialize, 
    Serialize 
};

#[derive(Serialize, Deserialize)]
pub struct PackerConfig {
    #[serde(default)]
    pub verbose: bool,

    #[serde(default)]
    pub atlas_size: u32,

    #[serde(default)]
    pub optimize: bool,

    #[serde(default)]
    pub force: bool
}

impl Default for PackerConfig {
    fn default() -> PackerConfig {
        PackerConfig {
            verbose: false,
            atlas_size: 0,
            optimize: true,
            force: false
        }
    }
}

impl PackerConfig {
}
