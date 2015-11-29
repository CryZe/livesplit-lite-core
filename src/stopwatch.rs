extern crate chrono;

use self::chrono::{UTC, DateTime, Duration};
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Clone, Copy)]
pub struct Stopwatch {
    started: Option<DateTime<UTC>>,
}

impl Default for Stopwatch {
    fn default() -> Stopwatch {
        Stopwatch { started: None }
    }
}

impl Display for Stopwatch {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.get_time())
    }
}

impl Stopwatch {
    pub fn new() -> Stopwatch {
        Stopwatch::default()
    }

    pub fn is_started(&self) -> bool {
        self.started.is_some()
    }

    pub fn start_new() -> Stopwatch {
        let mut stopwatch = Stopwatch::new();
        stopwatch.start();
        stopwatch
    }

    pub fn start(&mut self) {
        self.started = Some(UTC::now())
    }

    pub fn stop(&mut self) {
        self.started = None
    }

    pub fn get_time(&self) -> Duration {
        match self.started {
            Some(time) => UTC::now() - time,
            None => Duration::zero(),
        }
    }
}
