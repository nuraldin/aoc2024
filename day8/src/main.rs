/*
 Advent of Code 2024 Day 8

 Part one:

 The input is a map of antenna locations that resonate at a frequency shown by a char.
 Each antenna pair creates antinodes outside this antenas in the same line in the map. One antinode on each side only at the same distance the antenas are.
 The solution is to calculate how many unique antinodes there are, the antenna locations that have antinode are not counted.

 Part two:

 The antinodes can happen also in positions of other antennas and infinitely

*/
use utils::{ChallengeConfig, read_puzzle_input, ChallengeConfig, ChallengePart, Coordinate};

fn main() {
  let challenge_config = ChallengeConfig::get();
  let puzzle = parse_input(&challenge_config);

  // println!("Puzzle shape: {:?}", puzzle);

  match challenge_config.part {
    ChallengePart::One => println!("Number of antinodes: {}", get_antinodes(puzzle, false)),
    ChallengePart::Two => println!("Number of super antinodes: {}", get_antinodes(puzzle, true)),
  }
}

#[derive(Debug)]
struct Puzzle {
  map_size: i32,
  antennas: Vec<Antenna>
}

#[derive(Debug)]
struct AntennaPair {
  left: Antenna,
  right: Antenna
}

#[derive(Debug, Clone)]
struct Antenna {
  frequency: char,
  location: Coordinate,
}

fn parse_input(config: &ChallengeConfig) -> Puzzle {
  let mut map_size: i32 = 0;
  let mut antennas = Vec::new();

  for (row_idx, line) in read_puzzle_input(if config.is_test {"./src/example_input.txt"} else { "./src/puzzle_input.txt" }).enumerate() {
    if map_size == 0 {
      map_size = line.len() as i32;
    }

    for (col_idx, item) in line.chars().enumerate() {
      if item != '.' {
        antennas.push(Antenna { frequency: item, location: Coordinate { x: row_idx as i32, y: col_idx as i32 }})
      }
    }
  }

  Puzzle { map_size, antennas }
}

fn get_antinodes(puzzle: Puzzle, is_super: bool) -> i32 {
  let antenna_pairs = get_antenna_pairs(&puzzle.antennas);
  // println!("Antenna pairs: {:?}", antenna_pairs);

  let antenna_pairs_antinodes = get_antenna_pairs_antinodes(puzzle.map_size, &antenna_pairs, is_super);
  // println!("Antenna pair antinodes: {:?}", antenna_pairs_antinodes);

  antenna_pairs_antinodes.iter().count() as i32
}

fn get_antenna_pairs(antennas: &Vec<Antenna>) -> Vec<AntennaPair>{
  let mut antenna_pairs = Vec::new();

  for (antenna_idx, antenna) in antennas.iter().enumerate() {
    for antenna_pair_idx in antenna_idx + 1..antennas.len() {
      let antenna_pair = &antennas[antenna_pair_idx];
      if antenna.frequency == antenna_pair.frequency {
        antenna_pairs.push(AntennaPair { left: antenna.clone(), right: antenna_pair.clone() })
      }
    }
  }

  antenna_pairs
}

fn get_antenna_pairs_antinodes(map_size: i32, antenna_pairs: &Vec<AntennaPair>, is_super: bool) -> Vec<Coordinate> {
  let mut antinodes = Vec::new();

  for antenna_pair in antenna_pairs {
    let antenna_pair_distance = get_antenna_pair_distance(antenna_pair);
    // println!("Antenna pair: {:?}, 'Antenna pair distance: {:?}", antenna_pair, antenna_pair_distance);
    
    if is_super {
      if !antinodes.contains(&antenna_pair.left.location) {
        antinodes.push(antenna_pair.left.location.clone())
      }

      if !antinodes.contains(&antenna_pair.right.location) {
        antinodes.push(antenna_pair.right.location.clone())
      } 
    }

    let mut next_antinode_location = Coordinate { 
      x: antenna_pair.left.location.x + antenna_pair_distance.x, 
      y: antenna_pair.left.location.y + antenna_pair_distance.y
    };
    while !is_location_outbounds(&next_antinode_location, map_size) {
      if !antinodes.contains(&next_antinode_location) {
        antinodes.push(next_antinode_location.clone())
      }

      if !is_super {
        break;
      }

      next_antinode_location = Coordinate { 
        x: next_antinode_location.x + antenna_pair_distance.x, 
        y: next_antinode_location.y + antenna_pair_distance.y
      };
    } 

    let mut next_antinode_location = Coordinate { 
      x: antenna_pair.right.location.x - antenna_pair_distance.x, 
      y: antenna_pair.right.location.y - antenna_pair_distance.y
    };
    while !is_location_outbounds(&&next_antinode_location, map_size) {
      if !antinodes.contains(&next_antinode_location) {
        antinodes.push(next_antinode_location.clone());
      }

      if !is_super {
        break;
      }

      next_antinode_location = Coordinate { 
        x: next_antinode_location.x - antenna_pair_distance.x, 
        y: next_antinode_location.y - antenna_pair_distance.y
      };
    } 
  }

  antinodes
}

fn get_antenna_pair_distance(antenna_pair: &AntennaPair) -> Coordinate {
  let left_antenna = &antenna_pair.left;
  let right_antenna = &antenna_pair.right;

  Coordinate {
    x: left_antenna.location.x - right_antenna.location.x,
    y: left_antenna.location.y - right_antenna.location.y,
  }
}

fn is_location_outbounds(location: &Coordinate, map_size: i32) -> bool {
  location.x >= map_size|| location.x < 0 || location.y >= map_size || location.y < 0
}