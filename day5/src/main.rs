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

 At the beginning I went with a hashmap and a simple hashmap checking for rules.
 I then found this answer https://www.youtube.com/watch?v=TymMkbH8e6A to solver part two which changed completely my point ofview of the problem.
 If any number is not in the after set, i.e. that number should be after another one, it can be placed in one index.
 After applying that algorithm it worked excellently and refactored the whole solutions. 
*/
use std::{collections::{HashMap,HashSet}, vec};
use utils::{get_challenge_config, read_puzzle_input, ChallengePart};

fn main() {
  let challenge_config = get_challenge_config();

  let (page_ordering_rules, page_updates_list) = parse_input(challenge_config.is_test);

  let mut page_updates: Vec<Vec<u32>> = Vec::new();
  match challenge_config.part {
    ChallengePart::One => {
      for page_update in page_updates_list {
        if check_rules(&page_ordering_rules, &page_update) {
          page_updates.push(page_update)
        }
      }
    },
    ChallengePart::Two => {
      for page_update in page_updates_list {
        if !check_rules(&page_ordering_rules, &page_update) {
          page_updates.push(correct_incorrect_update(&mut page_update.clone(), &page_ordering_rules));
        }
      }
    },
  }

  let middle_page_sum = page_updates
  .iter()
  .fold(0, |accum, update| accum + update[update.len() / 2]);

  println!("Correct page update middle page sum: {}", middle_page_sum);
}

fn check_rules(page_ordering_rules: &HashMap<u32, OrderingSets>, page_update: &Vec<u32>) -> bool {
  for (page_idx, page) in page_update.iter().enumerate() {
    let ordering_sets = page_ordering_rules.get(page).unwrap();

    for other_page_idx in page_idx+1..page_update.len() {
      let other_page = page_update[other_page_idx];

      if ordering_sets.before.contains(&other_page) {
        return false
      }
    }
  }

  true
}

#[derive(Debug)]
struct OrderingSets {
  before: HashSet<u32>,
  after: HashSet<u32>
} 

fn parse_input(is_test: bool) -> (HashMap<u32, OrderingSets>, Vec<Vec<u32>>) {
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
    
    let before_page = page_rules_line[0];
    let after_page = page_rules_line[1];

    let before_page_rules = page_rules
      .entry(before_page)
      .or_insert(OrderingSets { before: HashSet::new(), after: HashSet::new()});
    before_page_rules.after.insert(after_page);

    let after_page_rules = page_rules.entry(after_page)
      .or_insert(OrderingSets { before: HashSet::new(), after: HashSet::new()});
    after_page_rules.before.insert(before_page);
  }

  (page_rules, page_updates_list)
}

fn correct_incorrect_update(page_update: &Vec<u32>, page_ordering_rules_sets: &HashMap<u32, OrderingSets>) -> Vec<u32> {
  let mut corrected_update = vec![];

  let mut page_set = HashSet::new();
  for page in page_update {
    page_set.insert(page);
  }

  while page_set.len() > 0 {
    for page in page_set.clone() {
      let mut placeable = true;

      for other_page in page_set.clone() {
        if page == other_page {
          continue;
        }

        if page_ordering_rules_sets.get(other_page).unwrap().after.contains(page) {
          placeable = false;
        }
      }

      if placeable {
        corrected_update.push(*page);
        page_set.remove(page);
        break;
      }
    }
  } 

  // println!("update: {page_update:?}, corrected update: {corrected_update:?}");
  corrected_update
}