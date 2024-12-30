/*
 Advent of Code 2024 Day 1: Historian Hysteria

 Part one:

 Calculate the distances of the location lists.
 That is pairing the smallest number of one list to the smallest of the other and then next smallest and so forth.
 Then calculate the distance between both numbers e.g. 9 and 3 is 6, whatever the order.
 Then add up all distances together and voila.

 Part two:

 Calculate how often each number from the left list appears in the right list.
 It is needed to calculate the similarity score which tis the number of the left column times the times it appears on the right.
 All added together for each element of the left list.
*/
use utils::{ChallengeConfig, ChallengePart};

fn main() {
    let challenge_config = ChallengeConfig::get();

    let (mut column1, mut column2) = parse_puzzle_input(&challenge_config);

    match challenge_config.part {
      ChallengePart::One => println!("Locations distance: {:?}", calculate_distance(&mut column1, &mut column2)),
      ChallengePart::Two => println!("Similarity score: {:?}", calculate_similarity_score(&column1, &column2)),
    }
}

fn parse_puzzle_input(config: &ChallengeConfig) -> (Vec<i32>, Vec<i32>) {
  let mut column1: Vec<i32> = Vec::new();
  let mut column2: Vec<i32> = Vec::new();

  for line in config.read_puzzle_input(None) {
      let columns: Vec<&str> = line.split_whitespace().collect();

      if columns.len() >= 2 {
          let col1: i32 = columns[0].parse().expect("One element of the input is corrupted");
          let col2: i32 = columns[1].parse().expect("one element of the input is corrupted");

          column1.push(col1);
          column2.push(col2);
      }
  }

  (column1, column2)
}

fn calculate_distance(column1: &mut Vec<i32>, column2: &mut Vec<i32>) -> i32 {
    // Sort the lists
    column1.sort();
    column2.sort();

    let mut results: Vec<i32> = Vec::new();

    for i in 0..column1.len() {
        results.push((column1[i] - column2[i]).abs());
    }

    // println!("Results: {:?}", results); // Output: [3, 3, 3, 4, 5, 9]

    results.iter().sum::<i32>()
}

// To resolve part two
fn calculate_similarity_score(column1: &Vec<i32>, column2: &Vec<i32>) -> i32 {
    let mut results: Vec<i32> = Vec::new();

    for i in 0..column1.len() {
        let count: i32 = column2
            .iter()
            .filter(|&&x| x == column1[i])
            .count()
            .try_into()
            .unwrap();

        results.push(count * column1[i]);
    }

    // println!("Results: {:?}", results); // Output: [3, 3, 3, 4, 5, 9]

    results.iter().sum::<i32>()
}
