pub struct Config {
    pub width: usize,
    pub height: usize,
}

impl Config {
    /// # Errors
    ///
    /// Returns an error message if argument parsing fails.
    pub fn build() -> Result<Self, &'static str> {
        let mut args = std::env::args();

        args.next(); // eat program name argument

        let mut next_arg_int = || -> Result<usize, &str> {
            args.next()
                .map_or(Err("Could not find width, height arguments"), |x| {
                    x.parse::<usize>()
                        .map_or(Err("provided width is not a valid unsigned integer"), Ok)
                })
        };

        let width = next_arg_int()?;
        let height = next_arg_int()?;

        Ok(Self { width, height })
    }
}
