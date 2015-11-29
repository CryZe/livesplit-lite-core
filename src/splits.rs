use segment::Segment;
use active_attempt::ActiveAttempt;

pub struct Splits {
    pub segments: Vec<Segment>,
}

impl Splits {
    pub fn do_attempt(&self, id: usize) -> ActiveAttempt {
        ActiveAttempt::new(self, id)
    }
}
