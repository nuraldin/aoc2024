use std::vec;

/*
 Advent of Code 2024 Day 19: Linen Layout

 The puzzle input is a collection of available towel patterns and a list of desired designs.
 I need to help an onsen hotel to arrange their towels.
 Every towel is marked with a pattern of colored stripes. The stripes can be:
  - w (white)
  - r (red)
  - g (green)
  - u (blue)
  - b (black) 
 e.g. ggr would have a green stripe, a green strip and then a red stripe.
 There are however standard designs that are combinations of colors.

 Part one:

 How many designs are possible?

 Part two:
*/
use utils::{ChallengeConfig, ChallengePart};

fn main() {
  let challenge_config = ChallengeConfig::get();
  
  let (patterns, designs) = parse_input(&challenge_config);
  // println!("patterns: {patterns:?}, designs: {designs:?}");

  match challenge_config.part {
    ChallengePart::One => println!("The amount of possible designs is: {}", possible_designs(designs, &patterns)),
    ChallengePart::Two => println!("Not implemented yet"),
  }
}

fn parse_input(config: &ChallengeConfig) -> (Vec<String>, Vec<String>) {
  let mut patterns = vec![];
  let mut designs = vec![];

  for (idx, line) in config.read_puzzle_input(None).enumerate() {
    if idx == 0 {
      patterns = line
        .split(", ")
        .map(|line| line.to_string())
        .collect();
      
      continue;
    } else if idx == 1 {
      continue;
    }

    designs.push(line);
  }

  (patterns, designs)
}

fn possible_designs(ds: Vec<String>, ps: &Vec<String>) -> i32 {
  let mut possible_designs = 0;

  for d in ds {
    if is_design_possible(d, &ps) {
      possible_designs += 1;
    }
  }

  possible_designs
}

fn is_design_possible(d: String, ps: &Vec<String>) -> bool {
  println!("d: {}, ps: {:?}", d, ps);

  if d.len() == 0 {
    return true;
  }

  if !ps.iter().any(|ps| d.starts_with(ps)) {
    return false;
  }

  let possible_patterns: Vec<String> = ps.iter().filter(|p| d.starts_with(*p)).cloned().collect();
  for pattern in possible_patterns {
    let next_pattern = d.clone().strip_prefix(&pattern).unwrap().to_string();

    if is_design_possible(next_pattern, ps) {
      return true;
    }
  }

  false
}

#[cfg(test)]
mod tests {
  use utils::TEST_CONFIG;

use super::*;

  #[test]
  fn is_brwrr_possible_with_example_input() {
    let design = "brwrr".to_string();
    let patterns: Vec<String> = ["r", "wr", "b", "g", "bwu", "rb", "gb", "br"].iter().map(|p| p.to_string()).collect();
    
    assert!(is_design_possible(design, &patterns))
  }

  #[test]
  fn is_ubwu_impossible_with_example_input() {
    let design = "ubwu".to_string();
    let patterns: Vec<String> = ["r", "wr", "b", "g", "bwu", "rb", "gb", "br"].iter().map(|p| p.to_string()).collect();
   
    assert!(!is_design_possible(design, &patterns))
  }

  #[test]
  fn example_input_returns_6_possible_desings() { 
    let (patterns, designs) = parse_input(&TEST_CONFIG);

    assert_eq!(possible_designs(designs, &patterns), 6);
  }
}