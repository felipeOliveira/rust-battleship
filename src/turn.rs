use crate::player::Player;

pub struct Turn<'a> {
    round: usize,
    player: Option<&'a Player>,
}

impl<'a> Turn<'a> {
    pub fn new() -> Turn<'a> {
        Turn {
            round: 0,
            player: None,
        }
    }

    pub fn player(&self) -> Option<&'a Player> {
        self.player
    }

    pub fn round(&self) -> &usize {
        &self.round
    }

    pub fn change(&mut self, player: &'a Player) {
        self.player = Some(player)
    }
}
