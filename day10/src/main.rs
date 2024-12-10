/*
 Advent of Code 2024 Day 10

 Part one:
 
 the input is a topographic map that indicate the `height` at each position using a scale of 0 to 9 (highest).
 a `hiking trail` is a longest possible, even ,gradual uphill slope. i.e. starts at 0 ends at 9 and always increases at a height of 1 at each step.
 The steps can only be up, down, left or right. 
 a `trailhead` is any position that starts a one or more hiking trails.
 a trailhead's score is the number of unique 9 height positions reachable from that trailhead via a hiking trail.
 the result is the sum of the scores of all trailheads.

 Part two:
 now the result is the number of distinct hiking trails which begin at a certain trailhead.
 aka doesn't mattter if it arrives to hte same 9, just how many ways it has to reach to 9s
*/
use utils::{get_challenge_config, read_puzzle_input, ChallengeConfig, ChallengePart};

fn main() {
    let challenge_config = get_challenge_config();
    
    let topographic_map = parse_input(&challenge_config);

    match challenge_config.part {
      ChallengePart::One => println!("The sum of trailheads score is: {}", trailheads_score(topographic_map)),
      ChallengePart::Two => println!("The sum of trailheads ratings is: {}", trailheads_ratings(topographic_map)),
    }
}

type TopographicMap = Vec<Vec<char>>;

fn parse_input(config: &ChallengeConfig) -> TopographicMap {
  let mut topographic_map = Vec::new();

  for line in read_puzzle_input(if config.is_test {"./src/example_input.txt"} else { "./src/puzzle_input.txt" }) {
    topographic_map.push(line.chars().collect())
  }

  topographic_map
}

#[derive(Debug, PartialEq)]
struct Location {
  x: i32,
  y: i32,
}

fn trailheads_score(topographic_map: TopographicMap) -> usize {
  let trailheads = get_trailheads(&topographic_map);
  // println!("Trailheads locations: {:?}", trailheads);

  let mut trailheads_score = 0;
  for trailhead in trailheads {
    trailheads_score += get_trailhead_score(&trailhead, &topographic_map);
  }

  trailheads_score
}

fn trailheads_ratings(topographic_map: TopographicMap) -> usize {
  let trailheads = get_trailheads(&topographic_map);
  // println!("Trailheads locations: {:?}", trailheads);

  let mut trailheads_ratings = 0;
  for trailhead in trailheads {
    trailheads_ratings += get_trailhead_rating(&trailhead, &topographic_map);
  }

  trailheads_ratings
}

fn get_trailheads(topographic_map: &TopographicMap) -> Vec<Location> {
  let mut trailheads = Vec::new();

  for (row_idx, row) in topographic_map.iter().enumerate() {
    for (col_idx, col) in row.iter().enumerate() {
      if *col == '0' {
        trailheads.push(Location { x: col_idx as i32, y: row_idx as i32})
      }
    }
  } 

  trailheads
}

fn get_trailhead_score(trailhead: &Location, topographic_map: &TopographicMap) -> usize {
  let mut hiking_trails = Vec::new();
  get_hiking_trails_end_locations(trailhead, topographic_map, &mut hiking_trails, true); 
  // println!("Unique Hiking trails ending locations: {:?}", hiking_trails);

  hiking_trails.iter().count()
}

fn get_trailhead_rating(trailhead: &Location, topographic_map: &TopographicMap) -> usize {
  let mut hiking_trails = Vec::new();
  get_hiking_trails_end_locations(trailhead, topographic_map, &mut hiking_trails, false); 
  // println!("Hiking trails ending locations: {:?}", hiking_trails);

  hiking_trails.iter().count()
}

fn is_location_outbounds(location: &Location, topographic_map: &TopographicMap) -> bool {
  location.x >= topographic_map.len() as i32 || location.x < 0 || location.y >= topographic_map[0].len() as i32 || location.y < 0
}

fn get_hiking_trails_end_locations(location: &Location, topographic_map: &TopographicMap, hiking_trails: &mut Vec<Location>, unique: bool) {
  if is_location_outbounds(location, topographic_map) {
    return;
  }

  if topographic_map[location.y as usize][location.x as usize] == '9' {
    if !unique || !hiking_trails.iter().any(|item| item == location ) {
      hiking_trails.push(Location { x: location.x, y: location.y });
    }
  }

  for next_location in get_next_possible_locations(&location, topographic_map) {
    get_hiking_trails_end_locations(&next_location, topographic_map, hiking_trails, unique);
  }
} 

fn get_next_possible_locations(location: &Location, topographic_map: &TopographicMap) -> Vec<Location> {
  let mut next_possible_locations = Vec::new();

  let left = Location { 
    x: location.x - 1,
    y: location.y
  };

  let right = Location { 
    x: location.x + 1,
    y: location.y
  };

  let up = Location { 
    x: location.x,
    y: location.y - 1
  };

  let down = Location { 
    x: location.x,
    y: location.y + 1,
  };

  // can I go up?
  if !is_location_outbounds(&up, topographic_map) && !is_wall(&up, topographic_map) &&is_evenly_higher(&up, location, topographic_map) {
    next_possible_locations.push(up);
  }
  // can I go up?
  if !is_location_outbounds(&down, topographic_map) && !is_wall(&down, topographic_map) && is_evenly_higher(&down, location, topographic_map) {
    next_possible_locations.push(down);
  }
  // can I go up?
  if !is_location_outbounds(&left, topographic_map) && !is_wall(&left, topographic_map) && is_evenly_higher(&left, location, topographic_map) {
    next_possible_locations.push(left);
  }
  // can I go up?
  if !is_location_outbounds(&right, topographic_map) && !is_wall(&right, topographic_map) && is_evenly_higher(&right, location, topographic_map) {
    next_possible_locations.push(right);
  }

  // println!("current location: {:?}, next possible locations: {:?}", location, next_possible_locations);
  next_possible_locations
}

fn is_evenly_higher(next_location: &Location, current_location: &Location, topographic_map: &TopographicMap) -> bool {
  let next_location = topographic_map[next_location.y as usize][next_location.x as usize].to_digit(10).unwrap() as i32;
  let current_location = topographic_map[current_location.y as usize][current_location.x as usize].to_digit(10).unwrap() as i32;

  next_location - current_location == 1 
}

fn is_wall(next_location: &Location, topographic_map: &TopographicMap) -> bool {
  let next_location = topographic_map[next_location.y as usize][next_location.x as usize];

  next_location == '.'
}