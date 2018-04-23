extern crate lintgram;

use lintgram::linter::{Issue, IssueSeverity, lint_scene_file};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    println!("Reading file: {}", config.input_file);

    if let Err(message) = run(config) {
        eprintln!("Error: {}", message);
        std::process::exit(1);
    }
}

struct Config {
    input_file: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments.");
        }
        let input_file = args[1].clone();
        Ok(Config { input_file })
    }
}

fn run(config: Config) -> Result<(), Box<Error>> {
    let mut file = File::open(config.input_file)?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;

    let issues : Vec<Issue> = lint_scene_file(&text)?;

    println!("Linter found {} issues.", issues.len());

    for issue in issues.iter() {
        let severity = match issue.severity() {
            IssueSeverity::Warning => "WARNING",
            IssueSeverity::Error => "ERROR",
        };
        println!("{} {}", severity, issue.description());
        println!("    at line {}, col {}: {}", issue.line_number(), issue.column_number(), issue.context());
    }

    Ok(())
}
