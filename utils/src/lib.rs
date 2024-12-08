use std::env;
use std::fs::File;
use std::io::{self, BufRead};


pub fn read_puzzle_input(file_path: &str) -> impl Iterator<Item = String> {
    let file: File = File::open(file_path).expect("Couldn't open specified file");
    let reader = io::BufReader::new(file);

    reader.lines().map(|line| line.expect("Error reading a line"))
}

pub enum ChallengePart {
  One,
  Two
}

/*
  get_challenge_part:

  It parses the command line argument to identify which solution of the aoc2024 day challenge should be run.
  
  defaultst to "one" if no argument is provided
 */
pub fn get_challenge_part() -> ChallengePart {
  // get the second command-line argument (first is the program name)
  let cmd_input = env::args().nth(1).unwrap_or("one".to_string());

  // Match input to the corresponding enum variant
  match cmd_input.as_str() {
    "one" => ChallengePart::One,
    "two" => ChallengePart::Two,
    _ => panic!("Invalid option: only `one` and `two` are supported."),
  }
}
