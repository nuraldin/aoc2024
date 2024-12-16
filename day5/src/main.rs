/*
 Advent of Code 2024 Day 5: Print Queue

 The puzzle is a set of rules for constructing sleigh launch safety manual .
 The new pages for the safety manuals must be printed in very specific order.
 The notation X | Y means that X must be printed at some point before page Y
 Every line of pages is called an update. That update must be in the order specified by the rules.

 Part one:

 Determine the updates that are in the correct order.
 What do I get if I add up all the middle page numbers of those correctly-ordered updates?

 Part two:

 Determine the updates that are incorrectly ordered.
 Use the ordering rules to rearrange them.
 Add up the middle page numberes similar to previous part.

 Solution: 
*/
use std::collections::HashMap;
use utils::{get_challenge_config, read_puzzle_input, ChallengePart};

fn main() {
  let challenge_config = get_challenge_config();

  let (page_ordering_rules, page_updates_list) = parse_input(challenge_config.is_test);

  match challenge_config.part {
    ChallengePart::One => {
      let mut correct_ordering_updates = Vec::new();
      for page_update in page_updates_list {
        if check_rules(&page_ordering_rules, &page_update) {
          correct_ordering_updates.push(page_update)
        }
      }

      let middle_page_sum = correct_ordering_updates
        .iter()
        .fold(0, |accum, correct_update| accum + correct_update[correct_update.len() / 2]);

      println!("Correct page update middle page sum: {}", middle_page_sum);
    },
    ChallengePart::Two => {
      let mut corrected_incorrect_ordering_updates = Vec::new();
      for page_update in page_updates_list {
        if !check_rules(&page_ordering_rules, &page_update) {
          corrected_incorrect_ordering_updates.push(correct_incorrect_update(page_update, page_ordering_rules.clone()));
        }
      }

      let middle_page_sum = corrected_incorrect_ordering_updates
      .iter()
      .fold(0, |accum, incorrect_update| accum + incorrect_update[incorrect_update.len() / 2]);
    
      println!("Corrected incorrect page update middle page sum: {}", middle_page_sum);
    },
  }
}

fn parse_input(is_test: bool) -> (HashMap<u32, Vec<u32>>, Vec<Vec<u32>>) {
  let mut page_rules = HashMap::new();
  let mut page_updates_list: Vec<Vec<u32>> = Vec::new();
  
  let (updates_file_path, rules_file_path) = if is_test {
    ("./src/example_pages.txt", "./src/example_page_ordering_rules.txt")
  } else {
    ("./src/puzzle_pages.txt", "./src/puzzle_page_ordering_rules.txt")
  };

  for line in read_puzzle_input(updates_file_path) {
    let pages_line: Vec<u32> = line.split(",").map(|s| s.parse().unwrap()).collect();
    page_updates_list.push(pages_line)
  }

  for line in read_puzzle_input(rules_file_path) {
    let page_rules_line: Vec<u32> = line.split("|").map(|s| s.parse().unwrap()).collect();
    
      // Add an element to the array corresponding to a key
    page_rules.entry(page_rules_line[0])
      .or_insert(Vec::new()) // Insert an empty vector if the key doesn't exist
      .push(page_rules_line[1]);            // Append 42 to the array
  }

  (page_rules, page_updates_list)
}

fn check_rules(page_ordering_rules: &HashMap<u32, Vec<u32>>, page_update: &Vec<u32>) -> bool {
  let mut pass = true;

  for (page_idx, page) in page_update.iter().enumerate() {
    if let Some(ordering_rule) = page_ordering_rules.get(page) {
      for other_page in ordering_rule {
        if let Some(other_page_index) = page_update.iter().position(|&page| page == *other_page) {
          if other_page_index < page_idx {
            pass = false;
          }
        }
      }
    }

    if !pass {
      break;
    }
  }

  pass 
}

fn correct_incorrect_update(incorrect_page_update: Vec<u32>, page_rules: HashMap<u32, Vec<u32>>) -> Vec<u32> {
  let mut corrected_page_update = incorrect_page_update.clone();
  let default_vector: Vec<u32> = Vec::new();

  let mut corrected_page_idx = 0;
  loop {
    if corrected_page_idx >= corrected_page_update.len() {
      break;
    } 
    let incorrect_page = corrected_page_update[corrected_page_idx];

    let rules = page_rules.get(&incorrect_page).unwrap_or_else(|| &default_vector);
    let mut rules_idx = 0;
    loop {
      let mut swap_flag = false; 

      if rules_idx >= rules.len() {
        corrected_page_idx += 1;
        // println!("Finished checking page: {}, corrected state: {:?}", incorrect_page, corrected_page_update);
        break;
      }

      let rule = rules[rules_idx];
      if incorrect_page_update.contains(&rule) {
        let rule_idx = corrected_page_update.iter().position(|&page| page == rule).unwrap();
        let page_idx = corrected_page_update.iter().position(|&page| page == incorrect_page_update[corrected_page_idx]).unwrap();

        if rule_idx < page_idx {
          corrected_page_update.swap(rule_idx,page_idx);
          swap_flag = true;
          // println!("Had to correct the page update, new state: {:?}", corrected_page_update);
        }
      }

      if !swap_flag {
        rules_idx += 1;
      }
    }

    if check_rules(&page_rules, &corrected_page_update) {
      break;
    }
  }

  println!("incorrect page update: {:?}, corrected page update: {:?}, rules: {:?}", incorrect_page_update, corrected_page_update, page_rules);

  corrected_page_update
}