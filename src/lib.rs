use crate::tile::{Tile, Constraint, Any};
use rand::{thread_rng, Rng};

mod tile;

pub struct Config {
    pub width: usize,
    pub height: usize,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); // eat program name argument

        let width = match args.next() {
            None => return Err("Could not find width, height arguments"),
            Some(x) => match x.parse() {
                Ok(num) => num,
                Err(_) => return Err("Could not parse width"),
            },
        };

        let height = match args.next() {
            None => return Err("Could not find width, height arguments"),
            Some(x) => match x.parse() {
                Ok(num) => num,
                Err(_) => return Err("Could not parse height"),
            },
        };

        Ok(Config { width, height })
    }
}

fn set_tile<'a>(index: usize, output: &mut [&'a Tile], tiles: &'a [Tile], config: &Config) {
    let width = config.width;

    let mut requirements = Constraint {
        up: Any,
        right: Any,
        down: Any,
        left: Any,
    };

    // is there is a valid index above the current one?
    // same as "is this index NOT in the topmost row?"
    if index >= width {
        requirements.up = output[index - width].constraints.down;
    }

    // is there is a valid index to the left of the current one?
    // same as "is this index NOT in the leftmost column?"
    if index % width != 0 {
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

fn display_output(output: Vec<&Tile>, config: Config) {
    for (i, tile) in output.iter().enumerate() {
        print!("{tile}");
        if (i + 1) % config.width == 0 {
            println!()
        }
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
    let num_elements = config.width * config.height;
    let mut output: Vec<&Tile> = vec![&tiles[0]; num_elements];

    for i in 0..num_elements {
        set_tile(i, &mut output, &tiles, &config);
    }

    display_output(output, config);
}
