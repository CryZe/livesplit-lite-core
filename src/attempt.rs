use splits::Splits;
use segment_attempt::SegmentAttempt;

pub struct Attempt<'segments> {
    pub id: usize,
    pub segments: Vec<SegmentAttempt<'segments>>,
}

impl<'segments> Attempt<'segments> {
    pub fn new(splits: &Splits, id: usize) -> Attempt {
        Attempt {
            id: id,
            segments: splits.segments.iter().map(|segment| SegmentAttempt::new(segment)).collect(),
        }
    }
}
