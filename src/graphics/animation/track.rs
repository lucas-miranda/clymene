use super::{FrameIndicesGroup, TrackList};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Track {
    pub label: Option<String>,

    #[serde(skip_serializing_if = "TrackList::is_empty", default)]
    pub tracks: TrackList,

    indices: FrameIndicesGroup,
}

impl Track {
    pub fn new(label: Option<String>, indices: FrameIndicesGroup) -> Self {
        Self {
            label,
            indices,
            tracks: TrackList::new(),
        }
    }

    /// Group of frame indices.
    pub fn indices(&self) -> &FrameIndicesGroup {
        &self.indices
    }
}
