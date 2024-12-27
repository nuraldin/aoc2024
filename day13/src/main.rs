/*
 Advent of Code 2024 Day 13: Claw Contraption

 There is a claw machine that has only two buttons: `A` and `B`
 It costs 3 tokens to push the A button and 1 token to push the B button.
 Each machine's buttones have a specific configurtion to move the claw to the right (X axis) or forward (Y axis) a specific amount each time a button is pressed.
 Each machine containes one prize. to win the prize the claw must be exactly aboeve the prize on both X and Y axes.
 No button needs to be pressed more than 100 times to win a prize.
 There could be combinations that are not possible.

 Part one:

 What is the fewest tokens you would have to spend to win all possible prizes?

 Part two:

 add to the prize locations 10_000_000_000_000 and calculate the tokens again

 Solution:

 For this challenge I used linear programming to solve for one of the tokens variables. Even though I started trying to brute force by checking each button a and b token combination that satisfied the equation.
 I then restorted to the math approach for part tow as it wouldn't have scaled previous algorithm.
*/
use utils::{ChallengeConfig, read_puzzle_input, ChallengePart};
use regex::Regex;

fn main() {
    let challenge_config = ChallengeConfig::get(();
    
    let parsed_input = parse_input(challenge_config.is_test);

    match challenge_config.part {
      ChallengePart::One => println!("The minimum tokens needed to get any of the prizes is: {}", calculate_minimum_tokens(parsed_input, 0)),
      ChallengePart::Two => println!("The minimum tokens needed to get any of the prizes is: {}", calculate_minimum_tokens(parsed_input, 10_000_000_000_000)),
    }
}

#[derive(Debug)]
struct Location {
  x: i64,
  y: i64,
}

#[derive(Debug)]
struct ButtonConfig {
  a: Location,
  b: Location,
} // The button configuratioon can be stored similar to a location but have different meaning.

#[derive(Debug)]
struct ClawMachineConfig {
  prize: Location,
  button_config: ButtonConfig,
}

fn parse_input(is_test: bool) -> Vec<ClawMachineConfig> {
  let mut claw_machine_configs = Vec::new();
  
  let file_path = if is_test { "./src/example_input.txt" } else { "./src/puzzle_input.txt" };

  let button_a_pattern = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)").unwrap();
  let button_b_pattern = Regex::new(r"Button B: X\+(\d+), Y\+(\d+)").unwrap();
  let prize_pattern = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

  let mut lines = Vec::new();
  for line in read_puzzle_input(file_path) {
    if line.starts_with("Prize") {
      lines.push(line);
      let claw_machine_config_string: String = lines.join(" ");
      let button_a = button_a_pattern.captures(&claw_machine_config_string).unwrap();
      let button_b = button_b_pattern.captures(&claw_machine_config_string).unwrap();
      let prize = prize_pattern.captures(&claw_machine_config_string).unwrap();

      claw_machine_configs.push(ClawMachineConfig {
        prize: Location {
          x: prize.get(1).unwrap().as_str().parse().unwrap(),
          y: prize.get(2).unwrap().as_str().parse().unwrap(),
        },
        button_config: ButtonConfig {
          a:  Location {
            x: button_a.get(1).unwrap().as_str().parse().unwrap(),
            y: button_a.get(2).unwrap().as_str().parse().unwrap(),
          },
          b:  Location {
            x: button_b.get(1).unwrap().as_str().parse().unwrap(),
            y: button_b.get(2).unwrap().as_str().parse().unwrap(),
          },
        }
      });

      lines = Vec::new();
    } else {
      lines.push(line);
    }

  }

  claw_machine_configs
}

fn calculate_minimum_tokens(configs: Vec<ClawMachineConfig>, correction: i64) -> i64 {
  let mut minimum_tokens_needed = 0;
  for config in configs {
    minimum_tokens_needed += get_tokens(config, correction);
  }

  minimum_tokens_needed
}

fn get_tokens(config: ClawMachineConfig, correction: i64) -> i64 {
  let w = config.prize.x + correction;
  let y = config.prize.y + correction;
  let a = config.button_config.a.x;
  let b = config.button_config.b.x;
  let c = config.button_config.a.y;
  let d = config.button_config.b.y;

  let m = (c * w - a * y) / ( c * b - a * d);
  let n = (y - m * d) / c;

  // println!("m: {m}, n: {n}");

  let result_x = n * a + m * b;
  let result_y = n * c + m * d;

  // println!("X={w}, Y={y}");
  // println!("X={result_x},Y={result_y}");

  if result_x == w && result_y == y {
    return n * 3 + m;
  }

  0
}