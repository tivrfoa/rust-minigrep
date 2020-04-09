use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

/**
* Box<dyn Error> means the function will return a type
* that implements the Error trait
*/
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // '?' will return the error value from the current
    // function for the caller to handle
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    Ok(())
}
