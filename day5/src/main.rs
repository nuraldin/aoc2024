/*
 Advent of Code 2024 Day 5 

 Part one:

 We have pages of a safety manual. It has page ordering rules and pages to produce the update.
 The first section is a pattern X|Y that means that X must be produced before page number Y, not necessarily immediately before.
 The second section are the page numbers for each update.
 Which updates^re already in the right order?
 The solution is the middle page of each pages number list added up together.

 Part two:

*/
use std::collections::HashMap;

use utils::read_puzzle_input;

fn main() {
  let page_updates_list = parse_page_updates_list();
  let page_rules = parse_page_rules();

  // Uncomment for having the part one results
  // let mut correct_ordering_updates = Vec::new();
  // for page_update in page_updates_list {
  //   if check_rules(page_rules.clone(), page_update.clone()) {
  //     correct_ordering_updates.push(page_update)
  //   }
  // }


  // let mut middle_page_sum = 0;
  // for correct_update in correct_ordering_updates {
  //   middle_page_sum += correct_update[correct_update.len() / 2];
  // }

  // println!("Correct page update middle page sum: {}", middle_page_sum);

  let mut corrected_incorrect_ordering_updates = Vec::new();
  for page_update in page_updates_list {
    if !check_rules(page_rules.clone(), page_update.clone()) {
      corrected_incorrect_ordering_updates.push(correct_incorrect_update(page_update, page_rules.clone()));
    }
  }

  let mut middle_page_sum = 0;
  for incorrect_update in corrected_incorrect_ordering_updates {
    middle_page_sum += incorrect_update[incorrect_update.len() / 2];
  }

  println!("Corrected incorrect page update middle page sum: {}", middle_page_sum);
}

fn parse_page_updates_list() -> Vec<Vec<u32>> {
  let mut pages_list: Vec<Vec<u32>> = Vec::new();

  for line in read_puzzle_input("./src/puzzle_pages_list.txt") {
    let pages_line: Vec<u32> = line.expect("Couldn't parse line").split(",").map(|s| s.parse().unwrap()).collect();
    pages_list.push(pages_line)
  }
  
  pages_list
}

fn parse_page_rules() -> HashMap<u32, Vec<u32>> {
  let mut page_rules = HashMap::new();

  for line in read_puzzle_input("./src/puzzle_page_ordering_rules.txt") {
    let line = line.expect("Couldn't parse line");
    let page_rules_line: Vec<u32> = line.split("|").map(|s| s.parse().unwrap()).collect();
    
      // Add an element to the array corresponding to a key
    page_rules.entry(page_rules_line[0])
      .or_insert(Vec::new()) // Insert an empty vector if the key doesn't exist
      .push(page_rules_line[1]);            // Append 42 to the array
  }

  // println!("page rules: {:?}", page_rules);

  page_rules
}

fn check_rules(page_rules: HashMap<u32, Vec<u32>>, page_update: Vec<u32>) -> bool {
  let mut pass = true;
  for (page_idx, page) in page_update.iter().enumerate() {
    let default_vector: Vec<u32> = Vec::new();
    for rule in page_rules.get(page).unwrap_or_else(|| &default_vector) {
      if page_update.contains(rule) {
        let rule_idx = page_update.iter().position(|&page| page == *rule).unwrap();
        if rule_idx < page_idx {
          pass = false;
        }
      }

      if !pass {
        break;
      }
    }

    if !pass {
      break;
    }
  }

  pass
}

fn correct_incorrect_update(incorrect_page_update: Vec<u32>, page_rules: HashMap<u32, Vec<u32>>) -> Vec<u32> {
  incorrect_page_update
}