/*
 Advent of Code 2024 Day 23: LAN Party

 The puzzle input is a map of a local network. The map provides a list of every connection between two computers. The connections are not directional. 
 LAN parties involve sets of 3 connected computers.
 The chief historian's computer starts with a t.
 
 Part one:

 How many sets of three inter-connected computers contain at least one computer with a name that starts with t?

 Part two:

 Solution:

*/
use std::collections::{HashMap, HashSet};
use utils::{ChallengeConfig, read_puzzle_input, ChallengePart};

fn main() {
    let challenge_config = ChallengeConfig::get();

    let mut network_map = parse_input(challenge_config.is_test);

    let networks = find_connections(&mut network_map);

    match challenge_config.part {
      ChallengePart::One => println!("Networks that contain at least a computer that starts with t: {}", find_connections_with_computers_starting_with(networks, 't')),
      ChallengePart::Two => println!("Not implemented yet"),
    }
}

// Parse the input to get the computers and each connection
// This will be a map of computers to a set of network including itself. Helpful for later intersecting sets.
fn parse_input(is_test: bool) -> Vec<(String, HashSet<String>)> {
  let mut network_map: HashMap<String, HashSet<String>> = HashMap::new();

  let file_path = if is_test { "./src/example_input.txt" } else { "./src/puzzle_input.txt" };

  for line in read_puzzle_input(file_path) {
    let computers: Vec<&str> = line.split("-").collect();

    let left_computer = computers[0].to_string();
    let right_computer = computers[1].to_string();

    if let Some(connections) = network_map.get_mut(&left_computer) {
      connections.insert(right_computer.clone());
    } else {
      network_map.insert(left_computer.to_string(), HashSet::from([right_computer.clone()]));
    }

    if let Some(connections) = network_map.get_mut(&right_computer) {
      connections.insert(left_computer.clone());
    } else {
      network_map.insert(right_computer.clone(), HashSet::from([left_computer.clone()]));
    }

    // println!("left computer: {left_computer}, right_computer: {right_computer} network map: {network_map:?}");
  }

  network_map
    .iter()
    .fold(
      vec![],
      |mut acum, (k, v)| {
        acum.push((k.clone(), v.clone()));
        acum
      }
    )
}

fn find_connections_with_computers_starting_with(networks: Vec<HashSet<String>>, letter: char) -> i32 {
  let mut amount = 0;
  
  for network in networks {
    if network.iter().any(|computer| computer.starts_with(letter)) {
      amount += 1;
    }
  }

  amount
}

// it finds the connections of the computer with length n
fn find_connections(computers: &mut Vec<(String, HashSet<String>)>) -> Vec<HashSet<String>> {
  let mut connected_computers: Vec<HashSet<String>> = Vec::new();

  while let Some((pc, pc_network)) = computers.pop() {
    for other_pc in pc_network.iter() {
      let pair_set = HashSet::from([pc.clone(), other_pc.clone()]);

      let third_pcs: Vec<String> = computers
        .iter()
        .filter_map(|(a,b)| { 
          let intersect: Vec<&String> = b.intersection(&pair_set).collect();
          if intersect.len() == 2 {
            return Some(a.clone());
          }

          None
        })
        .collect();
      
      for third_pc in third_pcs {
        let new_set = HashSet::from([pc.clone(), other_pc.clone(), third_pc]);
        if !connected_computers.contains(&new_set) {
          connected_computers.push(new_set);
        }
      }
    }
  }

  connected_computers
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn number_of_three_interconnected_computers_in_example() {
    let mut puzzle_map = parse_input(true);

    let three_interconnected_computers = find_connections(&mut puzzle_map);

    assert_eq!(three_interconnected_computers.len(), 12);
  }

  #[test]
  fn number_of_three_interconnected_computers_in_example_that_have_a_computer_starting_with_t() {
    let mut puzzle_map = parse_input(true);

    let three_interconnected_computers = find_connections(&mut puzzle_map);
    let starting_with_t = find_connections_with_computers_starting_with(three_interconnected_computers, 't');

    assert_eq!(starting_with_t, 7);
  }
}