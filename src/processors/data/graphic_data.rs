use serde::{
    Deserialize,
    Serialize
};

use super::{
    FrameData,
    TrackData
};

#[derive(Serialize, Deserialize, Clone)]
pub struct GraphicData {
    pub frames: Vec<FrameData>,
    pub tracks: Vec<TrackData>
}

impl GraphicData {
    pub fn new() -> Self {
        Self {
            frames: Vec::new(),
            tracks: Vec::new()
        }
    }
}
