/*
 Advent of Code 2024 Day 7: Bridge Repair

 The operators for some calculations have been stolen.
 The puzzle input is the result and the values for the calculations. e.g. 190 (result): 10 19 (values)
 Each line is a single equation. The operators are always evaluated left to right.
 Numbers cannot be rearranged and there are only two different operators (+ sum and * multiply)
 The calibration result is the sum of the test values from the equations that could be true.

 Part one:

 What is the total calibration result for the puzzle input?

 Part two:

 There is now a third type of operator. The concatenation ||.
 This will combine digits from its left and right into a single numbers.
 What is the total calibration result adding the possibility of this other operator?

 Solution: 

 The solutions for both parts are pretty similar, the difference is the added operator. That makes the solution I have make in part two take much time. i.e. it doesn't scale.
 However, it gives the correct solution. The issue is that calculating all possible operator combination and then looping around it is inefficient.
 I will leave that solution just for historicity but found a recursive solution in internet that I tried to applied and made the part two go from 60s in my 2019 intel mac, to 6s.

*/
use std::collections::HashMap;
use utils::{ChallengeConfig, read_puzzle_input, ChallengePart};

fn main() {
  let challenge_config = ChallengeConfig::get(();

  let puzzle = parse_input(challenge_config.is_test);
  // println!("Puzzle shape: {:?}", puzzle);

  match challenge_config.part {
    ChallengePart::One => println!("Total calibration result: {:?}", total_calibration_result_recursive(puzzle, &["*", "+"])),
    ChallengePart::Two => println!("Total calibration result: {:?}", total_calibration_result_recursive(puzzle, &["*", "+", "||"])),
  }
}

fn parse_input(is_test: bool) -> HashMap<u64, Vec<u64>> {
  let mut equations_map = HashMap::new();  

  let file_path = if is_test { "./src/example_input.txt" } else { "./src/puzzle_input.txt" };
  
  for line in read_puzzle_input(file_path) {
    let result: Vec<&str> = line.split(':').collect();

    equations_map.insert(
      result[0].parse().unwrap(), 
      result[1].trim().split(' ').map(|item|  item.parse().unwrap()).collect()
    );
  }

  equations_map
}

fn total_calibration_result(mut puzzle: HashMap<u64, Vec<u64>>, operators: &[&str]) -> u64 {
  let mut possible_values = Vec::new();

  for (result, numbers) in puzzle.iter_mut() {
    // println!("result: {result}, numbers: {numbers:?}");
    if exists_combination(*result, numbers, operators) {
      possible_values.push(*result);
    }
  }

  // println!("Possible values: {:?}", possible_values);

  possible_values.iter().sum()
}

fn total_calibration_result_recursive(mut puzzle: HashMap<u64, Vec<u64>>, operators: &[&str]) -> u64 {
  let mut possible_values = Vec::new();

  for (result, numbers) in puzzle.iter_mut() {
    if exists_combination_recursive(*result,numbers, operators, 0) {
      possible_values.push(*result);
    }
  }

  // println!("Possible values: {:?}", possible_values);

  possible_values.iter().sum()
}

fn exists_combination(result: u64, numbers: &mut Vec<u64>, operators: &[&str]) -> bool {
  let mut operators_combinations = generate_combinations(numbers.len(), operators); 

  // println!("operators cominations: {operators_combinations:?}");
  for operators_combination in operators_combinations.iter_mut() {
    let mut acum = 0;
    operators_combination.insert(0, "+".to_string());
    // println!("operator combination: {operators_combination:?}");

    for (idx, number) in numbers.clone().iter().enumerate() {
      acum = match operators_combination[idx].as_str() {
        "+" => acum + *number,
        "*" => acum * *number,
        "||" => {
          let next_number = format!("{}{}", acum, number);
          next_number.parse().unwrap()
        },
        _ => panic!("This shouldn't happen") 
      };

      // println!("acum: {acum}, number: {number}, idx: {idx}");
    }
 
    if acum == result {
      return true;
    }

  }
  
  false
}

fn generate_combinations(n: usize, operators: &[&str]) -> Vec<Vec<String>> {
  let mut combinations = Vec::new();
  let total_combinations = operators.len().pow((n - 1) as u32);

  for i in 0..total_combinations {
      let mut combination = vec![];

      let res = to_base(operators.len() as u32, i as u32, n - 1);
      // println!("{res}");
      for c in res.chars() {
        let op_idx = c.to_digit(10).unwrap() % operators.len() as u32;
        let op = operators[op_idx as usize];

        combination.push(op.to_string());
      }

      combinations.push(combination);
  }
  
  // println!("combinations: {total_combinations}");
  // println!("combinations {:?}", combinations);

  combinations
}

fn to_base(base: u32, mut num: u32, digits: usize) -> String {
  let mut result = String::new();

  while num > 0 {
      result.insert(0, char::from_digit(num % base, 10).unwrap());
      num /= base;
  }

  if result.is_empty() {
      result.push('0'); // Handle 0 explicitly
  }

  let mut size: usize = result.len();
  while size < digits { 
    result.insert(0, '0');
    size += 1;
  }

  result
}

fn exists_combination_recursive(target: u64, mut numbers: &[u64], operators: &[&str], acum: u64) -> bool {
  if acum > target {
    return false;
  }

  if numbers.is_empty() {
    return target == acum;
  }

  let (next, rest ) = numbers.split_first().unwrap();

  let mut acums: Vec<u64> = Vec::new();
  for operator in operators {
    match *operator {
      "+" => acums.push(acum + next),
      "*" => acums.push(acum * next),
      "||" => acums.push(format!("{}{}", acum, next).parse().unwrap()),
      _ => unreachable!(),
    }
  }

  exists_combination_recursive(target, rest, operators, acums[0]) ||
  exists_combination_recursive(target, rest, operators, acums[1]) ||
  (*acums.get(2).unwrap_or(&u64::MAX) != u64::MAX && exists_combination_recursive(target, rest, operators, format!("{}{}", acum, next).parse().unwrap()))
}