use rand::{Rng, thread_rng};
use crate::tile::{Tile, constraint::{Constraint, requirement::Requirement::Any}};

pub mod tile;

const WIDTH:  usize = 8;

pub fn set_tile<'a>(index: usize, output: &mut [&'a Tile], tiles: &'a [Tile]) {
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

pub fn display_output(output: Vec<&Tile>) {
    for (i, tile) in output.iter().enumerate() {
        print!("{tile}");
        if (i + 1) % WIDTH == 0 { println!() }
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
