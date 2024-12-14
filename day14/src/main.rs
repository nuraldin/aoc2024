use regex::Regex;
/*
 Advent of Code 2024 Day 14: Restroom Redoubt

 There are robots that move in predictable straight lines.
 The input is a list of all of the robots current positions (p) and velocities (v).
 Each position is given as p=x,y where x is the number of tiles from the left wall and y the number of tiles from the top wall. (when viewed from above)
 Velocities are given in a similar fashion where vx is to the right, if positive, and vy to the bottom.
 Robots can share same tiles and wrap around edges.  
 The floor map is divided into quadrants. The `safety factor` is the multiplicatation of the amount of robots per quadrant.
 Robots that are exactly in the middle horizontally or vertically, do not count.
 Our puzzle map is 101 wide x 103 tall.

 Part one:

 What will the safety factor be after exactly 100 seconds have elapsed.

 Part two:

 There is an easter egg where the robots are forming a christmass tree. 
 What is the fewest number of seconds that must elapse for the robots to display the Easter egg?

 Solution:

 unfortunately for part two first time I had to copy the solution of davidkna https://www.reddit.com/r/adventofcode/comments/1hdvhvu/comment/m213uxb/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button
 as my printing wasn't showing the correct numbers. His solution worked at the first attempt.
 I noticed later that I was calculating positions wrongly, corrected that and found the image where it should have been.

 In general, the idea is to calculate using modulus arithmetic the possition after n secodns of the robots and multiply the robots of each quadrant. That worked initially but my heuristic for finding the christmas tree didn't.n
*/
use utils::{get_challenge_config, read_puzzle_input, ChallengePart};

fn main() {
    let challenge_config = get_challenge_config();
    
    let robot_list = parse_robot_list(challenge_config.is_test);
    // println!("robot_list: {robot_list:?}");

    let seconds = 7000; // Change this to test other seconds
    let room_dimension = RoomDimension {
      x: if challenge_config.is_test { 11 } else { 101 },
      y: if challenge_config.is_test { 7 } else { 103 }
    };

    match challenge_config.part {
      ChallengePart::One => println!("The safety factor after {seconds}s is: {}", safety_factor(robot_list, seconds, room_dimension)),
      ChallengePart::Two => println!("Not implemented yet"),
    }
}

fn parse_robot_list(is_test: bool) -> Vec<Robot> {
  let mut robot_list = vec![];
  let list_regex = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();

  let file_path = if is_test { "./src/example_input.txt" } else { "./src/puzzle_input.txt" };
  for line in read_puzzle_input(file_path) {
    let captured = list_regex.captures(&line).unwrap();
    
    robot_list.push(Robot {
      position: Position  {
        x: captured.get(1).unwrap().as_str().parse().unwrap(),
        y: captured.get(2).unwrap().as_str().parse().unwrap(),
      },
      speed: Speed {
        x: captured.get(3).unwrap().as_str().parse().unwrap(),
        y: captured.get(4).unwrap().as_str().parse().unwrap(),
      }
    })
  }

  robot_list
}

#[derive(Debug, Clone, PartialEq)]
struct Position {
  x: i32,
  y: i32,
}

type Speed = Position;
type RoomDimension = Position;

#[derive(Debug, PartialEq)]
struct Robot {
  position: Position,
  speed: Speed,
}

fn safety_factor(mut robot_list: Vec<Robot>, seconds: i32, room_dimension: RoomDimension) -> i32 {
  for seconds in 1..seconds {
    for robot in robot_list.iter_mut() {
      robot.position.x = (robot.position.x + robot.speed.x).rem_euclid(room_dimension.x);
      robot.position.y = (robot.position.y + robot.speed.y).rem_euclid(room_dimension.y);
    } 

    print_map(&robot_list, &room_dimension, seconds);
  }
  
  let mut top_right = vec![];
  let mut top_left = vec![];
  let mut bottom_right = vec![];
  let mut bottom_left = vec![];

  for robot in &robot_list {
    let vertical_half = room_dimension.y / 2;
    let horizontal_half = room_dimension.x / 2;

    if robot.position.x < horizontal_half && robot.position.y < vertical_half {
      top_left.push(robot);
    } else if robot.position.x < horizontal_half && robot.position.y > vertical_half {
      bottom_left.push(robot);
    } else if robot.position.x > horizontal_half && robot.position.y < vertical_half {
      top_right.push(robot);
    } else if robot.position.x > horizontal_half && robot.position.y > vertical_half {
      bottom_right.push(robot);
    }
  }

  // println!("top right:\n {:?}\n, top left:\n {:?}\n, bottom right:\n {:?}\n, bottom left:\n {:?}\n", top_right, top_left, bottom_right, bottom_left);
  // println!("top right: {}, top left: {}, bottom right: {}, bottom left {}", top_right.len(), top_left.len(), bottom_right.len(), bottom_left.len());
  (top_left.len() * top_right.len() * bottom_left.len() * bottom_right.len()) as i32
}

fn print_map(robot_list: &Vec<Robot>, room_dimension: &RoomDimension, seconds: i32) {
  let mut map_row = Vec::new();
  for idy in 0..room_dimension.y {
    let mut map_line: Vec<&str> = vec![];

    for idx in 0..room_dimension.x {
      if robot_list.iter().any(|robot| robot.position.x == idx && robot.position.y == idy) {
        map_line.push("*");
      } else {
        map_line.push(" ");
      }
    }

    map_row.push(map_line.concat());
  }

  if map_row.iter().any(|row| row.contains("******")) {
    println!("seconds: {seconds}\n");
    for row in map_row  {
      println!("{row}");
    }
  }
}