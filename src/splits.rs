use segment::Segment;
use game::Game;
use category::Category;
use std::rc::Rc;

pub struct Splits {
    pub game: Game,
    pub category: Category,
    pub segments: Vec<Rc<Segment>>,
}

impl Splits {
    pub fn new_empty(game: Game, category: Category) -> Splits {
        Self::new(game, category, vec![])
    }

    pub fn new(game: Game, category: Category, segments: Vec<Segment>) -> Splits {
        Splits {
            game: game,
            category: category,
            segments: segments.into_iter().map(|x| Rc::new(x)).collect(),
        }
    }

    pub fn add_segment(&mut self, segment: Segment) {
        self.segments.push(Rc::new(segment));
    }
}
