use rand::{Rng, thread_rng};
use Requirement::{MustBe, Any};

const HEIGHT: usize = 4;
const WIDTH:  usize = 8;

#[derive(Copy, Clone, Debug)]
enum Requirement {
    MustBe(bool),
    Any,
}

impl Requirement {
    fn equals(&self, other: &Requirement) -> bool {
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
        Tile {
            char,
            constraints: Constraint {
                up:    MustBe(up),
                right: MustBe(right),
                down:  MustBe(down),
                left:  MustBe(left)
            }
        }
    }

}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.char)
    }
}

fn get_index_on_side(side: usize, index: usize) -> usize {
    match side {
        // get index above
        0 => index - WIDTH,
        // get index leftward
        3 => index - 1,
        _ => panic!("Trippin"),
    }
}

fn is_side_of_index_inside(side: usize, index: usize) -> bool {
    match side {
        // get index above
        0 => index >= WIDTH,
        // get index rightward
        1 => !(index != 0 && (index - 1) % WIDTH == 0),
        // get index downward
        2 => index < WIDTH * (WIDTH - 1),
        // get index leftward
        3 => index % WIDTH != 0,
        _ => panic!("Trippin"),
    }
}

fn set_tile<'a>(index: usize, output: &mut [&'a Tile], tiles: &'a [Tile]) {
    let side_is_inside = |side: usize| -> bool {
        is_side_of_index_inside(side, index)

    };
    let get_tile_to_the_side = |side: usize| -> &Tile {
        output[get_index_on_side(side, index)]
    };

    let mut requirements = Constraint {
        up:    Any,
        right: Any,
        down:  Any,
        left:  Any
    };

    // up
    if side_is_inside(0) {
        requirements.up = get_tile_to_the_side(0).constraints.down;
    }

    // left
    if side_is_inside(3) {
        requirements.left = get_tile_to_the_side(3).constraints.right;
    }

    let possibilities: &Vec<&Tile> = &tiles
        .iter()
        .filter(|t| t.constraints.equals(&requirements))
        .collect();

    let mut rng = thread_rng();
    let tile_ref: &Tile = possibilities
        .get(rng.gen_range(0..possibilities.len()))
        .unwrap();
    *output.get_mut(index).unwrap() = tile_ref;
}

fn main() {
    let tiles: Vec<Tile> = vec![
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
    ];

    let num_elements = HEIGHT * WIDTH;

    let mut output: Vec<&Tile> = vec![&tiles[0]; num_elements];

    for i in 0..num_elements {
        set_tile(i, &mut output, &tiles);
    }

    for (i, tile) in output.iter().enumerate() {
        print!("{tile}");
        if (i + 1) % WIDTH == 0 { println!() }
    }
}
