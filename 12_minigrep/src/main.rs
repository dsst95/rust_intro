use std::{env, process};

use minigrep::*;

fn main() {
  let args: Vec<String> = env::args().collect();

  let config = Config::build(&args).unwrap_or_else(|error| {
    eprintln!("Problem parsing arguments: {error}");
    process::exit(1);
  });

  println!("Searching for {}", config.query);
  println!("In file {}", config.file_path);

  if let Err(e) = run(config) {
    eprintln!("Application error: {e}");
    process::exit(1);
  }
}
