/// These are all the variants of possible directions in a map
#[derive(Clone, PartialEq, Debug, Eq, Hash)]
pub enum Direction {
  Up,
  Down,
  Left,
  Right,
  UpRight,
  UpLeft, 
  DownLeft,
  DownRight,
}

impl Direction {
  /// Creates a direction from a character
  pub fn from_char(char: char) -> Direction {
    match char { 
      '^' => Direction::Up,
      '<' => Direction::Left,
      '>' => Direction::Right,
      'v' => Direction::Down ,
      _ => panic!("Cannot transform into a Direction type"),
    }
  }

  /// Returns the equivalente character to the direction enum
  pub fn to_char(&self) -> char {
    match self { 
      Direction::Up => '^', 
      Direction::Right => '>',
      Direction::Down => 'v',
      Direction::Left => '<',
      _ => unreachable!() // Currently, diagonals are not supported
    }
  }

  /// Rotates the direction 90 Degrees right.
  pub fn rotate_right(&self) -> Direction {
    match self { 
      Direction::Up => Direction::Right, 
      Direction::Right => Direction::Down,
      Direction::Down => Direction::Left,
      Direction::Left => Direction::Up,
      _ => unreachable!()
    }
  }

  /// Returns an vector representation of the directions enum, good for iterating through them.
  /// Diagonals are not currently supported.
  pub fn iter() -> impl Iterator<Item = Direction> {
    [Direction::Up,  Direction::Down, Direction::Left, Direction::Right].into_iter()
  }

  /// Returns an vector representation of the directions enum, good for iterating through them.
  /// Diagonals are not currently supported.
  pub fn diagonal_iter() -> impl Iterator<Item = Direction> {
    [Direction::UpRight,  Direction::DownRight, Direction::DownLeft, Direction::UpLeft].into_iter()
  }
}