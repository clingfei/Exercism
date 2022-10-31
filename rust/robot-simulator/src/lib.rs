// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

use Direction::*;

pub struct Robot {
    x: i32,
    y: i32,
    d: Direction
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot{
            x: x, y: y, d: d
        }
    }

    #[must_use]
    pub fn turn_right(mut self) -> Self {
        self.d = match self.d {
            North => East,
            East => South,
            South => West,
            West => North
        };
        self
    }

    #[must_use]
    pub fn turn_left(mut self) -> Self {
        self.d = match self.d {
            North => West,
            West => South,
            South => East,
            East => North
        };
        self
    }

    #[must_use]
    pub fn advance(mut self) -> Self {
        match self.d {
            North=> self.y += 1,
            East => self.x += 1,
            South => self.y -= 1,
            West => self.x -= 1
        };
        self
    }

    #[must_use]
    pub fn instructions(mut self, instructions: &str) -> Self {
        for i in instructions.chars() {
            self = match i {
                'A' => self.advance(),
                'L' => self.turn_left(),
                'R' => self.turn_right(),
                _ => self
            };
        }
        self
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}
