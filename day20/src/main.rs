/*
 Advent of Code 2024 Day 20: Race Condition

 The programs compete to see who can finish in the fewest picoseconds.
 The input is the map of a racetrack.
 The track is marked with a . the walls with a #, the start with a S and the end with an E.
 Each move in the racetrack takes 1 picosecond.
 However, programs are allowed to cheat. Exactly once during a race a program may disable collision for up to 2 picoseconds.
 This allows to pass through walls as if they were regular track.

 Part one:

 How many cheats would save you at least 100 picoseconds?

 Part two:

 Now collisions disabling can last up to 20 picoseconds.
 How many cheats would save me at least 100 picoseconds?

*/
use utils::{get_challenge_config, print_coordinate_map, read_puzzle_input, ChallengePart, Coordinate, Direction, TopographicMap};
use std::collections::HashSet;

fn main() {
    let challenge_config = get_challenge_config();

    let race_map = parse_input(challenge_config.is_test);
    // print_coordinate_map(&race_map);

    match challenge_config.part {
      ChallengePart::One => println!("The amount of cheats that will save me 100 picoseconds is {}", find_cheats_atleast(&race_map, 100)),
      ChallengePart::Two => println!("Not implemented yet"),
    }
}

fn parse_input(is_test: bool) -> TopographicMap<char> {
  let mut map = TopographicMap::new();

  let file_path = if is_test { "./src/example_input.txt" } else { "./src/puzzle_input.txt" };
  
  for (row_idx, line) in read_puzzle_input(file_path).enumerate() {
    for (col_idx, c) in line.chars().enumerate() {
      map.insert(
        Coordinate::new(row_idx as i32, col_idx as i32),
        c
      );
    }
  }

  map
}

fn run_track(puzzle_map: &TopographicMap<char>) -> i32 {
  // get starting position
  let mut curr_pos = find_in_map(&puzzle_map, 'S').unwrap();
  let mut prev_pos = curr_pos.clone();

  let mut picoseconds = 0;
  
  while *puzzle_map.get(&curr_pos).unwrap() != 'E' {
    for direction in Direction::to_vec() {
      let next_pos = curr_pos.add_delta(&direction);
      match  puzzle_map.get(&next_pos) {
        Some(value) => {
          if (*value == '.' || *value == 'E') && prev_pos != next_pos {
            // println!("prev_pos: {prev_pos:?}, next_pos: {curr_pos:?}, picoseconds: {picoseconds:?}");
            prev_pos = curr_pos.clone();
            curr_pos = next_pos.clone();
            break;
          }
        },
        None => ()
      }
    }
    
    picoseconds += 1;
  }

  picoseconds
}

// returns the path done by the program
fn track_path(puzzle_map: &TopographicMap<char>) -> Vec<Coordinate> {
  let mut path = vec![];
  // get starting position
  let mut curr_pos = find_in_map(&puzzle_map, 'S').unwrap();
  let mut prev_pos = curr_pos.clone();
  path.push(curr_pos.clone());

  while *puzzle_map.get(&curr_pos).unwrap() != 'E' {
    for direction in Direction::to_vec() {
      let next_pos = curr_pos.add_delta(&direction);
      match  puzzle_map.get(&next_pos) {
        Some(value) => {
          if (*value == '.' || *value == 'E') && prev_pos != next_pos {
            // println!("prev_pos: {prev_pos:?}, next_pos: {curr_pos:?}, picoseconds: {picoseconds:?}");
            prev_pos = curr_pos.clone();
            curr_pos = next_pos.clone();
            path.push(curr_pos.clone());
            break;
          }
        },
        None => ()
      }
    }
  }

  path
}

fn find_in_map(map: &TopographicMap<char>,item: char) -> Option<Coordinate> {
  for (key, value) in map.clone() {
    if value == item {
      return Some(key);
    }
  }

  None
}

fn find_cheats(race_map: &TopographicMap<char>, duration: i32) -> i32 {
  let disable_duration = 2;
  let race_duration = run_track(&race_map);
  let race_track: Vec<Coordinate> = track_path(&race_map);
  // println!("race duration: {race_duration}");
  // println!("race track: {race_track:?}");

  let mut cheats = 0;
  for (curr_pos, curr) in race_track.clone().iter().enumerate() {
    let possible_cheats = possible_cheats(curr.clone(), &race_map);
    for possible_cheat in possible_cheats {
      let cheat_pos = race_track.iter().position(|pos| *pos == possible_cheat).unwrap();
    
      if cheat_pos < curr_pos {
        continue;
      }

      let savings = (cheat_pos - curr_pos) as i32 - disable_duration;
      // println!("duration: {race_duration}, cur pos: {curr_pos}, cheat pos: {cheat_pos}, savings: {savings}");
      
      if savings == duration {
        cheats += 1;
      }
    } 
  } 

  cheats
}

