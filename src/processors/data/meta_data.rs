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
            app: String::from("https://github.com/lucas-miranda/clymene"),
            version: env!("CARGO_PKG_VERSION").to_owned()
        }
    }
}
