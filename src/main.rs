use rand::{Rng, thread_rng};

const SIDE_LENGTH: usize = 3;

// "do lines continue in [direction]?"
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

fn get_index_on_side(side: usize, index: usize) -> Option<usize> {
    match side {
        // get index above
        0 => if index < SIDE_LENGTH { None } else { Some(index - SIDE_LENGTH) },
        // get index leftward
        3 => if index % SIDE_LENGTH == 0 { None } else { Some(index - SIDE_LENGTH) },
        _ => panic!("Trippin"),
    }
}

fn space_on_side_of_index_is_inside(side: usize, index: usize) -> bool {
    match side {
        // get index above
        0 => if index < SIDE_LENGTH { false } else { true },
        // get index rightward
        1 => if index - 1 % SIDE_LENGTH == 0 { false } else { true },
        // get index downward
        2 => if index >= SIDE_LENGTH * (SIDE_LENGTH - 1) { false } else { true },
        // get index leftward
        3 => if index % SIDE_LENGTH == 0 { false } else { true },
        _ => panic!("Trippin"),
    }
}

fn get_random_tile(tiles: &Vec<Tile>) -> &Tile {
    let mut rng = thread_rng();
    let possibilities: &Vec<&Tile> = &tiles.iter().filter(|t| !t.constraints.left && !t.constraints.up).collect();
    *possibilities.get(rng.gen_range(0..possibilities.len())).unwrap()
}

fn set_tile<'a>(index: usize, output: &mut Vec<&'a Tile>, tiles: &'a Vec<Tile>) {
    // generate possibilities according to neighboring tiles,
    // no neighbor => use function to determine space type
    // then push the tile
    let now_index = output.len();
    let side_is_inside = |side: usize| -> bool {
        space_on_side_of_index_is_inside(side, now_index)
    };
    let is_set = |side: usize| -> bool {
        space_on_side_of_index_is_inside(side, now_index)
        && output.get(get_index_on_side(side, now_index).unwrap()).unwrap().char != ' '
    };
    let possibilities: &Vec<&Tile> = &tiles.iter().filter(|t|
        ((!side_is_inside(0) && t.constraints.up == false) || side_is_inside(0) && t.constraints.up == output[get_index_on_side(0, now_index).unwrap()].constraints.down)
        // && (side_is_inside(1) || t.constraints.right == false)
        // && (side_is_inside(2) || t.constraints.down == false)
        && ((!side_is_inside(3) && t.constraints.left == false) || side_is_inside(3) && t.constraints.left == output[get_index_on_side(3, now_index).unwrap()].constraints.right) 
    )
        .collect();
    let mut rng = thread_rng();
    let tile_ref = *possibilities.get(rng.gen_range(0..possibilities.len())).unwrap();
    *output.get_mut(index).unwrap() = tile_ref;
}

fn main() {
    let tiles: Vec<Tile> = vec![
        Tile::new(' ', false, false, false, false),
        Tile::new('╭', false, true, true, false),
        Tile::new('╮', false, false, true, true),
        Tile::new('╰', true, true, false, false),
        Tile::new('╯', true, false, false, true),
        Tile::new('─', false, true, false, true),
        Tile::new('│', true, false, true, false),
        Tile::new('├', true, true, true, false),
        Tile::new('┤', true, false, true, true),
        Tile::new('┬', false, true, true, true),
        Tile::new('┼', true, true, true, true),
        Tile::new('┴', true, true, false, true),
    ];

    let num_elements = SIDE_LENGTH * SIDE_LENGTH;

    let mut output: Vec<&Tile> = vec![&tiles[0]; num_elements];

    for i in 0..num_elements {
        set_tile(i, &mut output, &tiles);
    }

    for (i, tile) in output.iter().enumerate() {
        print!("{tile}");
        if (i + 1) % SIDE_LENGTH == 0 { print!("\n") }
    }

}
