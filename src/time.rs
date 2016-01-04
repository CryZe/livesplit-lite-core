use chrono::Duration;

pub trait Time {
    fn total_seconds(&self) -> f32;
    fn format(&self) -> String;
}

impl Time for Duration {
    fn total_seconds(&self) -> f32 {
        self.num_milliseconds() as f32 / 1000.0
    }

    fn format(&self) -> String {
        let total_seconds = self.total_seconds();
        let seconds = total_seconds % 60f32;
        let seconds_right = seconds % 10f32;
        let seconds_left = seconds as usize / 10;
        let total_minutes = total_seconds as usize / 60;
        let minutes = total_minutes % 60;
        let total_hours = total_minutes / 60;
        if total_hours > 0 {
            format!("{}:{:02}:{}{:.2}",
                    total_hours,
                    minutes,
                    seconds_left,
                    seconds_right)
        } else if total_minutes > 0 {
            format!("{}:{}{:.2}", total_minutes, seconds_left, seconds_right)
        } else {
            format!("{:.2}", seconds)
        }
    }
}
