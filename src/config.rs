pub struct Config {
    pub width: usize,
    pub height: usize,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        args.next(); // eat program name argument

        let width = match args.next() {
            None => return Err("Could not find width, height arguments"),
            Some(x) => match x.parse() {
                Ok(num) => num,
                Err(_) => return Err("provided width is not a valid unsigned integer"),
            },
        };

        let height = match args.next() {
            None => return Err("Could not find width, height arguments"),
            Some(x) => match x.parse() {
                Ok(num) => num,
                Err(_) => return Err("provided height is not a valid unsigned integer"),
            },
        };

        Ok(Self { width, height })
    }
}
