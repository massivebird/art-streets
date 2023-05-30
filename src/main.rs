use rand::{Rng, thread_rng};

const SIDE_LENGTH: usize = 3;

#[derive(Copy, Clone, Debug)]
struct Constraint {
    up: bool,
    right: bool,
    down: bool,
    left: bool,
}

impl Constraint {
    fn equals(&self, other: &Constraint) -> bool {
        self.up == other.up
        && self.right == other.right
        && self.down == other.down
        && self.left == other.left
    }
}

#[derive(Copy, Clone, Debug)]
struct Tile {
    char: char,
    constraints: Constraint
}

impl Tile {
    fn new(char: char, up: bool, right: bool, down: bool, left: bool) -> Tile {
        Tile { char, constraints: Constraint { up, right, down, left } }
    }

    fn fits_on_tile_on_side(&self, other: &Tile, side: usize) -> bool {
        match side {
            // fits above
            0 => self.constraints.down == other.constraints.up,
            // fits rightward
            1 => self.constraints.left == other.constraints.right,
            // fits below
            2 => self.constraints.up == other.constraints.down,
            // fits leftward
            3 => self.constraints.right == other.constraints.left,
            _ => panic!("Bad side"),
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.char)
    }
}

struct Grid {
    tiles: [Option<Tile>; SIDE_LENGTH * SIDE_LENGTH]
}

impl Grid {
    fn new() -> Grid {
        Grid {
            tiles: [None; SIDE_LENGTH * SIDE_LENGTH],
        }
    }

    fn print(&self) {
        for i in 0..self.tiles.len() {
            match self.tiles[i] {
                Some(tile) => print!("{}", tile),
                None => print!("?"),
            }
            if (i + 1) % SIDE_LENGTH == 0 { print!("\n") };
        }
    }
}

fn get_random_tile(tiles: &Vec<Tile>) -> &Tile {
    let mut rng = thread_rng();
    let possibilities: &Vec<&Tile> = &tiles.iter().filter(|t| !t.constraints.left && !t.constraints.up).collect();
    *possibilities.get(rng.gen_range(0..possibilities.len())).unwrap()
}

fn main() {
    let tiles: Vec<Tile> = vec![
        Tile::new(' ', false, false, false, false),
        Tile::new('─', false, true, false, true),
        Tile::new('│', true, false, true, false),
        Tile::new('╭', false, true, true, false),
        Tile::new('╮', false, false, true, true),
        Tile::new('╰', true, true, false, false),
        Tile::new('╯', true, false, false, true),
        Tile::new('├', true, true, true, false),
    ];

    let mut output: Vec<&Tile> = Vec::new();

    output.push(get_random_tile(&tiles));

    for i in 0..=5 {
        // let this_tile = tiles.
    }

    for tile in output {
        println!("{tile}");
    }

}
