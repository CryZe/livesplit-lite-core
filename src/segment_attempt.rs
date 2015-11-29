extern crate chrono;

use segment::Segment;
use self::chrono::Duration;

pub struct SegmentAttempt<'segment> {
    pub segment: &'segment Segment,
    pub time: Option<Duration>,
}

impl<'segment> SegmentAttempt<'segment> {
    pub fn new(segment: &'segment Segment) -> SegmentAttempt {
        SegmentAttempt {
            segment: segment,
            time: None,
        }
    }
}
