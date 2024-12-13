use std::u32::MAX;

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
*/
use utils::{get_challenge_config, read_puzzle_input, ChallengePart};
use regex::Regex;

fn main() {
    let challenge_config = get_challenge_config();
    
    let parsed_input = parse_input(challenge_config.is_test);

    match challenge_config.part {
      ChallengePart::One => println!("The minimum tokens needed to get any of the prizes is: {}", calculate_minimum_tokens(parsed_input)),
      ChallengePart::Two => println!("The minimum tokens needed to get any of the prizes is: {}", calculate_minimum_tokens(parsed_input)),
    }
}

#[derive(Debug)]
struct Location {
  x: u32,
  y: u32,
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

fn calculate_minimum_tokens(configs: Vec<ClawMachineConfig>) -> u32 {
  let mut minimum_tokens_needed = 0;
  for config in configs {
    minimum_tokens_needed += get_tokens(config);
  }

  minimum_tokens_needed
}

fn get_tokens(config: ClawMachineConfig) -> u32 {
  let mut valid_combinations: Vec<(u32, u32)> = Vec::new();

  for n in 0..100 {
    for m in 0..100 {
      if satisfies_calculation(&config, n, m) {
        valid_combinations.push((n,m))
      }
    }
  }
  println!("config: {config:?}, \nvalid_combinations: {valid_combinations:#?}\n");

  let mut minimum_tokens_needed = if valid_combinations.len() > 0 { MAX } else { 0 };
  for (a_presses, b_presses)in valid_combinations {
    let tokens = a_presses * 3 + b_presses;
    minimum_tokens_needed = minimum_tokens_needed.min(tokens);
  }

  println!("minimum tokens needed: {minimum_tokens_needed}\n");

  minimum_tokens_needed
}

fn satisfies_calculation(config: &ClawMachineConfig, n: u32, m: u32) -> bool {
  config.prize.x == config.button_config.a.x * n + config.button_config.b.x * m &&
  config.prize.y == config.button_config.a.y * n + config.button_config.b.y * m
}