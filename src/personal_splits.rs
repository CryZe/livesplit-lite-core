use splits::Splits;
use attempt::Attempt;
use active_attempt::ActiveAttempt;
use std::rc::Rc;

pub struct PersonalSplits {
    pub splits: Rc<Splits>,
    pub attempts: Vec<Rc<Attempt>>,
    pub personal_best: Option<Rc<Attempt>>,
}

impl PersonalSplits {
    pub fn new(splits: Splits) -> PersonalSplits {
        PersonalSplits {
            splits: Rc::new(splits),
            attempts: vec![],
            personal_best: None,
        }
    }

    pub fn add_finished_attempt(&mut self, attempt: Attempt) {
        self.attempts.push(Rc::new(attempt));
        let last_attempt = self.attempts.iter().last().unwrap();

        let last_time = last_attempt.segments.iter().last().unwrap().time;
        let pb_time = self.personal_best
                          .as_ref()
                          .and_then(|pb| pb.segments.iter().last().unwrap().time);

        let is_faster = last_time.and_then(|last| pb_time.map(|pb| last < pb)).unwrap_or(false);

        if is_faster {
            self.personal_best = Some(last_attempt.clone());
        }
    }

    pub fn do_attempt(&self, id: usize) -> ActiveAttempt {
        ActiveAttempt::new(self.splits.clone(), id)
    }
}
