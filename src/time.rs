use chrono::Duration;

pub trait Time {
    fn total_seconds(&self) -> f32;
}

impl Time for Duration {
	fn total_seconds(&self) -> f32 {
		self.num_milliseconds() as f32 / 1000.0
	}
}