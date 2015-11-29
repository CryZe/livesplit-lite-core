use splits::Splits;
use attempt::Attempt;
use stopwatch::Stopwatch;
use segment_attempt::SegmentAttempt;
use chrono::Duration;

#[derive(Eq, PartialEq)]
pub enum AttemptState {
    NotRunning,
    Running,
    Ended,
}

use self::AttemptState::*;

pub struct ActiveAttempt<'splits> {
    state: AttemptState,
    split_index: usize,
    attempt: Attempt<'splits>,
    stopwatch: Stopwatch,
}

impl<'splits> ActiveAttempt<'splits> {
    pub fn new(splits: &Splits, id: usize) -> ActiveAttempt {
        ActiveAttempt {
            state: NotRunning,
            split_index: 0,
            attempt: Attempt::new(splits, id),
            stopwatch: Stopwatch::new(),
        }
    }

    pub fn is_running(&self) -> bool {
        self.state == Running
    }

    pub fn get_current_segment(&self) -> Option<&SegmentAttempt> {
        if self.is_running() {
            Some(&self.attempt.segments[self.split_index])
        } else {
            None
        }
    }

    pub fn start(&mut self) {
        self.stopwatch.start();
        self.state = Running;
    }

    fn move_to_next_segment(&mut self, time: Option<Duration>) -> Option<&SegmentAttempt> {
        self.attempt.segments[self.split_index].time = time;
        let result = Some(&self.attempt.segments[self.split_index]);
        if self.split_index == self.attempt.segments.len() - 1 {
            self.state = Ended;
        } else {
            self.split_index += 1;
        }
        result
    }

    pub fn split(&mut self) -> Option<&SegmentAttempt> {
        if self.is_running() {
            let time = Some(self.stopwatch.get_time());
            self.move_to_next_segment(time)
        } else {
            None
        }
    }

    pub fn skip(&mut self) -> Option<&SegmentAttempt> {
        if self.is_running() && self.split_index < self.attempt.segments.len() - 1 {
            self.move_to_next_segment(None)
        } else {
            None
        }
    }

    pub fn undo(&mut self) -> Option<&SegmentAttempt> {
        if self.split_index > 0 {
            if self.state == Ended {
                self.state = Running;
            } else {
                self.split_index -= 1;
            }
            self.attempt.segments[self.split_index].time = None;
            Some(&self.attempt.segments[self.split_index])
        } else {
            None
        }
    }

    pub fn reset(self) -> Attempt<'splits> {
        self.attempt
    }
}
