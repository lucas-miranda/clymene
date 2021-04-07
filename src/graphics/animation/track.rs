use super::Frame;

#[derive(Debug)]
pub struct Track {
    label: Option<String>,
    frames: Vec<Frame>
}

impl Track {
    pub fn new(label: Option<String>) -> Self {
        Self {
            label,
            frames: Vec::new()
        }
    }

    pub fn push(&mut self, frame: Frame) {
        self.frames.push(frame);
    }
}
