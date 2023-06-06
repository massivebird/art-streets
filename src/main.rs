use std::env;
use art_boxes::Config;
use art_boxes::{tile::Tile, generate_tiles, set_tile, display_output};

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Error parsing arguments: {err}");
        std::process::exit(1)
    });

    let tiles: Vec<Tile> = generate_tiles();

    let num_elements = config.width * config.height;

    let mut output: Vec<&Tile> = vec![&tiles[0]; num_elements];

    for i in 0..num_elements {
        set_tile(i, &mut output, &tiles, &config);
    }

    display_output(output, config);
}
