use std::env;
use std::fs::File;
use std::io::{self, BufRead};

pub mod topography;

#[derive(Debug)]
pub enum ChallengePart {
  One,
  Two
}

pub struct ChallengeConfig {
  pub is_test: bool,
  pub part: ChallengePart
}

impl ChallengeConfig {
  const TEST_INPUT_FILE_PATH: &str = "./src/example_input.txt";
  const PUZZLE_INPUT_FILE_PATH: &str = "./src/puzzle_input.txt";

  /// Parses the command arguments and returns the current's challenge runtime config.
  /// The default configuration is to run challenge's part one with the puzzle input. 
  /// For the other configurations there are the following arguments:
  ///   -2, --two: for running the second part of the challenge.
  ///   -t, --test: for using the test input.
  pub fn get() -> Self {
    let is_test = env::args().any(|arg| ["-t", "--test"].contains(&arg.as_str()));

    let part = if env::args().any(|arg| ["-2", "--two"].contains(&arg.as_str())) {
      ChallengePart::Two
    } else {
      ChallengePart::One
    };

    println!("------ Running part: {:?}; Using: {} input -------", part, if is_test { "test" } else { "puzzle" } );
  
    Self { is_test, part }
  }

  /// Returns an iterator on the input files.
  /// If no file path is specified it uses the challenge config's defaults.
  pub fn read_puzzle_input(&self, file_path: Option<&str>) -> impl Iterator<Item = String> {
    let default_file_path = if self.is_test { 
      Self::TEST_INPUT_FILE_PATH 
    } else { 
      Self::PUZZLE_INPUT_FILE_PATH 
    };
    
    let file_path = file_path.unwrap_or(default_file_path);
    
    let file: File = File::open(file_path).expect(format!("Couldn't open {file_path}").as_str());
    let reader = io::BufReader::new(file);

    reader.lines().map(|line| line.expect("Error reading a line"))
  }
}

pub const TEST_CONFIG: ChallengeConfig = ChallengeConfig {
  is_test: true,
  part: ChallengePart::One,
};