/*
 Advent of Code 2024 Day 8

 Part one:

 Part two:

*/
use utils::{get_challenge_config, read_puzzle_input, ChallengeConfig, ChallengePart};

fn main() {
  let challenge_config = get_challenge_config();
  let puzzle = parse_input(&challenge_config);

  // println!("Puzzle shape: {:?}", puzzle);

  match challenge_config.part {
    ChallengePart::One => println!("Number of antinodes: {}", get_antinodes(puzzle)),
    ChallengePart::Two => println!("Not implemented yet"),
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

#[derive(Debug, Clone, PartialEq)]
struct Location {
  row: i32,
  col: i32,
}

#[derive(Debug, Clone)]
struct Antenna {
  frequency: char,
  location: Location,
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
        antennas.push(Antenna { frequency: item, location: Location { row: row_idx as i32, col: col_idx as i32 }})
      }
    }
  }

  Puzzle { map_size, antennas }
}

fn get_antinodes(puzzle: Puzzle) -> i32 {
  let antenna_pairs = get_antenna_pairs(&puzzle.antennas);
  // println!("Antenna pairs: {:?}", antenna_pairs);

  let antenna_pairs_antinodes = get_antenna_pairs_antinodes(puzzle.map_size, &antenna_pairs);
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

fn get_antenna_pairs_antinodes(map_size: i32, antenna_pairs: &Vec<AntennaPair>) -> Vec<Location> {
  let mut antinodes = Vec::new();

  for antenna_pair in antenna_pairs {
    let antenna_pair_distance = get_antenna_pair_distance(antenna_pair);
    // println!("Antenna pair: {:?}, 'Antenna pair distance: {:?}", antenna_pair, antenna_pair_distance);

    if !is_location_outbounds(&antenna_pair.left.location, &antenna_pair_distance, map_size, false) {
      let antinode = Location { 
        row: antenna_pair.left.location.row + antenna_pair_distance.row, 
        col: antenna_pair.left.location.col + antenna_pair_distance.col
      };
      if !antinodes.contains(&antinode) {
        antinodes.push(antinode)
      }
    } 


    if !is_location_outbounds(&antenna_pair.right.location, &antenna_pair_distance, map_size, true) {
      let antinode = Location { 
        row: antenna_pair.right.location.row - antenna_pair_distance.row, 
        col: antenna_pair.right.location.col - antenna_pair_distance.col
      };

      if !antinodes.contains(&antinode) {
        antinodes.push(antinode);
      }
    } 

  }

  antinodes
}

fn get_antenna_pair_distance(antenna_pair: &AntennaPair) -> Location {
  let left_antenna = &antenna_pair.left;
  let right_antenna = &antenna_pair.right;

  Location {
    row: left_antenna.location.row - right_antenna.location.row,
    col: left_antenna.location.col - right_antenna.location.col,
  }
}

fn is_location_outbounds(location: &Location, distance: &Location, map_size: i32, is_right: bool) -> bool {
  let multiplier = if is_right { -1 } else { 1 };
  let antinode_row = location.row + (distance.row * multiplier);
  let antinode_col = location.col + (distance.col * multiplier);

  antinode_row >= map_size|| antinode_row < 0 || antinode_col >= map_size || antinode_col < 0
}