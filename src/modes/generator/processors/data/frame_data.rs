use std::convert::TryInto;

use serde::{Deserialize, Serialize};

use crate::{
    math::Rectangle, modes::generator::processors::image::format_handlers::aseprite_handler,
};

#[derive(Serialize, Deserialize, Clone)]
pub struct FrameData {
    pub atlas_region: Rectangle<u32>,
    pub duration: Option<u32>,
    pub source_region: Rectangle<u32>,
}

impl From<&aseprite_handler::data::FrameData> for FrameData {
    fn from(aseprite_frame_data: &aseprite_handler::data::FrameData) -> Self {
        Self {
            atlas_region: Rectangle::default(),
            duration: {
                match aseprite_frame_data.duration {
                    0 => None,
                    d => d.try_into().ok(),
                }
            },
            source_region: Rectangle::with(
                aseprite_frame_data.sprite_source_size.x,
                aseprite_frame_data.sprite_source_size.y,
                aseprite_frame_data.sprite_source_size.w,
                aseprite_frame_data.sprite_source_size.h,
            )
            .unwrap_or_else(Rectangle::default),
        }
    }
}

impl From<aseprite_handler::data::FrameData> for FrameData {
    fn from(aseprite_frame_data: aseprite_handler::data::FrameData) -> Self {
        FrameData::from(&aseprite_frame_data)
    }
}
