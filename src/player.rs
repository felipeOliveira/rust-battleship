use std::fmt::Display;

#[derive(Debug, Eq, PartialEq)]
pub struct Player(String);

impl Player {
    pub fn new(name: String) -> Player {
        Player(name)
    }
    
    pub fn name(&self) -> &str {
        &self.0
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
