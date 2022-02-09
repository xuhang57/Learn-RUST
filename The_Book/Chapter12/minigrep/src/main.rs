use std::env;
use std::fs;
use std::process:

fn main() {
    let args: Vec<String> = env::args().collect();

    // let (query, filename) = parse_config(&args);
    //let config = parse_config(&args);
    //let config = Config::new(&args);
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query)
    println!("In file {}", config.filename);

    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong");
}

Struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str {
        if args.len() < 3 {
            // panic!("not enough arguments");
            return Err("not enough arguments")
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename})
    }
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    Config { query, filename }
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];

    (query, filename)

    // println!("Searching for {}", query);
    // println!("In file {}", filename);

    // let contents = fs::read_to_string(filename)
    //     .expect("Something went wrong reading the file");

    // println!("With text:\n{}", contents);
}


// Extract Logic From Main

fn main() {
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    // run(config);
    if let Err(e) = run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}

fn run(config: Config) {
    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file");
}

use std::error::Error;

// dyn --> dynamic
// ? --> ranter than panic! on an error, ? will return the error value from the current function for the caller to handle
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    Ok(())
}

// After adding lib.rs

use minigrep::Config;

fn main() {
    // ...
    if let Err(e) = minigrep::run(config) {
        // ...
    }
}