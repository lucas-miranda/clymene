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

const WAIT_TIME_LIMIT: u64 = 3000;

pub fn wait_until<F: Fn() -> bool>(wait_condition: F) {
    let mut elapsed_time = 0;
    while !wait_condition() {
        std::thread::sleep(std::time::Duration::from_millis(10u64));
        elapsed_time += 10;

        if elapsed_time >= WAIT_TIME_LIMIT {
            panic!(
                "Conditional wait has reached it's time limit ({:.2}s)",
                (WAIT_TIME_LIMIT as f64) / 1000.0f64
            );
        }
    }
}
