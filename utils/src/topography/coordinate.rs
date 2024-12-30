
use super::direction::Direction;

/// This type represents a topographic location in a map.
/// X and Y are the row and column respectively.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Coordinate {
  pub x: i32,
  pub y: i32,
}

impl Coordinate {
  pub fn new(x: i32, y: i32) -> Self {
    Self { x, y }
  }

  pub fn is_outside_boundaries(&self, max: (i32, i32)) -> bool {
    (self.x < 0 || self.y < 0) || (self.x >= max.0 || self.y >= max.1 )
  }

  pub fn add_delta(&self, direction: &Direction) -> Coordinate {
    match direction {
      Direction::Up =>        Coordinate { x: self.x - 1, y: self.y },
      Direction::Down =>      Coordinate { x: self.x + 1, y: self.y },
      Direction::Right =>     Coordinate { x: self.x,     y: self.y + 1 },
      Direction::Left =>      Coordinate { x: self.x,     y: self.y - 1 },
      Direction::UpRight =>   Coordinate { x: self.x - 1, y: self.y + 1},
      Direction::UpLeft =>    Coordinate { x: self.x - 1, y: self.y - 1},
      Direction::DownRight => Coordinate { x: self.x + 1, y: self.y + 1},
      Direction::DownLeft =>  Coordinate { x: self.x + 1, y: self.y - 1},
    }
  }

  pub fn is_adjacent(&self, coordinate: &Coordinate) -> bool {
    for direction in Direction::iter() {
      let next = coordinate.add_delta(&direction);

      if next == *self {
        return true; 
      }
    }

    false
  }

  pub fn is_diagonal(&self, coordinate: &Coordinate) -> bool {
    for direction in Direction::diagonal_iter() {
      let next = coordinate.add_delta(&direction);

      if next == *self {
        return true;
      }
    }

    false
  }

  pub fn get_direction(&self, coordinate: &Coordinate) -> Option<Direction> {
    for direction in Direction::diagonal_iter() {
      let next = coordinate.add_delta(&direction);

      if next == *self {
        return Option::from(direction);
      }
    }

    Option::None
  }

  pub fn is_outer_edge(&self, direction: &Direction, surrounding_coordinates: &Vec<Coordinate>) -> bool {
    match direction {
      Direction::DownLeft => {
        !surrounding_coordinates.contains(&self.add_delta(&Direction::Left)) && 
        !surrounding_coordinates.contains(&self.add_delta(&Direction::Down))
      },
      Direction::DownRight => {
        !surrounding_coordinates.contains(&self.add_delta(&Direction::Down)) && 
        !surrounding_coordinates.contains(&self.add_delta(&Direction::Right))
      },
      Direction::UpRight => {
        !surrounding_coordinates.contains(&self.add_delta(&Direction::Up)) && 
        !surrounding_coordinates.contains(&self.add_delta(&Direction::Right))
      },
      Direction::UpLeft => {
        !surrounding_coordinates.contains(&self.add_delta(&Direction::Up)) && 
        !surrounding_coordinates.contains(&self.add_delta(&Direction::Left))
      },
      _ => unreachable!()
    }
  }

  pub fn is_inner_edge(&self, direction: &Direction, surrounding_coordinates: &Vec<Coordinate>) -> bool {
    match direction {
      Direction::DownLeft => {
        surrounding_coordinates.contains(&self.add_delta(&Direction::Left)) && 
        surrounding_coordinates.contains(&self.add_delta(&Direction::Down)) &&
        !surrounding_coordinates.contains(&self.add_delta(&Direction::DownLeft))
      },
      Direction::DownRight => {
        surrounding_coordinates.contains(&self.add_delta(&Direction::Down)) && 
        surrounding_coordinates.contains(&self.add_delta(&Direction::Right)) &&
        !surrounding_coordinates.contains(&self.add_delta(&Direction::DownRight))
      },
      Direction::UpRight => {
        surrounding_coordinates.contains(&self.add_delta(&Direction::Up)) && 
        surrounding_coordinates.contains(&self.add_delta(&Direction::Right)) &&
        !surrounding_coordinates.contains(&self.add_delta(&Direction::UpRight))
      },
      Direction::UpLeft => {
        surrounding_coordinates.contains(&self.add_delta(&Direction::Up)) && 
        surrounding_coordinates.contains(&self.add_delta(&Direction::Left)) &&
        !surrounding_coordinates.contains(&self.add_delta(&Direction::UpLeft))
      },
      _ => unreachable!()
    }
  }
}