fn find_cheats_upto(race_map: &TopographicMap<char>, duration: i32, secs: i32) -> i32 {
  let disable_duration = 2;
  let race_track: Vec<Coordinate> = track_path(&race_map);

  let mut cheats = 0;
  for (curr_pos, curr) in race_track.clone().iter().enumerate() {
    let possible_cheats = possible_cheats_upto(curr.clone(), &race_map, secs);
    for possible_cheat in possible_cheats {
      let cheat_pos = race_track.iter().position(|pos| *pos == possible_cheat).unwrap();
    
      if cheat_pos < curr_pos {
        continue;
      }

      let savings = (cheat_pos - curr_pos) as i32 - disable_duration;
      // println!("duration: {race_duration}, cur pos: {curr_pos}, cheat pos: {cheat_pos}, savings: {savings}");
      
      if savings == duration {
        cheats += 1;
      }
    } 
    break;
  } 

  cheats
}

fn find_cheats_atleast(race_map: &TopographicMap<char>, duration: i32) -> i32 {
  let disable_duration = 2;
  let race_duration = run_track(&race_map);
  let race_track: Vec<Coordinate> = track_path(&race_map);
  println!("race duration: {race_duration}");
  // println!("race track: {race_track:?}");

  let mut cheats = 0;
  for (curr_pos, curr) in race_track.clone().iter().enumerate() {
    let possible_cheats = possible_cheats(curr.clone(), &race_map);
    for possible_cheat in possible_cheats {
      let cheat_pos = race_track.iter().position(|pos| *pos == possible_cheat).unwrap();
    
      if cheat_pos < curr_pos {
        continue;
      }

      let savings = (cheat_pos - curr_pos) as i32 - disable_duration;
      // println!("duration: {race_duration}, cur pos: {curr_pos}, cheat pos: {cheat_pos}, savings: {savings}");
      
      if savings >= duration {
        cheats += 1;
      }
    } 
  } 

  cheats
}

fn possible_cheats(pos: Coordinate, map: &TopographicMap<char>) -> Vec<Coordinate> {
  let mut possible_cheats = Vec::new();

  for direction in Direction::to_vec() {
    let mut next_pos = pos.add_delta(&direction);
    if let Some(value) = map.get(&next_pos) {
      if *value == '#' {
        next_pos = next_pos.add_delta(&direction);
        if let Some(value) = map.get(&next_pos) {
          if ['.', 'E'].contains(value) {
            possible_cheats.push(next_pos);
          }
        }
      }
    }
  }

  possible_cheats
}

fn possible_cheats_recursive(pos: Coordinate, map: &TopographicMap<char>, remaining_secs: i32, possible_cheats: &mut HashSet<Coordinate>) -> HashSet<Coordinate>{
  println!("pos: {pos:?}, remaining secs: {remaining_secs}, possible_cheats: {possible_cheats:?}");
  if remaining_secs < 0 {
    return possible_cheats.clone();
  }

  let mut next = vec![];
  for direction in Direction::to_vec() {
    let prev_pos = pos.add_delta(&direction);

    if let Some(value) = map.get(&prev_pos) {
      if *value == '#' {
        let next_pos = prev_pos.add_delta(&direction);
        if let Some(value) = map.get(&next_pos) {
          match *value {
            '.' | 'E' => {
              possible_cheats.insert(next_pos);
            },
            '#' => {
              next.push(value)
            },
            _ => (),
          }
        }
      }
    }
  }

  if next.len() == 0 {
    return possible_cheats.clone()
  }

  for next_pos in next {
    possible_cheats_recursive(pos, map, remaining_secs - 2, possible_cheats)
  }

  return possible.cheats
}

