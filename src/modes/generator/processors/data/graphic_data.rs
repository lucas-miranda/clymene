use serde::{Deserialize, Serialize};

use crate::graphics::animation::TrackList;

use super::FrameData;

#[derive(Serialize, Deserialize, Clone)]
pub struct GraphicData {
    pub frames: Vec<FrameData>,
    pub tracks: TrackList,
}

impl GraphicData {
    pub fn new() -> Self {
        Self {
            frames: Vec::new(),
            tracks: TrackList::new(),
        }
    }
}
