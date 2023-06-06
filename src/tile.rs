use crate::tile::constraint::{requirement::Requirement::MustBe, Constraint};

pub mod constraint;

#[derive(Copy, Clone, Debug)]
pub struct Tile {
    char: char,
    pub constraints: Constraint,
}

impl Tile {
    pub fn new(char: char, up: bool, right: bool, down: bool, left: bool) -> Tile {
        Tile {
            char,
            constraints: Constraint {
                up: MustBe(up),
                right: MustBe(right),
                down: MustBe(down),
                left: MustBe(left),
            },
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.char)
    }
}
