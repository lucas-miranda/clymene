use serde::{
    Deserialize,
    Serialize
};

#[derive(Serialize, Deserialize, Clone)]
pub enum FrameIndicesData {
    Value(u32),
    Range { from: u32, to: u32 }
}
