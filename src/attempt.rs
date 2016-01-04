use splits::Splits;
use segment_attempt::SegmentAttempt;
use std::rc::Rc;

pub struct Attempt {
    pub id: usize,
    pub splits: Rc<Splits>,
    pub segments: Vec<SegmentAttempt>,
}

impl Attempt {
    pub fn new(splits: Rc<Splits>, id: usize) -> Attempt {
        let segments = splits.segments
                             .iter()
                             .map(|segment| SegmentAttempt::new(segment.clone()))
                             .collect();
        Attempt {
            id: id,
            splits: splits,
            segments: segments,
        }
    }
}
