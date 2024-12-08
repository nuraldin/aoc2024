/*
 Advent of Code 2024 Day 6 

 Part one:

 There is a `guard` and its current position marked with a (^) to mark he is facing `up`
 All obstructions are marked as `#`. 
 It patrols as this:
  - If no obstacle, take a step forward
  - otherwise, turn right 90 degrees. 
 The goal is to calculate how many distinctive positions will the guard visit before leaving the mapped area, i.e. not hitting any obstacle.

 Part two:

*/
use utils::{get_challenge_part, read_puzzle_input, ChallengePart};

fn main() {
  let puzzle = parse_input();

  // println!("puzzle: {:?}", puzzle);

  match get_challenge_part() {
    ChallengePart::One => println!("The guard visited {} distinctive positions", calculate_positions(puzzle)),
    ChallengePart::Two => println!("Not implemented")
  }
}

fn calculate_positions(mut puzzle: Vec<Vec<char>>) -> usize {
  let mut police = get_initial_police_position(&puzzle);
  
  loop {
    let next_position = get_police_next_position(&police);
    let idx: usize = police.position.x.try_into().unwrap();
    let idy: usize = police.position.y.try_into().unwrap();

    // println!("Police current state: {:?}", police);
    // println!("Police next position: {:?}", next_position);

    if next_position.is_outside_puzzle(&puzzle) {
      puzzle[idy][idx] = 'X';
      break;
    } else if next_position.is_obstacle(&puzzle) {
      loop {
        police.rotate_direction();
      
        // println!("Rotated police: {:?}", police);

        if !get_police_next_position(&police).is_obstacle(&puzzle) {
          break;
        }
      }
    } 

    puzzle[idy][idx] = 'X';
    police.position = get_police_next_position(&police);

    // println!("Puzzle's current state: ");
    // for line in puzzle.clone() {
    //   println!("{:?}", line);
    // }
  }

  puzzle.iter().flatten().filter(|&&item| item == 'X' ).count()
}


fn parse_input() -> Vec<Vec<char>> {
  let mut input = Vec::new();

  for line in read_puzzle_input("./src/puzzle_input.txt") {
    input.push(line.chars().collect())
  }

  input
}

#[derive(Debug)]
struct Position {
  x: i32,
  y: i32,
}

impl Position {
  fn is_outside_puzzle(&self, puzzle: &Vec<Vec<char>>) -> bool {
    (self.x < 0 || self.y < 0) || (self.x >= puzzle.len().try_into().unwrap() || self.y >= puzzle[0].len().try_into().unwrap())
  }
  
  fn is_obstacle(&self, puzzle: &Vec<Vec<char>>) -> bool {
    let idx: usize = self.x.try_into().unwrap();
    let idy: usize = self.y.try_into().unwrap();

    puzzle[idy][idx] == '#'
  }
}

#[derive(Debug)]
enum Direction {
  Up,
  Left,
  Right,
  Down,
}

impl Direction {
  fn from_char(c: char) -> Direction {
    match c { 
      '^' => Direction::Up,
      '<' => Direction::Left,
      '>' => Direction::Right,
      'v' => Direction::Down ,
      _ => panic!("Cannot transform into a Direction type"),
    }
  }
}

#[derive(Debug)]
struct Police {
  position: Position,
  direction: Direction,
}

impl Police {
  fn rotate_direction(&mut self) {
    self.direction = match self.direction { 
      Direction::Up => Direction::Right, 
      Direction::Right => Direction::Down,
      Direction::Down => Direction::Left,
      Direction::Left => Direction::Up,
    };
  }
}

fn get_initial_police_position(puzzle: &Vec<Vec<char>>) -> Police {
  for (row_idx, row) in puzzle.iter().enumerate() {
    for (column_idx, column) in row.iter().enumerate() {
      match column {
        '>' | '<' | 'v' | '^' => return Police { 
          position: Position { 
            x: column_idx.try_into().unwrap(), 
            y: row_idx.try_into().unwrap() 
          }, 
          direction: Direction::from_char(*column)
        },
        _ => continue,
      }
    }
  }

  panic!("Couldn't find initial position, the input must be corrupted");
}

fn get_police_next_position(police: &Police) -> Position {
  match police.direction {
    Direction::Up =>  Position { x: police.position.x,     y: police.position.y - 1 },
    Direction::Right =>  Position { x: police.position.x + 1, y: police.position.y },
    Direction::Down => Position { x: police.position.x,     y: police.position.y + 1},
    Direction::Left =>    Position { x: police.position.x - 1, y: police.position.y },
  }
}