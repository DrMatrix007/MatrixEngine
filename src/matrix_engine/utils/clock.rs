use std::time::{Instant, Duration};

#[derive(Copy,Clone)]
pub struct Clock {
    start_time: Instant
}

impl Clock {
    pub fn start_new() -> Self {
        Clock { start_time: Instant::now() }
    }
    pub fn elapsed(&self) -> Duration {
        Instant::now() - self.start_time
    }
    pub fn restart(&mut self) -> Duration {
        let ans = self.elapsed();
        self.start_time = Instant::now();
        ans
    }
}