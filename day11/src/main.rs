/*
 Advent of Code 2024 Day 11 - Plutonian Pebbles

 The input is a an arrangement of `stones` arranged in a straigt line.
 Each stone has a number engraved on it. Every time I blink, the stones change.
 The number engraved might change or be split in two.
 The rules are:
    - If the number is 0 it is replaced by the number 1
    - if the number has an even number of digits, it is replaced by two stones. The left half of digits on the new left stones, the right half on the right stone (without leading zeroes).
    - If no rule applies, the number is replaced by its number multiplied by 2024.

 The order is always preserved.

 Part one:

 How many stones will I have after blinking 25 times?

 Part two:

 How many stones will there be a after blinking 75 times?

*/
use std::thread;

use utils::{get_challenge_config, read_puzzle_input, ChallengePart};

fn main() {
    let challenge_config = get_challenge_config();
    
    let stones = get_stones(challenge_config.is_test);
    // println!("Stone arrangement: {:?}", stones);

    match challenge_config.part {
      ChallengePart::One => println!("After blinking {} times the amount of stones is: {}", 25, after_n_blinks(25, stones)),
      ChallengePart::Two => println!("After blinking {} times the amount of stones is: {}", 75, after_n_blinks(75, stones)),
    }
}

#[derive(Debug, Clone)]
struct Stone {
  engravement: String,
  value: u64,
}

fn after_n_blinks(blinks: i32, stones: Vec<Stone>) -> usize {
  let num_threads = 8; // split the blinking operation in threads.
  let stones_chunk_size = (stones.len() / num_threads).max(1);
  let stones_chunks: Vec<Vec<Stone>> = stones
    .chunks(stones_chunk_size)
    .map(|chunk| chunk.to_vec())
    .collect();

  let mut handles = vec![];

  for stones_chunk in stones_chunks {
    let handle = thread::spawn(move || {
      let mut stones = stones_chunk;
      for n_blink in 0..blinks {
        stones = blink(stones);
        // println!("Stone arrangement after {} blinks: {:?}", n_blink + 1, stones.iter().map(|stone| format!("{} ", stone.engravement)).collect::<String>());
      }
      stones
    });

    handles.push(handle)
  }

  let mut stones = 0;
  for handle in handles {
    let chunk_result = handle.join().unwrap();
    stones += chunk_result.len();
    // println!("stones chunk: {:?}", chunk_result);
  }
  
  stones
}

fn get_stones(is_test: bool) -> Vec<Stone> {
  let mut stones = Vec::new();

  let file_path = if is_test { "./src/example_input.txt" } else { "./src/puzzle_input.txt" };

  for line in read_puzzle_input(file_path) {
    let arrangement: Vec<&str> = line.split(' ').collect(); 

    for stone in arrangement {
      stones.push(Stone {
        engravement: stone.to_string(),
        value: stone.parse().unwrap()
      })
    }
  }

  stones
}

fn blink(stones: Vec<Stone>)  -> Vec<Stone> {
  let mut next_stones = Vec::new();
  
  for stone in stones {
    if stone.value == 0 {
      next_stones.push(Stone {
        engravement: String::from('1'),
        value: 1,
      })
    } else if stone.engravement.len() % 2 == 0 { 
      let (left, right) = stone.engravement.split_at(stone.engravement.len() / 2);
      // println!("left split: {left}, right split: {right}");

      let left_value: u64 = left.parse().unwrap();
      next_stones.push(Stone {
        engravement: left_value.to_string(),
        value: left_value,
      });

      let right_value: u64 = right.parse().unwrap();
      next_stones.push(Stone {
        engravement: right_value.to_string(),
        value: right_value
      });
    } else {
      let next_value = stone.value * 2024;
      next_stones.push(Stone {
        engravement: next_value.to_string(),
        value: next_value,
      })
    }

  }
  
  next_stones
}