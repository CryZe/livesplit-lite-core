use std::fmt;

#[derive(Clone)]
pub struct Segment {
    pub name: String,
}

impl fmt::Display for Segment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Segment {
    pub fn new(name: String) -> Segment {
        Segment { name: name }
    }
}
