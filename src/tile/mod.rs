pub use crate::tile::{
    constraint::Constraint,
    requirement::Requirement::{Any, MustBe},
};

pub mod constraint;
pub mod requirement;

#[derive(Copy, Clone, Debug)]
pub struct Tile {
    char: char,
    pub constraints: Constraint,
}

impl Tile {
    pub const fn new(char: char, up: bool, right: bool, down: bool, left: bool) -> Self {
        Self {
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

pub fn generate_tiles() -> Vec<Tile> {
    vec![
        Tile::new(' ', false, false, false, false),
        Tile::new('┏', false, true, true, false),
        Tile::new('┓', false, false, true, true),
        Tile::new('┗', true, true, false, false),
        Tile::new('┛', true, false, false, true),
        Tile::new('━', false, true, false, true),
        Tile::new('┃', true, false, true, false),
        Tile::new('┣', true, true, true, false),
        Tile::new('┫', true, false, true, true),
        Tile::new('┳', false, true, true, true),
        Tile::new('┻', true, true, false, true),
        Tile::new('╋', true, true, true, true),
    ]
}

