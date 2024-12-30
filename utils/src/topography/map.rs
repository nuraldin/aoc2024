use std::collections::HashMap;

use super::coordinate::Coordinate;

/// Main topograhic map type
pub type TopographicMap<T> = HashMap<Coordinate, T>;

/// Helper function to print current map state
pub fn print_coordinate_map(map: &TopographicMap<char>) {
  let mut max_x = 0;
  let mut max_y = 0;
  for key in map.keys() {
    max_x = max_x.max(key.x);
    max_y = max_y.max(key.y);
  }

  for idx in 0..max_x + 1 {
    let mut line = Vec::new();
    for idy in 0..max_y + 1 {
      let coordinate = Coordinate { x: idx as i32, y: idy as i32};
      if let Some(item) = map.get(&coordinate) {
        line.push(item.to_string());
      } else {
        line.push(" ".to_string());
      }
    }

    if line.len() > 0 && !line.iter().all(|c| c == " "){
      println!("{}", line.concat())
    }
  }
}
