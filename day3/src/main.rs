/*
 Advent of Code 2024 Day 3: Mull it over

 Part one:

 The memory (puzzle input) is `corrupted`
 the goal is to multiply some numbers but there are some invalid characters that should be `ignored`
 find in the text all mul(x,y) which are not corrupted, multiply their numbers and add them all together.
 
 Part two:

 Similar to the first but now there are do() and don't() instructions that enable or disables future mul instructions.
 they all start `enabled`
*/
use utils::{get_challenge_config, read_puzzle_input, ChallengePart};
use regex::Regex;

fn main() {
  let challenge_config = get_challenge_config(); 

  let memory_line = get_memory_line(challenge_config.is_test);

  match challenge_config.part {
    ChallengePart::One => println!("The non corrupted multiplications add up to: {:?}", part_one(memory_line)),
    ChallengePart::Two => println!("The non corrupted and enabled multiplications add up to: {:?}", part_two(memory_line)),
  }
}

fn get_memory_line(is_test: bool) -> String {
  let file_path = if is_test { "./src/example_input.txt" } else { "./src/puzzle_input.txt" };

  for line in read_puzzle_input(file_path) {
    return line;
  }  

  unreachable!();
}

fn part_one(line: String) -> u32 {
  let mut result = 0;
  let re = Regex::new(r"mul\((\d+),(\d*)\)").unwrap();

  for cap in re.captures_iter(line.as_str()) {
    let x: u32 =  cap.get(1).unwrap().as_str().parse().unwrap(); // First number (x)
    let y: u32 = cap.get(2).unwrap().as_str().parse().unwrap(); // Second number (y)

    result += x * y;

    // println!("Found: mul({}, {}) Res: {}", x, y, x * y);
  }

  result
}

fn part_two(line: String) -> u32 {
  let mut result = 0;
  let re = Regex::new(r"do\(\)|don't\(\)|mul\((\d+),(\d*)\)").unwrap();

  let mut enable_mul = true;
  for cap in re.captures_iter(line.as_str()) {
    let captured_element = cap.get(0).unwrap().as_str();
    
    if captured_element == "don't()" {
      enable_mul  = false;
      continue;
    } else if captured_element == "do()" {
      enable_mul = true;
      continue;
    }

    if enable_mul {
      let x: u32 =  cap.get(1).unwrap().as_str().parse().unwrap(); // First number (x)
      let y: u32 = cap.get(2).unwrap().as_str().parse().unwrap(); // Second number (y)
    
      result += x * y;
    }
  }

  result
}