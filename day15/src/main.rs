use std::{collections::HashMap, vec};

/*
 Advent of Code 2024 Day 15: Warehouse Woes

 The input is a map of a warehouse and a list of movements a robot will attempt to make.
 The movements will not always succeed as the warehouse has boxes that are shifted around.
 If the robot collides with a box (0) it will try to move it, if it is a wall (#) it will not move.
 The movements are (^:up <: left, >:right, v: down). and the list is a giant sequence in order.
 The boxes have GPS coordinates to track the mwhich is 100 times the distance from the top edge plus ist distance from the edge of the map. i.e. 100x + y.
 The robot is marked with an @

 Part one:

 What is the sum of all boxes' GPS coordinates?

 Part two:

 Everything except the robot is twice as wide.
 the distances are now measured from the edge of the map to the closest edge of the box in question.
 These are the rules to making it wider:

  If the tile is #, the new map contains ## instead.
  If the tile is O, the new map contains [] instead.
  If the tile is ., the new map contains .. instead.
  If the tile is @, the new map contains @. instead.

 Solution: 


*/
use utils::{ChallengeConfig, read_puzzle_input, ChallengePart};

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct Coordinate {
 x: i32,
 y: i32,
}

fn main() {
    let challenge_config = ChallengeConfig::get();

    let (robot, puzzle_map, instructions) = parse_puzzle_input(challenge_config.is_test);
    
    match challenge_config.part {
      ChallengePart::One => println!("Sum of all final GPS coordinates: {:?}", sum_gps_coordinates(robot, puzzle_map, instructions)),
      ChallengePart::Two => println!("Not implemented yet"),
    }
}

fn parse_puzzle_input(is_test: bool) -> (Coordinate, HashMap<Coordinate, char>, Vec<char>) {
  let mut instructions: Vec<char> = vec![];
  let mut puzzle_map: HashMap<Coordinate, char> = HashMap::new();
  let mut robot = Coordinate { x: 0, y: 0};

  let (map, directions) = if is_test { 
    ("./src/example_map.txt", "./src/example_robot_directions.txt") 
  } else { 
    ("./src/puzzle_map.txt", "./src/puzzle_robot_directions.txt") 
  };

  // parse boxes map
  for (row_idx, row) in read_puzzle_input(map).enumerate() {
    for (col_idx, location) in row.chars().enumerate() {
      let coordinate = Coordinate {
        x: row_idx as i32,
        y: col_idx as i32
      }; 

      if ['@', 'O' , '#' ].contains(&location) {
        if location == '@' {
          robot = coordinate.clone();
        }
        puzzle_map.insert(coordinate, location);
      }
    }
  }

  // parse robot direction instructions map
  for line in read_puzzle_input(&directions) {
    for direction in line.chars() {
      instructions.push(direction);
    }
  }
  
  (robot, puzzle_map, instructions)
}

fn sum_gps_coordinates(mut robot: Coordinate, mut puzzle_map: HashMap<Coordinate, char>, instructions: Vec<char>) -> i32 {
  calculate_final_coordinates(&mut robot, &mut puzzle_map, instructions);

  let mut sum = 0;
  for (coordinate, element) in puzzle_map {
    if element == 'O' {
      let gps_coordinate = coordinate.x * 100 + coordinate.y;
      // println!("gps coordinate for {:?}: {}", coordinate, gps_coordinate);
      sum += gps_coordinate;
    }
  } 

  sum
}

// Remember that idx 0 
fn calculate_final_coordinates(robot: &mut Coordinate, puzzle_map: &mut HashMap<Coordinate, char>, instructions: Vec<char>) {
  // println!("Starting state: ");
  for instruction in instructions {
    // print_current_location(puzzle_map);

    let delta = match instruction {
      '^' => Coordinate { x: -1, y:  0 },
      'v' => Coordinate { x:  1, y:  0 },
      '>' => Coordinate { x:  0, y:  1 },
      '<' => Coordinate { x:  0, y: -1 },
      _ => panic!("This shoulnd't be reachable"),
    };


    let next_robot_coordinate = Coordinate { x: robot.x + delta.x, y: robot.y + delta.y };
    // println!("instruction: {instruction} delta: {delta:?} next coordinate: {next_robot_coordinate:?}");

    match puzzle_map.get_mut(&next_robot_coordinate) { 
      Some(element) => {
        if *element == '#' {
          continue;
        } else if *element == 'O' {
          let mut other_coordinate = next_robot_coordinate.clone();
          
          let mut subset: HashMap<Coordinate, char> = HashMap::new();
          while let Some(element) = puzzle_map.get(&other_coordinate) {
            other_coordinate = Coordinate { x: other_coordinate.x + delta.x, y: other_coordinate.y + delta.y };
            subset.insert(other_coordinate.clone(), *element);
          }

          // println!("subset values: {subset:?}");
          if !subset.values().any(|value| *value == '#' ) {
            for coordinate in subset.keys().cloned() {
              puzzle_map.insert(coordinate, 'O');
            }
            puzzle_map.remove(*&robot);
            puzzle_map.insert(next_robot_coordinate.clone(), '@');
            *robot = next_robot_coordinate;
          }
        }
      },
      None => { 
        puzzle_map.remove(&robot);
        puzzle_map.insert(next_robot_coordinate.clone(), '@'); 
        *robot = next_robot_coordinate;
      }
    }
  }
}

fn print_current_location(puzzle_map: &mut HashMap<Coordinate, char>) {
  for x in 0..30 {
    let mut row: Vec<String> = vec![];

    for y in 0..30 {
      let coordinate = Coordinate { x, y };
      if let Some(value) = puzzle_map.get(&coordinate).cloned() {
        let item = format!("{value}");
        row.push(item);
      } else {
        row.push(".".to_string());
      }
    }

    println!("{}", row.concat());
  }
}