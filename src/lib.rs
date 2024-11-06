use self::tile::{generate_tiles, Any, Constraint, Tile};
use rand::{thread_rng, Rng};

mod tile;
pub mod config;

fn set_tile<'a>(row: usize, column: usize, output: &mut [Vec<&'a Tile>], tiles: &'a [Tile]) {
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
    if column > 0 {
        requirements.left = output[row][column - 1].constraints.right;
    }

    let possible_tiles: &Vec<&Tile> = &tiles
        .iter()
        .filter(|t| t.constraints.equals(requirements))
        .collect();

    let mut rng = thread_rng();

    unsafe {
        let chosen_tile: &Tile =
            possible_tiles.get_unchecked(rng.gen_range(0..possible_tiles.len()));
        *output.get_unchecked_mut(row).get_unchecked_mut(column) = chosen_tile;
    }
}

#[must_use]
pub fn generate_art(width: usize, height: usize) -> String {
    let tiles: Vec<Tile> = generate_tiles();
    let (width, height) = (width, height);
    let mut output: Vec<Vec<&Tile>> = vec![vec![&tiles[0]; width]; height];

    for row in 0..height {
        for column in 0..width {
            set_tile(row, column, &mut output, &tiles);
        }
    }

    // Tiles have been generated.
    // Return them in string format.

    let mut s = String::new();

    for (idx, row) in output.iter().enumerate() {
        for tile in row.iter().take(width) {
            s.push_str(&tile.to_string());
        }

        // Do not terminate with '\n'. I think delegating that to the
        // calling code is ok.
        if idx == output.len() - 1 {
            break;
        }

        s.push('\n');
    }

    s
}
