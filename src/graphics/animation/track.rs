#[derive(Debug)]
pub struct Track {
    pub label: Option<String>,
    pub frame_indices: Vec<u32>,
}

impl Track {
    pub fn new(label: Option<String>) -> Self {
        Self {
            label,
            frame_indices: Vec::new(),
        }
    }
}
