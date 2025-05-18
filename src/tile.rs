bitflags::bitflags! {
    #[derive(Copy, Clone, Debug)]
    pub struct Sides: u32 {
        const UP    = 0b0001;
        const DOWN  = 0b0010;
        const LEFT  = 0b0100;
        const RIGHT = 0b1000;
        const NONE  = 0b0000;
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
    // Import these constants for readability.
    const UP: Sides = Sides::UP;
    const DOWN: Sides = Sides::DOWN;
    const LEFT: Sides = Sides::LEFT;
    const RIGHT: Sides = Sides::RIGHT;
    const NONE: Sides = Sides::NONE;

    vec![
        Tile::new(' ', NONE),
        Tile::new('┏', DOWN | RIGHT),
        Tile::new('┓', LEFT | DOWN),
        Tile::new('┗', RIGHT | UP),
        Tile::new('┛', UP | LEFT),
        Tile::new('━', LEFT | RIGHT),
        Tile::new('┃', UP | DOWN),
        Tile::new('┣', UP | RIGHT | DOWN),
        Tile::new('┫', UP | LEFT | DOWN),
        Tile::new('┳', LEFT | RIGHT | DOWN),
        Tile::new('┻', UP | RIGHT | LEFT),
        Tile::new('╋', UP | RIGHT | DOWN | LEFT),
    ]
}
