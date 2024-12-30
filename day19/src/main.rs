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
    if is_design_possible(d, &ps, 0) {
      possible_designs += 1;
    }
  }

  possible_designs
}

fn is_design_possible_bfs(d: String, ps: &Vec<String>, depth: i32) -> bool {
  // get all possible patterns that could be used in the string
  let ps: Vec<String> = ps.iter().filter(|p| d.contains(*p)).cloned().collect();
  println!("design: {}, possible patterns: {:?}, depth: {}", d, ps, depth);

  // From all the possible, get the ones that would start the string
  let possible_patterns: Vec<String> = ps.iter().filter(|p| d.starts_with(*p)).cloned().collect();
  let mut combinations: Vec<String> = vec![];
  println!("possible patterns: {possible_patterns:?}");

  // Get the remaining string depending on what was already taken away
  for (idx, pattern ) in possible_patterns.iter().enumerate() {
    println!("testing pattern {idx}: {pattern}");
    if let Some(next_pattern) = d.strip_prefix(pattern) {
      combinations.push(next_pattern.to_string());
    } 
  }

  // if there are no possible patterns and any of the combinations has length zero, it succeeded on forming the design
  if combinations.len() == 0 {
    return true;
  }

  combinations.iter();

  true
}

fn is_design_possible(d: String, ps: &Vec<String>, depth: i32) -> bool {
  let ps: Vec<String> = ps.iter().filter(|p| d.contains(*p)).cloned().collect();
  println!("design: {}, possible patterns: {:?}, depth: {}", d, ps, depth);

  // if there is no more string to tokenize, the design is possible
  if d.len() == 0 {
    println!("There is no more string to tokenize");
    return true;
  }

  // if no pattern is possible, short circuit to false
  if ps.len() == 0 || !ps.iter().any(|ps| d.starts_with(ps)) {
    println!("There is no more patterns to try");
    return false;
  }

  let possible_patterns: Vec<String> = ps.iter().filter(|p| d.starts_with(*p)).cloned().collect();
  println!("possible patterns: {possible_patterns:?}");
  for (idx, pattern ) in possible_patterns.iter().enumerate() {
    println!("testing pattern {idx}: {pattern}");
    if let Some(next_pattern) = d.strip_prefix(pattern) {
      if is_design_possible(next_pattern.to_string(), &ps, depth + 1) {
        return true;
      }
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
    
    assert!(is_design_possible(design, &patterns, 0))
  }

  #[test]
  fn is_ubwu_impossible_with_example_input() {
    let design = "ubwu".to_string();
    let patterns: Vec<String> = ["r", "wr", "b", "g", "bwu", "rb", "gb", "br"].iter().map(|p| p.to_string()).collect();
   
    assert!(!is_design_possible(design, &patterns, 0))
  }

  #[test]
  fn example_input_returns_6_possible_desings() { 
    let (patterns, designs) = parse_input(&TEST_CONFIG);

    assert_eq!(possible_designs(designs, &patterns), 6);
  }
}