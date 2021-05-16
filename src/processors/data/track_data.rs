use serde::{
    Deserialize,
    Serialize
};

use super::FrameIndicesData;

#[derive(Serialize, Deserialize, Clone)]
pub struct TrackData {
    pub label: Option<String>,
    pub indices: Vec<FrameIndicesData>
}
