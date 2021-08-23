use std::{fs::OpenOptions, io::BufReader, path::Path};

use serde::Deserialize;

use crate::processors::image::format_handlers::FormatHandlerError;

#[derive(Deserialize)]
pub struct Data {
    pub frames: Vec<FrameData>,
    pub meta: MetaData,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetaData {
    pub app: String,
    pub version: String,
    pub format: String,
    pub size: SizeData,
    pub scale: String,
    pub frame_tags: Vec<FrameTagData>,

    #[serde(default)]
    pub slices: Vec<SliceData>,
}

#[derive(Deserialize)]
pub struct SliceData {
    pub name: String,
    pub color: String,

    #[serde(default)]
    pub data: String,

    pub keys: Vec<SliceKeyData>,
}

#[derive(Deserialize)]
pub struct SliceKeyData {
    pub frame: i16,
    pub bounds: BoundsData,

    #[serde(default)]
    pub pivot: PositionData,
}

#[derive(Deserialize)]
pub struct FrameTagData {
    pub name: String,
    pub from: i16,
    pub to: i16,
    pub direction: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FrameData {
    pub filename: String,
    pub frame: BoundsData,
    pub rotated: bool,
    pub trimmed: bool,
    pub sprite_source_size: BoundsData,
    pub source_size: SizeData,
    pub duration: i16,
}

#[derive(Deserialize)]
pub struct BoundsData {
    pub x: i16,
    pub y: i16,
    pub w: i16,
    pub h: i16,
}

#[derive(Default, Deserialize)]
pub struct PositionData {
    pub x: i16,
    pub y: i16,
}

#[derive(Deserialize)]
pub struct SizeData {
    pub w: i16,
    pub h: i16,
}

impl Data {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, FormatHandlerError> {
        let data_file = OpenOptions::new().read(true).open(path).unwrap();
        let buf_reader = BufReader::new(data_file);
        let aseprite_animation_data: Self =
            serde_json::from_reader(buf_reader).map_err(FormatHandlerError::Deserialize)?;

        Ok(aseprite_animation_data)
    }
}
