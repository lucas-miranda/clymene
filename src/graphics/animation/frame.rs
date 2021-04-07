#[derive(Debug)]
pub struct Frame {
    pub source_index: u32,
    pub duration: u32
}

impl Frame {
    pub fn new(source_index: u32, duration: u32) -> Self {
        Self {
            source_index,
            duration
        }
    }
}
