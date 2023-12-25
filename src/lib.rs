use self::{
    config::Config,
    tile::{Any, Constraint, Tile, generate_tiles},
};
use rand::{thread_rng, Rng};

mod tile;

pub mod config;

fn set_tile<'a>(
    row: usize,
    column: usize,
    output: &mut [Vec<&'a Tile>],
    tiles: &'a [Tile]
) {
    let mut requirements = Constraint {
        up: Any,
        right: Any,
        down: Any,
        left: Any,
    };

    // all logic is driven by this principle:
    // we create tiles from top to bottom, left to right!

    // if this is not the topmost row,
    // then set upwards requirements according to tile above
    if row != 0 {
        requirements.up = output[row - 1][column].constraints.down;
    }

    // if this is not the leftmost column,
    // then set leftwards requirements according to tile leftwards
    if column > 0  {
        requirements.left = output[row][column - 1].constraints.right;
    }

    let possible_tiles: &Vec<&Tile> = &tiles
        .iter()
        .filter(|t| t.constraints.equals(requirements))
        .collect();

    let mut rng = thread_rng();

    unsafe {
        let chosen_tile: &Tile =
        possible_tiles
            .get_unchecked(rng.gen_range(0..possible_tiles.len()));
        *output.get_unchecked_mut(row)
            .get_unchecked_mut(column) = chosen_tile;
    }   
}

fn display_output(output: &[Vec<&Tile>], config: &Config) {
    for row in output {
        for tile in row.iter().take(config.width) {
            print!("{tile}");
        }
        println!();
    }
}

pub fn run(config: &Config) {
    let tiles: Vec<Tile> = generate_tiles();
    let (width, height) = (config.width, config.height);
    let mut output: Vec<Vec<&Tile>> = vec![vec![&tiles[0]; width]; height];

    for row in 0..height {
        for column in 0..width {
            set_tile(row, column, &mut output, &tiles);
        }
    }

    display_output(&output, config);
}
