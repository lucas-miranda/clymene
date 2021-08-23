use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum FrameIndicesData {
    Value(u32),
    Range { from: u32, to: u32 },
}
