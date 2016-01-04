extern crate chrono;

use segment::Segment;
use self::chrono::Duration;
use std::rc::Rc;

pub struct SegmentAttempt {
    pub segment: Rc<Segment>,
    pub time: Option<Duration>,
}

impl SegmentAttempt {
    pub fn new(segment: Rc<Segment>) -> SegmentAttempt {
        SegmentAttempt {
            segment: segment,
            time: None,
        }
    }
}
