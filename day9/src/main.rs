/*
 Advent of Code 2024 Day 9

 Part one:

 The input is a `disk map`, i.e. a series of characters in on line.
 These represent the layout of `files` and `free space`.
 Every file has an ID based on the order of the appearance before being rearranged.
 The disk must be first compacted by filling the blank spaces with the files from the end to the beginning until they are all filled.
 The result is to calculate the checksum which is the position (starting at 0) times the ID it has.

 Part two:

 similar to part one but now the files should be moved completely to the nearest free slot otherwise they are not moved
*/
use utils::{ChallengeConfig, ChallengePart};

fn main() {
    let challenge_config = ChallengeConfig::get();
    
    let puzzle = parse_input(&challenge_config);

    match challenge_config.part {
      ChallengePart::One => println!("filesystem checksum: {:?}", filesystem_checksum(puzzle)),
      ChallengePart::Two => println!("filesystem checksum improved: {:?}", filesystem_checksum_improved(puzzle)),
    }
}

fn parse_input(config: &ChallengeConfig) -> String {
  let mut disk_map = String::new();
  
  for line in config.read_puzzle_input(None) {
    disk_map = line;
  }

  disk_map
}

fn filesystem_checksum(disk_map: String) -> i64 {
  println!("disk map: {disk_map}");

  let expanded_disk_map = expand_disk_map(disk_map);
  println!("disk map expanded: {:?}, disk length: {}", expanded_disk_map, expanded_disk_map.len());

  let compacted_disk = compact_disk(expanded_disk_map);
  println!("compacted disk:    {:?}", compacted_disk);
  
  let mut checksum: i64 = 0;
  for (idx, item) in compacted_disk.iter().enumerate() {
    if *item > 0 {
      let partial: i64 = idx as i64 * item;
      checksum += partial;
    }
  }

  checksum
} 

fn expand_disk_map(disk_map: String) -> Vec<i64> {
  let mut disk = Vec::new();

  let mut file_id = 0;
  let mut file_toggle = true;
  for item in disk_map.chars() {
    let amount = item.to_digit(10).expect("Couldn't transform character to digit");

    if file_toggle  {
      for _ in 0..amount {
        disk.push(file_id)
      }
      file_id += 1;
      file_toggle = false;
    } else {
      for _ in 0..amount {
        disk.push(-1);
      }
      file_toggle = true; 
    }

  }

  disk
}

fn compact_disk(mut expanded_disk_map: Vec<i64>) -> Vec<i64> {
  let mut next_item_idx = expanded_disk_map.iter().rposition(|item| *item > 0).expect("Didn't find a file"); // For getting from the last position
  for disk_idx in 0..expanded_disk_map.len() {
    let current_item = &expanded_disk_map[disk_idx];
    if *current_item >= 0|| next_item_idx < disk_idx {
      continue;
    }

    expanded_disk_map.swap(disk_idx, next_item_idx);
    // let partial: String = expanded_disk_map.iter().collect();
    // println!("Swapped elements:  {partial}");

    next_item_idx = expanded_disk_map.iter().rposition(|item| *item > 0).expect("Didn't a file"); // For getting from the last position
  }

  expanded_disk_map
}

fn filesystem_checksum_improved(disk_map: String) -> i64 {
  // println!("disk map: {disk_map}");

  let parsed_disk_map = parse_disk_map(disk_map);
  // println!("parsed disk map expanded: {:?}, disk length: {}", parsed_disk_map, parsed_disk_map.len());

  let rearranged_disk_map = rearrange_disk_map(parsed_disk_map);

  let transformed_disk_map = transform_disk_map(rearranged_disk_map);
  // println!("transformed disk map: {:?}",  transformed_disk_map);

  let mut checksum: i64 = 0;
  for (idx, item) in transformed_disk_map.iter().enumerate() {
    if *item > 0 {
      let partial: i64 = idx as i64 * item;
      checksum += partial;
    }
  }

  checksum  
}

#[derive(Debug, Clone)]
struct FileInfo {
  amount: u32,
  file_id: i32,
}

fn parse_disk_map(disk_map: String) -> Vec<FileInfo> {
  let mut parsed_disk_map = Vec::new();

  let mut file_id = 0;
  let mut file_toggle = true;
  for item in disk_map.chars() {
    let amount = item.to_digit(10).expect("Couldn't transform character to digit");

    if file_toggle  {
      parsed_disk_map.push(FileInfo { amount, file_id });
      file_id += 1;
      file_toggle = false;
    } else {
      parsed_disk_map.push(FileInfo { amount, file_id: -1 });
      file_toggle = true; 
    }

  }

  parsed_disk_map
}

fn transform_disk_map(parsed_disk_map: Vec<FileInfo>) -> Vec<i64> {
  let mut disk_map = Vec::new();

  for file_info in parsed_disk_map {
    for _ in 0..file_info.amount {
      disk_map.push(file_info.file_id as i64);
    }
  }

  disk_map
}

fn rearrange_disk_map(mut parsed_disk_map: Vec<FileInfo>) -> Vec<FileInfo> {
  let mut next_item_idx = parsed_disk_map
                                .iter()
                                .rposition(|item| item.file_id >= 0) // a valid file id
                                .expect("Didn't find a file");

  let mut next_item = parsed_disk_map[next_item_idx].clone();          
  let mut current_file_id = next_item.file_id;

  loop {
    // println!("next_item_idx: {}, next_item: {:?}, current_file_id: {}", next_item_idx, next_item, current_file_id);

    // get next valid file from the right
    if next_item.file_id == 0 {
      break;
    }

    let next_possible_place_idx: i32 = match parsed_disk_map
                                  .iter().position(|item| item.file_id < 0 && item.amount >= next_item.amount) {
                                    Some(value) => value as i32,
                                    None => -1
                                  };

    // println!("next_possible_place_idx: {}", next_possible_place_idx);
    if next_possible_place_idx < 0 || next_possible_place_idx > next_item_idx as i32 {
      current_file_id -= 1;
      next_item_idx = parsed_disk_map
                                .iter()
                                .rposition(|item| item.file_id == current_file_id) // a valid file id
                                .expect("Didn't find a file");
      next_item = parsed_disk_map[next_item_idx].clone();
      continue; // if no space, skip number
    } 

    let possible_item = parsed_disk_map[next_possible_place_idx as usize].clone();
    
    if possible_item.amount == next_item.amount {
      parsed_disk_map.swap(next_possible_place_idx as usize, next_item_idx);
    } {
      parsed_disk_map[next_possible_place_idx as usize].amount -= next_item.amount;
      parsed_disk_map[next_item_idx].file_id = -1;
      parsed_disk_map.insert(next_possible_place_idx as usize, next_item);
    }

    current_file_id -= 1;
    next_item_idx = parsed_disk_map
                              .iter()
                              .rposition(|item| item.file_id == current_file_id) // a valid file id
                              .expect("Didn't find a file");
    next_item = parsed_disk_map[next_item_idx].clone();
  
  }

  parsed_disk_map
}