use std::env;
use std::fs::File;
use std::io::{self, BufRead};

pub fn read_puzzle_input(file_path: &str) -> impl Iterator<Item = String> {
    let file: File = File::open(file_path).expect("Couldn't open specified file");
    let reader = io::BufReader::new(file);

    reader.lines().map(|line| line.expect("Error reading a line"))
}

#[derive(Debug)]
pub enum ChallengePart {
  One,
  Two
}


pub struct ChallengeConfig {
  pub is_test: bool,
  pub part: ChallengePart
}

/*
  get_challenge_part:

  It parses the command line argument to identify which solution of the aoc2024 day challenge should be run.
  
  defaultst to "one" if no argument is provided
 */
pub fn get_challenge_config() -> ChallengeConfig {
  // get the second command-line argument (first is the program name)
  let first_argument = env::args().nth(1).unwrap_or("one".to_string());

  let mut challenge_part: ChallengePart = ChallengePart::One; 
  let mut test = false;

  // Match input to the corresponding enum variant
  match first_argument.as_str() {
    "two" => { 
      challenge_part = ChallengePart::Two;
      let second_argument = env::args().nth(2).unwrap_or("".to_string());
      match second_argument.as_str() {
        "-t" | "--test" => { test = true },
        _ => (),
      }
    },
    "-t" | "--test" => { 
      test = true 
    },
    "one" => {  
      let second_argument = env::args().nth(2).unwrap_or("".to_string());
      match second_argument.as_str() {
        "-t" | "--test" => { test = true },
        _ => (),
      }
    },
    _ => (),
  }

  println!("------ Running part: {:?}; Using: {} input -------", challenge_part, if test { "test" } else { "puzzle" } );

  ChallengeConfig {
    is_test: test,
    part: challenge_part
  }
}
