use super::{FrameIndicesGroup, TrackList};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Track {
    pub label: Option<String>,

    #[serde(skip_serializing_if = "TrackList::is_empty")]
    pub tracks: TrackList,

    frame_indices: FrameIndicesGroup,
}

impl Track {
    pub fn new(label: Option<String>, frame_indices: FrameIndicesGroup) -> Self {
        Self {
            label,
            frame_indices,
            tracks: TrackList::new(),
        }
    }

    /// [`FrameIndicesGroup`] in this `Track`.
    pub fn frame_indices(&self) -> &FrameIndicesGroup {
        &self.frame_indices
    }
}
