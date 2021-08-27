use std::time::{Duration, Instant};

pub mod fs;

pub struct Timer {
    start_instant: Instant,
}

impl Timer {
    pub fn start() -> Self {
        Self {
            start_instant: Instant::now(),
        }
    }

    pub fn end(self) -> Duration {
        self.start_instant.elapsed()
    }

    pub fn end_secs_str(self) -> String {
        format!("{:.3}", self.end().as_secs_f64())
    }
}
