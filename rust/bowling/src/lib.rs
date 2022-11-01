#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    current: u16,

}

impl BowlingGame {
    pub fn new() -> Self {
        unimplemented!();
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        self.current = pins;
    }

    pub fn score(&self) -> Option<u16> {
        unimplemented!("Return the score if the game is complete, or None if not.");
    }
}
