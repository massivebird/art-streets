use art_boxes::{Tile, generate_tiles, display_output, set_tile};

const HEIGHT: usize = 4;
const WIDTH:  usize = 8;

fn main() {
    let tiles: Vec<Tile> = generate_tiles();

    let num_elements = HEIGHT * WIDTH;

    let mut output: Vec<&Tile> = vec![&tiles[0]; num_elements];

    for i in 0..num_elements {
        set_tile(i, &mut output, &tiles);
    }

    display_output(output);
}
