/*
 Advent of Code 2024 Day 7

 Part one:

 The input has a number and its factors. The operators to calculate it are missing.
 There is only + and *. 
 It is needed the totatl calibration result from the values of the results that could be possibly true. 

 Part two:

*/
use std::collections::HashMap;
use utils::{get_challenge_part, read_puzzle_input, ChallengePart};

fn main() {
  let puzzle = parse_input();

  println!("Puzzle shape: {:?}", puzzle);

  match get_challenge_part() {
    ChallengePart::One => println!("Total calibraion result: {:?}", total_calibration_result(puzzle)),
    ChallengePart::Two => println!("Not implemented yet"),
  }
}

fn parse_input() -> HashMap<u64, Vec<u64>> {
  let mut equations_map = HashMap::new();  
  
  for line in read_puzzle_input("./src/puzzle_input.txt") {
    let result: Vec<&str> = line.split(':').collect();

    equations_map.insert(result[0].parse().unwrap(), result[1].trim().split(' ').map(|item|  item.parse().unwrap()).collect());
  }

  equations_map
}

fn total_calibration_result(puzzle: HashMap<u64, Vec<u64>>) -> u64 {
  let mut possible_values = Vec::new();

  for (result, numbers) in puzzle.iter() {
    if exists_combination(*result, numbers.clone()) {
      possible_values.push(*result);
    }
  }

  println!("Possible values: {:?}", possible_values);

  possible_values.iter().sum()
}

fn exists_combination(result: u64, numbers: Vec<u64>) -> bool {
  let mut exists = false;
  let operators_combinations = generate_combinations(numbers.len() - 1); 

  for operators_combination in operators_combinations {
    let mut acum = numbers[0];
    let operators: Vec<char> = operators_combination.chars().collect();
    
    
    for number_idx in 1..numbers.len() {
      println!("{} {} acum: {}", operators[number_idx - 1],  numbers[number_idx] , acum);
      acum = match operators[number_idx - 1] {
        '+' => acum + numbers[number_idx],
        '*' => acum * numbers[number_idx],
        _ => panic!("This shouldn't happen") 
      }
    }
 
    if acum == result {
      exists = true;
      break;
    }

  }
 
  exists
}

fn generate_combinations(n: usize) -> Vec<String> {
  let mut combinations = Vec::new();
  let total_combinations = 1 << n; // 2^n combinations

  for i in 0..total_combinations {
      let mut combination = String::new();

      for bit in (0..n).rev() {
          if (i & (1 << bit)) != 0 {
              combination.push('*');
          } else {
              combination.push('+');
          }
      }

      combinations.push(combination);
  }
  
  println!("combinations {:?}", combinations); 

  combinations
}