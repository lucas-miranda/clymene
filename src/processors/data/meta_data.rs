use serde::{
    Deserialize,
    Serialize
};

#[derive(Serialize, Deserialize)]
pub struct MetaData {
    pub app: String,
    pub version: String,
}

impl MetaData {
    pub fn new() -> Self {
        Self {
            app: String::from("https://github.com/lucas-miranda/raven"),
            version: option_env!("CARGO_PKG_VERSION").unwrap_or("unknown").to_owned()
        }
    }
}
