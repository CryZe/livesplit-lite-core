pub struct Game {
    pub name: String,
}

impl Game {
    pub fn new(name: String) -> Game {
        Game { name: name }
    }
}
