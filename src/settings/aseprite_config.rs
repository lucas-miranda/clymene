use serde::{ 
    Deserialize, 
    Serialize 
};

#[derive(Serialize, Deserialize)]
pub struct AsepriteConfig {
    #[serde(default)]
    pub verbose: bool,

    #[serde(default)]
    pub bin_path: String,

    #[serde(default)]
    pub input_path: String,

    #[serde(default)]
    pub prettify_json: bool
}

impl Default for AsepriteConfig {
    fn default() -> AsepriteConfig {
        AsepriteConfig {
            verbose: false,
            bin_path: String::new(),
            input_path: String::new(),
            prettify_json: false
        }
    }
}
