use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;

pub type TopographicMap<T> = HashMap<Coordinate, T>;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Coordinate {
  pub x: i32,
  pub y: i32,
}

impl Coordinate {
  pub fn new(x: i32, y: i32) -> Self {
    Self { x, y }
  }

  pub fn is_outside_boundaries(&self, max: (i32, i32)) -> bool {
    (self.x < 0 || self.y < 0) || (self.x >= max.0 || self.y >= max.1 )
  }

  pub fn add_delta(&self, direction: &Direction) -> Coordinate {
    match direction {
      Direction::Up =>        Coordinate { x: self.x - 1, y: self.y },
      Direction::Down =>      Coordinate { x: self.x + 1, y: self.y },
      Direction::Right =>     Coordinate { x: self.x,     y: self.y + 1 },
      Direction::Left =>      Coordinate { x: self.x,     y: self.y - 1 },
      Direction::UpRight =>   Coordinate { x: self.x - 1, y: self.y + 1},
      Direction::UpLeft =>    Coordinate { x: self.x - 1, y: self.y - 1},
      Direction::DownRight => Coordinate { x: self.x + 1, y: self.y + 1},
      Direction::DownLeft =>  Coordinate { x: self.x + 1, y: self.y - 1},
    }
  }

  pub fn is_adjacent(&self, coordinate: &Coordinate) -> bool {
    for direction in Direction::to_vec() {
      let next = coordinate.add_delta(&direction);

      if next == *self {
        return true; 
      }
    }

    false
  }

  pub fn is_diagonal(&self, coordinate: &Coordinate) -> bool {
    for direction in Direction::get_diagonals() {
      let next = coordinate.add_delta(&direction);

      if next == *self {
        return true;
      }
    }

    false
  }

  pub fn get_direction(&self, coordinate: &Coordinate) -> Option<Direction> {
    for direction in Direction::get_diagonals() {
      let next = coordinate.add_delta(&direction);

      if next == *self {
        return Option::from(direction);
      }
    }

    Option::None
  }

  pub fn is_outer_edge(&self, direction: &Direction, surrounding_coordinates: &Vec<Coordinate>) -> bool {
    match direction {
      Direction::DownLeft => {
        !surrounding_coordinates.contains(&self.add_delta(&Direction::Left)) && 
        !surrounding_coordinates.contains(&self.add_delta(&Direction::Down))
      },
      Direction::DownRight => {
        !surrounding_coordinates.contains(&self.add_delta(&Direction::Down)) && 
        !surrounding_coordinates.contains(&self.add_delta(&Direction::Right))
      },
      Direction::UpRight => {
        !surrounding_coordinates.contains(&self.add_delta(&Direction::Up)) && 
        !surrounding_coordinates.contains(&self.add_delta(&Direction::Right))
      },
      Direction::UpLeft => {
        !surrounding_coordinates.contains(&self.add_delta(&Direction::Up)) && 
        !surrounding_coordinates.contains(&self.add_delta(&Direction::Left))
      },
      _ => unreachable!()
    }
  }

  pub fn is_inner_edge(&self, direction: &Direction, surrounding_coordinates: &Vec<Coordinate>) -> bool {
    match direction {
      Direction::DownLeft => {
        surrounding_coordinates.contains(&self.add_delta(&Direction::Left)) && 
        surrounding_coordinates.contains(&self.add_delta(&Direction::Down)) &&
        !surrounding_coordinates.contains(&self.add_delta(&Direction::DownLeft))
      },
      Direction::DownRight => {
        surrounding_coordinates.contains(&self.add_delta(&Direction::Down)) && 
        surrounding_coordinates.contains(&self.add_delta(&Direction::Right)) &&
        !surrounding_coordinates.contains(&self.add_delta(&Direction::DownRight))
      },
      Direction::UpRight => {
        surrounding_coordinates.contains(&self.add_delta(&Direction::Up)) && 
        surrounding_coordinates.contains(&self.add_delta(&Direction::Right)) &&
        !surrounding_coordinates.contains(&self.add_delta(&Direction::UpRight))
      },
      Direction::UpLeft => {
        surrounding_coordinates.contains(&self.add_delta(&Direction::Up)) && 
        surrounding_coordinates.contains(&self.add_delta(&Direction::Left)) &&
        !surrounding_coordinates.contains(&self.add_delta(&Direction::UpLeft))
      },
      _ => unreachable!()
    }
  }
}

#[derive(Clone, PartialEq, Debug, Eq, Hash)]
pub enum Direction {
  Up,
  Left,
  Right,
  Down,
  DownLeft,
  DownRight,
  UpRight,
  UpLeft
}

impl Direction {
  pub fn from_char(char: char) -> Direction {
    match char { 
      '^' => Direction::Up,
      '<' => Direction::Left,
      '>' => Direction::Right,
      'v' => Direction::Down ,
      _ => panic!("Cannot transform into a Direction type"),
    }
  }

  pub fn to_char(&self) -> char {
    match self { 
      Direction::Up => '^', 
      Direction::Right => '>',
      Direction::Down => 'v',
      Direction::Left => '<',
      _ => unreachable!()
    }
  }

  pub fn rotate_right(&self) -> Direction {
    match self { 
      Direction::Up => Direction::Right, 
      Direction::Right => Direction::Down,
      Direction::Down => Direction::Left,
      Direction::Left => Direction::Up,
      _ => unreachable!()
    }
  }

  pub fn delta(&self) -> Coordinate {
    match self {
      Direction::Up => Coordinate { x: -1, y:  0 },
      Direction::Down => Coordinate { x:  1, y:  0 },
      Direction::Right => Coordinate { x:  0, y:  1 },
      Direction::Left => Coordinate { x:  0, y: -1 },
      _ => unreachable!()
    }
  }

  pub fn add_delta(&self, coordinate: &Coordinate) -> Coordinate {
    match self {
      Direction::Up => Coordinate { x: coordinate.x -1, y: coordinate.y },
      Direction::Down => Coordinate { x:  coordinate.x + 1, y:  coordinate.y },
      Direction::Right => Coordinate { x:  coordinate.x, y: coordinate.y + 1 },
      Direction::Left => Coordinate { x:coordinate.x, y: coordinate.y - 1 },
      _ => unreachable!()
    }
  }

  pub fn to_vec() -> Vec<Direction> {
    let directions = [Direction::Up,  Direction::Down, Direction::Left, Direction::Right];
    directions.to_vec()
  }

  pub fn get_diagonals() -> Vec<Direction> {
    let directions = [Direction::UpRight,  Direction::DownRight, Direction::DownLeft, Direction::UpLeft];
    directions.to_vec()
  }
}

pub fn print_coordinate_map(map: &HashMap<Coordinate, char>) {
  let mut max_x = 0;
  let mut max_y = 0;
  for key in map.keys() {
    max_x = max_x.max(key.x);
    max_y = max_y.max(key.y);
  }

  for idx in 0..max_x + 1 {
    let mut line = Vec::new();
    for idy in 0..max_y + 1 {
      let coordinate = Coordinate { x: idx as i32, y: idy as i32};
      if let Some(item) = map.get(&coordinate) {
        line.push(item.to_string());
      } else {
        line.push(" ".to_string());
      }
    }

    if line.len() > 0 && !line.iter().all(|c| c == " "){
      println!("{}", line.concat())
    }
  }
}

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
