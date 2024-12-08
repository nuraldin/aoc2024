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
    ChallengePart::Two => println!("Obstruction possibilites: {}", calculate_obstructions(puzzle))
  }
}

fn calculate_obstructions(mut puzzle: Vec<Vec<Location>>) -> usize {
  let mut police = get_initial_police_position(&puzzle);
  let mut obstacles = 0;

  loop {
    let next_position = get_police_next_position(&police);
    let idx: usize = police.position.x.try_into().unwrap();
    let idy: usize = police.position.y.try_into().unwrap();
    let mut rotation_flag = false;
    let mut obstacle_flag: bool = false;

    // println!("Police current state: {:?}", police);
    // println!("Police next position: {:?}", next_position);

    if next_position.is_outside_puzzle(&puzzle) {
      return obstacles
    } else if next_position.is_obstacle(&puzzle) {
      loop {
        police.rotate_direction();
      
        // println!("Rotated police: {:?}", police);

        if !get_police_next_position(&police).is_obstacle(&puzzle) {
          break;
        }
      }

      rotation_flag = true;
    } else if next_position.with_obstacle_loops(police.clone(), &puzzle) {
      obstacles += 1;
      obstacle_flag = true;
    }

    if puzzle[idy][idx].element != '^' {
      puzzle[idy][idx].element = match rotation_flag {
        true => '+',
        false => match police.direction { 
          Direction::Left | Direction::Right => if puzzle[idy][idx].element == '|' { '+' } else {'-'} , 
          Direction::Down | Direction::Up => if puzzle[idy][idx].element == '-' { '+' } else {'|'}, 
        },
      };
      puzzle[idy][idx].directions.push(police.direction.clone());
    }
    police.position = get_police_next_position(&police);

    if obstacle_flag {
      println!("There was an obstacle, puzzle's current state: ");
      for line in puzzle.clone() {
        let elements: Vec<char> = line.iter().map(|item| item.element).collect();
        println!("{:?}", elements);
      }
    }
  }
}

fn calculate_positions(mut puzzle: Vec<Vec<Location>>) -> usize {
  let mut police = get_initial_police_position(&puzzle);
  
  loop {
    let next_position = get_police_next_position(&police);
    let idx: usize = police.position.x.try_into().unwrap();
    let idy: usize = police.position.y.try_into().unwrap();

    // println!("Police current state: {:?}", police);
    // println!("Police next position: {:?}", next_position);

    if next_position.is_outside_puzzle(&puzzle) {
      puzzle[idy][idx].element = 'X';
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

    puzzle[idy][idx].element = 'X';
    police.position = get_police_next_position(&police);

    // println!("Puzzle's current state: ");
    // for line in puzzle.clone() {
    //   println!("{:?}", line);
    // }
  }

  puzzle.iter().flatten().filter(|&item| item.element == 'X' ).count()
}


fn parse_input() -> Vec<Vec<Location>> {
  let mut input = Vec::new();

  for line in read_puzzle_input("./src/example_input.txt") {
    input.push(line.chars().map(|item| Location { element: item, directions: Vec::new()}).collect())
  }

  input
}

#[derive(Clone, Debug)]
struct Position {
  x: i32,
  y: i32,
}

impl Position {
  fn is_outside_puzzle(&self, puzzle: &Vec<Vec<Location>>) -> bool {
    (self.x < 0 || self.y < 0) || (self.x >= puzzle.len().try_into().unwrap() || self.y >= puzzle[0].len().try_into().unwrap())
  }
  
  fn is_obstacle(&self, puzzle: &Vec<Vec<Location>>) -> bool {
    let idx: usize = self.x.try_into().unwrap();
    let idy: usize = self.y.try_into().unwrap();

    puzzle[idy][idx].element == '#'
  }

  fn with_obstacle_loops(&self,mut police: Police, puzzle: &Vec<Vec<Location>>) -> bool{
    // Assume I need to rotate
    police.rotate_direction();

    let idx: usize = police.position.x.try_into().unwrap();
    let idy: usize = police.position.y.try_into().unwrap();

    while !self.is_outside_puzzle(puzzle) {
      match puzzle[idy][idx].element {
        '^' => if police.direction == Direction::Up { return true } else { continue }, 
        '|' | '-' | '+' => if puzzle[idy][idx].directions.contains(&police.direction) { return true } else { continue },
        _ => continue,
      }
    }

    false
  }
}

#[derive(Clone, PartialEq, Debug)]
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

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
struct Location {
  element: char,
  directions: Vec<Direction>
}

fn get_initial_police_position(puzzle: &Vec<Vec<Location>>) -> Police {
  for (row_idx, row) in puzzle.iter().enumerate() {
    for (column_idx, column) in row.iter().enumerate() {
      match column.element {
        '>' | '<' | 'v' | '^' => return Police { 
          position: Position { 
            x: column_idx.try_into().unwrap(), 
            y: row_idx.try_into().unwrap() 
          }, 
          direction: Direction::from_char(column.element)
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