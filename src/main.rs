use rand::{Rng, thread_rng};

const SIDE_LENGTH: usize = 10;

#[derive(Copy, Clone, Debug)]
enum Requirement {
    MustBe(bool),
    Any,
}

impl Requirement {
    fn equals(&self, other: &Requirement) -> bool {
        use Requirement::{Any, MustBe};
        match self {
            Any => true,
            MustBe(x) => match other {
                MustBe(y) => x == y,
                Any => true,
            }
        }
    }
}

// "do lines continue in [direction]?"
#[derive(Copy, Clone, Debug)]
struct Constraint {
    up: Requirement,
    right: Requirement,
    down: Requirement,
    left: Requirement,
}

impl Constraint {
    fn equals(&self, other: &Constraint) -> bool {
        self.up.equals(&other.up)
        && self.right.equals(&other.right)
        && self.down.equals(&other.down)
        && self.left.equals(&other.left)
    }
}

#[derive(Copy, Clone, Debug)]
struct Tile {
    char: char,
    constraints: Constraint
}

impl Tile {
    fn new(char: char, up: bool, right: bool, down: bool, left: bool) -> Tile {
        use Requirement::MustBe;
        Tile { char, constraints: Constraint { up: MustBe(up), right: MustBe(right), down: MustBe(down), left: MustBe(left) } }
    }

    fn fits_on_tile_on_side(&self, other: &Tile, side: usize) -> bool {
        match side {
            // fits above
            0 => self.constraints.down.equals(&other.constraints.up),
            // fits rightward
            1 => self.constraints.left.equals(&other.constraints.right),
            // fits below
            2 => self.constraints.up.equals(&other.constraints.down),
            // fits leftward
            3 => self.constraints.right.equals(&other.constraints.left),
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
        3 => if index % SIDE_LENGTH == 0 { None } else { Some(index - 1) },
        _ => panic!("Trippin"),
    }
}

fn is_space_on_side_of_index_inside(side: usize, index: usize) -> bool {
    match side {
        // get index above
        0 => if index < SIDE_LENGTH { false } else { true },
        // get index rightward
        1 => if index == 0 || index - 1 % SIDE_LENGTH != 0 { true } else { false },
        // get index downward
        2 => if index >= SIDE_LENGTH * (SIDE_LENGTH - 1) { false } else { true },
        // get index leftward
        3 => if index % SIDE_LENGTH == 0 { false } else { true },
        _ => panic!("Trippin"),
    }
}

// fn get_random_tile(tiles: &Vec<Tile>) -> &Tile {
//     let mut rng = thread_rng();
//     let possibilities: &Vec<&Tile> = &tiles.iter().filter(|t| !t.constraints.left && !t.constraints.up).collect();
//     *possibilities.get(rng.gen_range(0..possibilities.len())).unwrap()
// }

fn set_tile<'a>(index: usize, output: &mut Vec<&'a Tile>, tiles: &'a Vec<Tile>) {
    use Requirement::{Any, MustBe};
    // generate possibilities according to neighboring tiles,
    // no neighbor => use function to determine space type
    // then push the tile

    let side_is_inside = |side: usize| -> bool {
        is_space_on_side_of_index_inside(side, index)
    };
    let side_is_outside = |side: usize| !side_is_inside(side);

    // let tiles = tiles.iter();
    let mut reqs = Constraint { up: Any, right: Any, down: Any, left: Any};

    dbg!(index);

    // up
    if side_is_outside(0) {
        reqs.up = MustBe(false);
    } else {
        reqs.up = output[get_index_on_side(0, index).unwrap()].constraints.down;
    }

    // right
    if side_is_outside(1) {
        reqs.right = MustBe(false);
    }

    // down
    if side_is_outside(2) {
        reqs.down = MustBe(false);
    }

    // left
    if side_is_outside(3) {
        reqs.left = MustBe(false);
    } else {
        reqs.left = output[get_index_on_side(3, index).unwrap()].constraints.right;
    }

    dbg!(reqs);

    let possibilities: &Vec<&Tile> = &tiles
        .iter()
        .filter(|t| t.constraints.equals(&reqs))
        .collect();

    // dbg!(possibilities);

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
