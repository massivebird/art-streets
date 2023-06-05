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
                Any => true,
                MustBe(y) => x == y,
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
    constraints: Constraint,
}

impl Tile {
    fn new(char: char, up: bool, right: bool, down: bool, left: bool) -> Tile {
        Tile {
            char,
            constraints: Constraint {
                up:    MustBe(up),
                right: MustBe(right),
                down:  MustBe(down),
                left:  MustBe(left),
            }
        }
    }

}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.char)
    }
}

fn set_tile<'a>(index: usize, output: &mut [&'a Tile], tiles: &'a [Tile]) {
    let mut requirements = Constraint {
        up:    Any,
        right: Any,
        down:  Any,
        left:  Any,
    };

    // is there is a valid index above the current one?
    // same as "is this index NOT in the topmost row?"
    if index >= WIDTH {
        requirements.up = output[index - WIDTH].constraints.down;
    }

    // is there is a valid index to the left of the current one?
    // same as "is this index NOT in the leftmost column?"
    if index % WIDTH != 0 {
        requirements.left = output[index - 1].constraints.right;
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
