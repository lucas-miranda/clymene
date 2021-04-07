use serde::{ 
    Deserialize, 
    Serialize 
};

#[derive(Serialize, Deserialize)]
pub struct DataConfig {
    #[serde(default)]
    pub verbose: bool,

    #[serde(default)]
    pub prettify: bool
}

impl Default for DataConfig {
    fn default() -> DataConfig {
        DataConfig {
            verbose: false,
            prettify: false
        }
    }
}
