/*
 Advent of Code 2024 Day 6: Guard Gallivant

 A guard is patrolling a lab. He is marked with a ^ in the puzzle input map.
 The map is our input. Apart from the guard the map also shows obstructions shown as #
 The guard patrol always follows this steps:
  - If there is no obstacle, take a step forward.
  - Otherwise, turn right 90 degrees.
 The guard will always eventually leave the mapped area.
 All positions visited by the guard are marked with an X.

 Part one:

 How many distinct positions will the guard visit before leaving the mapped area?

 Part two:

*/
use std::collections::{HashMap,HashSet};
use utils::{get_challenge_config, read_puzzle_input, ChallengePart, Coordinate, Direction };

fn main() {
  let challenge_config = get_challenge_config();

  let (police_position, puzzle_map) = parse_input(challenge_config.is_test);
  // println!("puzzle: {:?}", puzzle);

  match challenge_config.part {
    ChallengePart::One => println!("The guard visited {} distinctive positions", calculate_positions(police_position, puzzle_map)),
    ChallengePart::Two => println!("Obstruction possibilites: {}", calculate_obstructions(police_position, puzzle_map))
  }
}

fn calculate_obstructions(mut police_position: Coordinate, mut puzzle_map: HashMap<Coordinate, char>) -> usize {
  let mut obstacles = 0;
  let mut police_direction = Direction::Up;
  let mut possible_position = police_direction.add_delta(&police_position);

  while let Some(item) = puzzle_map.get(&possible_position) {
    if *item != '#' {
      if loops(possible_position.clone(), police_position.clone(), police_direction.clone(), puzzle_map.clone()) {
        obstacles += 1;
      } 
    } else if *item == '#' {
      police_direction = police_direction.rotate_right();
      possible_position = police_direction.add_delta(&police_position);
      while *puzzle_map.get(&possible_position).unwrap() == '#' {
        police_direction = police_direction.rotate_right();
        possible_position = police_direction.add_delta(&police_position);
      } 
    }

    police_position = possible_position.clone();
    possible_position = police_direction.add_delta(&possible_position);

  }

  obstacles
}

fn loops(obstacle: Coordinate,mut police_position: Coordinate, mut police_direction: Direction, mut puzzle_map: HashMap<Coordinate, char>) -> bool {
  let mut cycle: HashSet<(Coordinate,Direction)> = HashSet::new();

  puzzle_map.insert(obstacle.clone(), '#');

  cycle.insert((police_position.clone(), police_direction.clone()));
  police_direction = police_direction.rotate_right(); 
  cycle.insert((police_position.clone(), police_direction.clone()));
  
  let mut possible_position = police_direction.add_delta(&police_position);
  while let Some(item) = puzzle_map.get(&possible_position) {
    if *item == '#' {
      police_direction = police_direction.rotate_right();
      possible_position = police_direction.add_delta(&police_position);
      while *puzzle_map.get(&possible_position).unwrap() == '#' {
        police_direction = police_direction.rotate_right();
        possible_position = police_direction.add_delta(&police_position);
      } 
    }

    cycle.insert((police_position.clone(), police_direction.clone()));
    police_position = possible_position.clone();
    possible_position = police_direction.add_delta(&possible_position);

    if cycle.contains(&(police_position.clone(), police_direction.clone())) {
      // println!("cycle: {cycle:?}");
      // println!("cycle contains pol: {police_position:?} dir: {police_direction:?}");
      return true;
    }
  }
  
  false
}

fn calculate_positions(mut police_position: Coordinate, mut puzzle_map: HashMap<Coordinate, char>) -> usize {
  let mut police_direction = Direction::Up;
  let mut possible_position = police_direction.add_delta(&police_position);

  while let Some(item) = puzzle_map.get(&possible_position) {
    if *item == '#' {
      police_direction = police_direction.rotate_right();
      possible_position = police_direction.add_delta(&police_position);
      while *puzzle_map.get(&possible_position).unwrap() == '#' {
        police_direction = police_direction.rotate_right();
        possible_position = police_direction.add_delta(&police_position);
      } 
    }

    puzzle_map.insert(police_position.clone(), 'X');
    puzzle_map.insert(possible_position.clone(), police_direction.to_char());
    police_position = possible_position.clone();
    possible_position = police_direction.add_delta(&possible_position);
  }

  puzzle_map.values().filter(|&item| *item == 'X' ).count() + 1
}

fn get_next_possible_direction(direction: Direction, position: &Coordinate, puzzle_map: &HashMap<Coordinate, char>) -> Direction {
  // println!("found obstacle at {possible_position:?} police will rotate and calculate possible next position");
  let mut direction = direction.rotate_right();
  let mut possible_position = direction.add_delta(&position);
  // println!("After rotation, next possible position is: {possible_position:?}");
  while *puzzle_map.get(&possible_position).unwrap() == '#' {
    // println!("found obstacle at {possible_position:?} police will rotate and calculate possible next position");
    direction = direction.rotate_right();
    possible_position = direction.add_delta(&position);
    // println!("After rotation, next possible position is: {possible_position:?}");
  } 

  direction
}

fn parse_input(is_test: bool) -> (Coordinate, HashMap<Coordinate, char>) {
  let mut puzzle_map = HashMap::new();
  let mut police_start_coordinate = Coordinate { x: 0, y: 0 };

  let file_path = if is_test { "./src/example_input.txt" } else { "./src/puzzle_input.txt" };

  for (row_idx, row) in read_puzzle_input(file_path).enumerate() {
    for (col_idx, item) in row.chars().enumerate() {
      let coordinate = Coordinate { x: row_idx as i32, y: col_idx as i32 };
      if item == '^' {
        police_start_coordinate = coordinate.clone();
      }
      puzzle_map.insert(coordinate, item);
    }
  }

  (police_start_coordinate, puzzle_map)
}

fn print_map(map: &HashMap<Coordinate, char>) {
  for idx in 0..map.len() {
    let mut line = Vec::new();
    for idy in 0..map.len() {
      let coordinate = Coordinate { x: idx as i32, y: idy as i32};
      if let Some(item) = map.get(&coordinate) {
        line.push(item.to_string());
      }
    }

    if line.len() > 0 {
      println!("{}", line.concat())
    }
  }
}