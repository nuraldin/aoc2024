use std::fs::read;

/*
 Advent of Code 2024 Day 25: Code Chronicle

 The puzzle input are schematics of every lock and every key for a floor I'm on.
 The locks are schematics with the top row filled with # and the bottom row empty, i.e. filled with '.'.
 The keys are the contrary to the locks.

 Part one:

 How many unique lock/key pairs fit together without overlapping in any column?

 Part two:
*/
use utils::{ChallengeConfig, ChallengePart};

fn main() {
  let challenge_config = ChallengeConfig::get();

  let (keys, locks) = parse_input(&challenge_config);
  // println!("keys: {:?} locks: {:?}", keys, locks);

  match challenge_config.part {
    ChallengePart::One => println!("Unique lock/keys pairs that do not overlap in any column: {}", get_unique_pairs(&keys, &locks)),
    ChallengePart::Two => println!("Not implemented yet"),
  }
}

type Combination = [i32; 5];

fn parse_input(config: &ChallengeConfig) -> (Vec<Combination>, Vec<Combination>) {
  let mut keys = vec![];
  let mut locks = vec![];


  let lines: Vec<String> = config.read_puzzle_input(None).collect(); 

  let mut i = 0;
  while i < lines.len() {
    if lines[i].is_empty() {
      i += 1;
      continue;
    }

    let slice = &lines[i..i+7];
    // println!("slice {slice:?}");
    let mut next_combination = [0, 0, 0, 0, 0];

    let inner_slice = &slice[1..slice.len()-1]; // remove the ends

    for line in inner_slice.iter() {
      for (idx, c) in line.chars().enumerate() {
        next_combination[idx] += if c == '.' { 0 } else { 1 };
      }
    }

    // println!("next combination: {next_combination:?}");
    // is key or lock
    if slice[0].chars().all(|c| c == '#') {
      keys.push(next_combination);
    } else if slice[0].chars().all(|c| c == '.' ) {
      locks.push(next_combination);
    } else {
      unreachable!("This shouldn't happen")
    }

    i += 7 
  }
  
  (keys, locks)
}

fn get_unique_pairs(keys: &Vec<Combination>, locks: &Vec<Combination>) -> i32 {
  let mut unique_pairs = 0;

  for key in keys {
    for lock in locks {
      let mut overlaps = false;

      for i in 0..5 {
        if key[i] + lock[i] > 5 {
          overlaps = true;
        }
      }

      if !overlaps {
        unique_pairs += 1;
      }
    }

  }

  unique_pairs
}