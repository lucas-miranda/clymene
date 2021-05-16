use serde::{
    Deserialize,
    Serialize
};

use crate::math::Rectangle;

#[derive(Serialize, Deserialize, Clone)]
pub struct FrameData {
    pub atlas_region: Rectangle<u32>,
    pub duration: Option<u32>,
    pub source_region: Rectangle<u32>
}
