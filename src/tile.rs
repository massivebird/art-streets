bitflags::bitflags! {
    #[derive(Copy, Clone, Debug)]
    pub struct Sides: u32 {
        const UP = 0b0001;
        const DOWN = 0b0010;
        const LEFT = 0b0100;
        const RIGHT = 0b1000;
        const NONE = 0b0000;
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Tile {
    char: char,
    pub sides: Sides,
}

impl Tile {
    pub const fn new(char: char, sides: Sides) -> Self {
        Self { char, sides }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.char)
    }
}

pub fn generate_tiles() -> Vec<Tile> {
    vec![
        Tile::new(' ', Sides::NONE),
        Tile::new('┏', Sides::DOWN | Sides::RIGHT),
        Tile::new('┓', Sides::LEFT | Sides::DOWN),
        Tile::new('┗', Sides::RIGHT | Sides::UP),
        Tile::new('┛', Sides::UP | Sides::LEFT),
        Tile::new('━', Sides::LEFT | Sides::RIGHT),
        Tile::new('┃', Sides::UP | Sides::DOWN),
        Tile::new('┣', Sides::UP | Sides::RIGHT | Sides::DOWN),
        Tile::new('┫', Sides::UP | Sides::LEFT | Sides::DOWN),
        Tile::new('┳', Sides::LEFT | Sides::RIGHT | Sides::DOWN),
        Tile::new('┻', Sides::UP | Sides::RIGHT | Sides::LEFT),
        Tile::new('╋', Sides::UP | Sides::RIGHT | Sides::DOWN | Sides::LEFT),
    ]
}
