use crate::{
    config::Config,
    tile::{Any, Constraint, Tile},
};
use rand::{thread_rng, Rng};

mod tile;

pub mod config;

fn set_tile<'a>(row: usize, column: usize, output: &mut Vec<Vec<&'a Tile>>, tiles: &'a [Tile]) {
    let mut requirements = Constraint {
        up: Any,
        right: Any,
        down: Any,
        left: Any,
    };

    // is there is a valid index above the current one?
    // same as "is this index NOT in the topmost row?"
    if row != 0 {
        requirements.up = output[row - 1][column].constraints.down;
    }

    // is there is a valid index to the left of the current one?
    // same as "is this index NOT in the leftmost column?"
    if column > 0  {
        requirements.left = output[row][column - 1].constraints.right;
    }

    let possibilities: &Vec<&Tile> = &tiles
        .iter()
        .filter(|t| t.constraints.equals(&requirements))
        .collect();

    let mut rng = thread_rng();
    let random_tile: &Tile = possibilities
        .get(rng.gen_range(0..possibilities.len()))
        .unwrap();
    *output.get_mut(row).unwrap().get_mut(column).unwrap() = random_tile;
}

fn display_output(output: Vec<Vec<&Tile>>, config: Config) {
    for row in 0..config.height {
        for column in 0..config.width {
            print!("{}", output[row][column])
        }
        println!("");
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

pub fn run(config: Config) {
    let tiles: Vec<Tile> = generate_tiles();
    let (width, height) = (config.width, config.height);
    let mut output: Vec<Vec<&Tile>> = vec![vec![&tiles[0]; width]; height];

    for row in 0..height {
        for column in 0..width {
            set_tile(row, column, &mut output, &tiles);
        }
    }

    display_output(output, config);
}
