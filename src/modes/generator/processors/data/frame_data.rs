use std::convert::TryInto;

use serde::{Deserialize, Serialize};

use crate::{
    math::Rectangle, modes::generator::processors::image::format_handlers::aseprite_handler,
};

#[derive(Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum FrameData {
    Empty,
    Contents {
        #[serde(rename = "atlas")]
        atlas_region: Rectangle<u32>,

        #[serde(skip_serializing_if = "Option::is_none")]
        duration: Option<u32>,

        #[serde(rename = "source")]
        source_region: Rectangle<u32>,
    },
}

impl From<&aseprite_handler::data::FrameData> for FrameData {
    fn from(aseprite_frame_data: &aseprite_handler::data::FrameData) -> Self {
        if aseprite_frame_data.sprite_source_size.w == 0
            || aseprite_frame_data.sprite_source_size.h == 0
        {
            return Self::Empty;
        }

        Self::Contents {
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
            .unwrap_or_default(),
        }
    }
}

impl From<aseprite_handler::data::FrameData> for FrameData {
    fn from(aseprite_frame_data: aseprite_handler::data::FrameData) -> Self {
        FrameData::from(&aseprite_frame_data)
    }
}
