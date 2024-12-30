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

 How many positons could I put an obstruction and generate an infinite loop?

 Solution:

 The first part was pretty straight-forward with a solution a map of coordinates pointint o map elements.
 Then the previous iterations were different but after seeing in internet other solutions I used the suggested set for tracking positions of the police.
 In the second part the tricky part is to realize that the obstruction may have not been if the police started from the beginning, i.e. I was trying to check every time I was moving the police and that made that issue.
 The correct, but brute force, solution is to check every next position for loops but starting from the police starting position. 
*/
use std::collections::{HashMap,HashSet};

use utils::{ChallengeConfig, ChallengePart, Coordinate, Direction };

fn main() {
  let challenge_config = ChallengeConfig::get();

  let (police_position, puzzle_map) = parse_input(&challenge_config);
  // println!("puzzle: {:?}", puzzle);

  match challenge_config.part {
    ChallengePart::One => println!("The guard visited {} distinctive positions", calculate_positions(police_position, puzzle_map)),
    ChallengePart::Two => println!("Obstruction possibilites: {}", calculate_obstructions(police_position, puzzle_map))
  }
}

fn parse_input(config: &ChallengeConfig) -> (Coordinate, HashMap<Coordinate, char>) {
  let mut puzzle_map = HashMap::new();
  let mut police_start_coordinate = Coordinate { x: 0, y: 0 };

  for (row_idx, row) in config.read_puzzle_input(None).enumerate() {
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

fn calculate_obstructions(mut police_position: Coordinate, mut puzzle_map: HashMap<Coordinate, char>) -> usize {
  let mut police_direction = Direction::Up;
  let mut obstacle_positions: HashSet<Coordinate> = HashSet::new();

  let initial_police_position = police_position.clone();
  let initial_police_direction = police_direction.clone();
  let mut possible_position = police_position.add_delta(&police_direction);
  while let Some(item) = puzzle_map.get(&possible_position) {
    if *item == '#' {
      police_direction = police_direction.rotate_right();
    } else {
      let mut clone_map = puzzle_map.clone();
      if loops(possible_position.clone(), initial_police_position.clone(), initial_police_direction.clone(), &mut clone_map) {
        obstacle_positions.insert(possible_position.clone());
        puzzle_map.insert(possible_position.clone(), 'O');
        // println!("next obstruction -------");
        // print_map(&clone_map);
      }
      police_position = possible_position.clone();
    }
    
    possible_position = police_position.add_delta(&police_direction);
  }

  // print_map(&puzzle_map);
  println!("obstacle posibilities: {:?}", obstacle_positions);
  obstacle_positions.iter().count()
}

fn loops(obstacle: Coordinate,mut police_position: Coordinate, mut police_direction: Direction, puzzle_map: &mut HashMap<Coordinate, char>) -> bool {
  puzzle_map.insert(obstacle.clone(), '#');
  // police_direction = police_direction.rotate_right(); 
  let mut visited_positions: HashSet<(Coordinate, Direction)> = HashSet::from([(police_position.clone(), police_direction.clone())]);
  
  let mut possible_position = police_position.add_delta(&police_direction);
  while let Some(item) = puzzle_map.get(&possible_position) {
    //println!("next possible positon: {possible_position:?} direction: {police_direction:?} item: {item} visited positions: {visited_positions:?}");
    if *item == '#' {
      police_direction = police_direction.rotate_right();
      // visited_positions.insert((police_position.clone(), police_direction.clone()));
    } else {
      visited_positions.insert((police_position.clone(), police_direction.clone()));
      puzzle_map.insert(police_position, 'X');
      police_position = possible_position.clone();
    }
    
    if visited_positions.contains(&(police_position.clone(), police_direction.clone())) {
      return true;
    }

    // println!("visited_positions: {visited_positions:?}");
    possible_position = police_position.add_delta(&police_direction);
  }

  false
}

fn calculate_positions(mut police_position: Coordinate, mut puzzle_map: HashMap<Coordinate, char>) -> usize {
  let mut police_direction = Direction::Up;
  let mut visited_positions: HashSet<Coordinate> = HashSet::from([police_position.clone()]);

  let mut possible_position = police_position.add_delta(&police_direction);
  while let Some(item) = puzzle_map.get(&possible_position) {
    if *item == '#' {
      police_direction = police_direction.rotate_right();
    } else {
      police_position = possible_position.clone();
      visited_positions.insert(police_position.clone());
    }
    
    possible_position = police_position.add_delta(&police_direction);
  }

  visited_positions.len()
}
