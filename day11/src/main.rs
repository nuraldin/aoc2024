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
use std::{collections::HashMap, thread};

use utils::{get_challenge_config, read_puzzle_input, ChallengePart};

fn main() {
    let challenge_config = get_challenge_config();
    
    let stones = get_stones(challenge_config.is_test);
    // println!("Stone arrangement: {:?}", stones);

    match challenge_config.part {
      ChallengePart::One => println!("After blinking {} times the amount of stones is: {}", 25, after_n_blinks(25, stones)),
      ChallengePart::Two => println!("After blinking {} times the amount of stones is: {}", 75, after_n_blinks_map(75, stones)),
    }
}

fn get_stones(is_test: bool) -> Vec<usize> {
  let mut stones = Vec::new();

  let file_path = if is_test { "./src/example_input.txt" } else { "./src/puzzle_input.txt" };

  for line in read_puzzle_input(file_path) {
    let arrangement: Vec<&str> = line.split(' ').collect(); 

    for stone in arrangement {
      stones.push(stone.parse().unwrap())
    }
  }

  stones
}

fn after_n_blinks(blinks: i32, stones: Vec<usize>) -> usize {
  let num_threads = 8; // split the blinking operation in threads.
  let stones_chunk_size = (stones.len() / num_threads).max(1);
  let stones_chunks: Vec<Vec<usize>> = stones
    .chunks(stones_chunk_size)
    .map(|chunk| chunk.to_vec())
    .collect();

  let mut handles = vec![];

  for stones_chunk in stones_chunks {
    let handle = thread::spawn(move || {
      let mut stones = stones_chunk;
      for _ in 0..blinks {
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

fn blink(stones: Vec<usize>)  -> Vec<usize> {
  let mut next_stones = Vec::new();
  
  for stone in stones {
    if stone == 0 {
      next_stones.push(1);
    } else if stone.to_string().len() % 2 == 0 { 
      let digits = 10u64.pow((stone as f64).log10().ceil() as u32 / 2); // Determine the split position
      let left_value = stone / digits as usize; // Extract higher digits
      let right_value = stone % digits as usize; 
    
      next_stones.push(left_value);
      next_stones.push(right_value);
    } else {
      next_stones.push(stone * 2024)
    }

  }

  next_stones
}

fn blink_simple(stones: HashMap<usize, usize>)  -> HashMap<usize, usize> {
  let mut next_stones = HashMap::new();

  for (stone, amount) in stones {
    let num = format!("{}", stone);
    if stone == 0 {
      next_stones.entry(1).and_modify(|value| *value += amount).or_insert(amount);
    } else if num.len() % 2 == 0 { 
      let left_value = num[..num.len() / 2].parse().unwrap(); // Extract higher digits
      let right_value = num[num.len() / 2..].parse().unwrap(); 
    
      next_stones.entry(left_value).and_modify(|value| *value +=  amount).or_insert(amount);
      next_stones.entry(right_value).and_modify(|value| *value +=  amount).or_insert(amount);

    } else {
      next_stones.entry(stone * 2024).and_modify(|value| *value += amount).or_insert(amount);
    }
  }
  
  next_stones
}

fn after_n_blinks_map(blinks: i32, stones: Vec<usize>) -> usize {
  let mut blink_map: HashMap<usize, usize> = HashMap::new();

  for stone in stones {
    blink_map.entry(stone).or_insert(1);
  }

  for n_blinks in 0..blinks {
    blink_map = blink_simple(blink_map);
  }

  blink_map.values().sum()  
}

