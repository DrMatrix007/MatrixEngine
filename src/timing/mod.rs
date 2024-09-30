use std::time::Instant;

pub struct Stopwatch {
    name: String,
    now: Instant,
}

impl Stopwatch {
    pub fn new(name: impl AsRef<str>) -> Self {
        Self {
            name: name.as_ref().to_string(),
            now: Instant::now(),
        }
    }

    pub fn debug_elapesd(&mut self) {
        let now = Instant::now();

        println!(
            "stopwatch: {:20} measerd: {:10.10}",
            self.name,
            (now - self.now).as_secs_f64()
        );
        self.now = Instant::now();
    }
}