fn possible_cheats_upto(pos: Coordinate, map: &TopographicMap<char>, secs: i32) -> HashSet<Coordinate> {
  let possible_cheats = possible_cheats_recursive(pos, map, secs, &mut HashSet::new());
  println!("possible cheats recursive: {possible_cheats:?}");

  possible_cheats
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn start_position_is_right() {
    let puzzle_map = parse_input(true);
    
    assert_eq!(find_in_map(&puzzle_map, 'S').unwrap(), Coordinate::new(3, 1));
  }

  #[test]
  fn finish_position_is_right() {
    let puzzle_map = parse_input(true);
    
    assert_eq!(find_in_map(&puzzle_map, 'E').unwrap(), Coordinate::new(7, 5));
  }

  #[test] 
  fn example_racetrack_finishes_in_84_picoseconds() {
    let puzzle_map = parse_input(true);

    assert_eq!(run_track(&puzzle_map), 84);
  }

  #[test]
  fn test_example_2_picoseconds_cheats() {
    let puzzle_map = parse_input(true);
    
    assert_eq!(find_cheats(&puzzle_map, 2), 14, "There should be 14 cheats that save 2 picoseconds");
    assert_eq!(find_cheats(&puzzle_map, 4), 14, "There should be 14 cheats that save 4 picoseconds");
    assert_eq!(find_cheats(&puzzle_map, 6),  2, "There should be 2 cheats that save 6 picoseconds");
    assert_eq!(find_cheats(&puzzle_map, 8),  4, "There should be 4 cheats that save 8 picoseconds");
    assert_eq!(find_cheats(&puzzle_map, 10), 2, "There should be 2 cheats that save 10 picoseconds");
    assert_eq!(find_cheats(&puzzle_map, 12), 3, "There should be 3 cheats that save 12 picoseconds");
    assert_eq!(find_cheats(&puzzle_map, 20), 1, "There should be 1 cheats that save 20 picoseconds");
    assert_eq!(find_cheats(&puzzle_map, 36), 1, "There should be 1 cheats that save 36 picoseconds");
    assert_eq!(find_cheats(&puzzle_map, 38), 1, "There should be 1 cheats that save 38 picoseconds");
    assert_eq!(find_cheats(&puzzle_map, 40), 1, "There should be 1 cheats that save 40 picoseconds");
    assert_eq!(find_cheats(&puzzle_map, 64), 1, "There should be 1 cheats that save 64 picoseconds");
  }

  #[test]
  fn test_example_atleast_cheats() {
    let puzzle_map = parse_input(true);
    
    assert_eq!(find_cheats_atleast(&puzzle_map, 20), 5, "There should be at least 5 cheats that save 20 picoseconds");
  }

  #[test]
  fn test_example_20_picoseconds_cheats() {
    let puzzle_map = parse_input(true);
    
    assert_eq!(find_cheats_upto(&puzzle_map, 50, 20), 32, "There should be 32 cheats that save 50 picoseconds");
    assert_eq!(find_cheats_upto(&puzzle_map, 52, 20), 31, "There should be 31 cheats that save 52 picoseconds");
    assert_eq!(find_cheats_upto(&puzzle_map, 54, 20), 29, "There should be 29 cheats that save 54 picoseconds");
    assert_eq!(find_cheats_upto(&puzzle_map, 56, 20), 39, "There should be 39 cheats that save 56 picoseconds");
    assert_eq!(find_cheats_upto(&puzzle_map, 58, 20), 25, "There should be 25 cheats that save 58 picoseconds");
    // assert_eq!(find_cheats(&puzzle_map, 60), 23, "There should be 23 cheats that save 60 picoseconds");
    // assert_eq!(find_cheats(&puzzle_map, 62), 20, "There should be 20 cheats that save 62 picoseconds");
    // assert_eq!(find_cheats(&puzzle_map, 64), 19, "There should be 19 cheats that save 64 picoseconds");
    // assert_eq!(find_cheats(&puzzle_map, 66), 12, "There should be 12 cheats that save 66 picoseconds");
    // assert_eq!(find_cheats(&puzzle_map, 68), 14, "There should be 14 cheats that save 68 picoseconds");
    // assert_eq!(find_cheats(&puzzle_map, 70), 12, "There should be 12 cheats that save 70 picoseconds");
    // assert_eq!(find_cheats(&puzzle_map, 72), 22, "There should be 22 cheats that save 72 picoseconds");
    // assert_eq!(find_cheats(&puzzle_map, 74),  4, "There should be 3 cheats that save 74 picoseconds");
    // assert_eq!(find_cheats(&puzzle_map, 76),  3, "There should be 4 cheats that save 76 picoseconds");
  }
}