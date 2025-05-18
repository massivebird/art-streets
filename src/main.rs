use self::{
    config::Config,
    tile::{generate_tiles, Sides, Tile},
};
use rand::{rngs::ThreadRng, thread_rng, Rng};
use std::io::Write;

mod config;
mod tile;

fn main() -> std::io::Result<()> {
    let config = Config::build().unwrap_or_else(|err| {
        eprintln!("Error parsing arguments: {err}");
        std::process::exit(1)
    });

    let tiles: Vec<Tile> = generate_tiles();
    let (width, height) = (config.width, config.height);
    let mut output: Vec<Vec<&Tile>> = vec![vec![&tiles[0]; width]; height];

    let mut rng = thread_rng();

    for row in 0..height {
        for column in 0..width {
            set_tile(row, column, &mut output, &tiles, &mut rng);
        }
    }

    let mut buf: Vec<u8> = Vec::new();

    for (idx, row) in output.iter().enumerate() {
        for tile in row.iter().take(width) {
            write!(&mut buf, "{}", &tile.to_string())?;
        }

        // Do not terminate with '\n'. I think delegating that to the
        // calling code is ok.
        if idx == output.len() - 1 {
            break;
        }

        writeln!(&mut buf)?;
    }

    print!("{}", String::from_utf8(buf).unwrap());

    Ok(())
}

fn set_tile<'a>(
    row: usize,
    column: usize,
    output: &mut [Vec<&'a Tile>],
    tiles: &'a [Tile],
    rng: &mut ThreadRng,
) {
    // Candidate tiles must at least contain ALL of these sides.
    let mut reqs: tile::Sides = Sides::NONE;

    // Candidate tiles must not contain ANY of these sides.
    let mut evil_sides: tile::Sides = Sides::NONE;

    // This function is called for each location, right to left, top to bottom.

    // If this is not the topmost row,
    // then refer to tile above for upwards req.
    if row != 0 {
        let tile_above = output[row - 1][column];

        if tile_above.sides.contains(Sides::DOWN) {
            reqs = Sides::UP;
        } else if (row, column) != (0, 0) {
            evil_sides = Sides::UP;
        }
    }

    // If this is not the leftmost column,
    // then refer to tile leftwards for left-facing req.
    if column > 0 {
        let tile_leftwards = output[row][column - 1];

        if tile_leftwards.sides.contains(Sides::RIGHT) {
            reqs |= Sides::LEFT;
        } else if (row, column) != (0, 0) {
            evil_sides |= Sides::LEFT;
        }
    }

    let possible_tiles: Vec<&Tile> = tiles
        .iter()
        .filter(|t| {
            let has_all_reqs = t.sides.contains(reqs);
            let no_evil_sides = evil_sides.is_empty() || !t.sides.intersects(evil_sides);

            has_all_reqs && no_evil_sides
        })
        .collect();

    unsafe {
        let chosen_tile: &Tile =
            possible_tiles.get_unchecked(rng.gen_range(0..possible_tiles.len()));

        *output.get_unchecked_mut(row).get_unchecked_mut(column) = chosen_tile;
    }
}
