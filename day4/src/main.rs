/*
 Advent of Code 2024 Day 4 

 Part one:

 We have a `word search` specifically `XMAS`.
 The word can be horizontal, vertical, diagonal, backwards or even overlapping other words.
 We need to find all of them

 Part two:

 We need to search this time X-MAS, i.e. the word MAS in a shape of an X.
 It can also be backwards like SAM
*/
use utils::read_puzzle_input;
use regex::Regex;

fn main() {
  //  println!("The puzzle: {:?}", parse_puzzle())
  let puzzle = parse_puzzle();

  let mut words = 0;
  
  // Uncomment to solve part one
  // for (line_idx, puzzle_line) in puzzle.iter().enumerate() {
  //   words += find_horizontal(puzzle_line.clone());
  //   words += find_vertical(line_idx, puzzle_line.clone(), puzzle.clone());
  //   words += find_diagonal(line_idx, puzzle_line.clone(), puzzle.clone());
  // }

  for line_idx in 1..puzzle.len() - 1 {
    words += find_x_mas(line_idx, puzzle.clone());
  }

  println!("The amount of XMAS in the puzzle is: {:?}", words);
}

fn parse_puzzle() -> Vec<Vec<char>> {
  let mut puzzle: Vec<Vec<char>> = Vec::new();

  for line in read_puzzle_input("./src/puzzle_input.txt") {
    let mut puzzle_line: Vec<char> = Vec::new();
    
    for letter in line.expect( "Couldn't get file line").chars() {
      puzzle_line.push(letter)
    }

    puzzle.push(puzzle_line)
  }

  puzzle
}

fn find_horizontal(puzzle_line: Vec<char>) -> usize {
  let mut horizontal_words = 0;
  let forward_pattern = Regex::new("XMAS").unwrap();
  let backward_patterns = Regex::new("SAMX").unwrap();

  horizontal_words += forward_pattern.find_iter(String::from_iter(puzzle_line.clone()).as_str()).count();
  horizontal_words += backward_patterns.find_iter(String::from_iter(puzzle_line.clone()).as_str()).count();

  horizontal_words
}

fn find_vertical(line_idx: usize, puzzle_line: Vec<char>, puzzle: Vec<Vec<char>>) -> usize {
  let mut vertical_words= 0;

  // do backwards only if there are sufficient lines above
  if line_idx >= 3 {
    for (column_idx, letter) in puzzle_line.iter().enumerate() {
      if *letter != 'X' {
        continue;
      }
  
      // I'll assume the puzzle is always a sufficient height
      if puzzle[line_idx - 1][column_idx] == 'M' &&  puzzle[line_idx - 2][column_idx] == 'A' &&  puzzle[line_idx - 3][column_idx] == 'S' {
        vertical_words += 1;
      }
    }  
  }

  // Check vertically if only there are sufficient amount of letters below
  if puzzle.len() - 1 - line_idx >= 3 { 
    for (column_idx, letter) in puzzle_line.iter().enumerate() {
      if *letter != 'X' {
        continue;
      }

      // I'll assume the puzzle is always a sufficient height
      if puzzle[line_idx + 1][column_idx] == 'M' &&  puzzle[line_idx + 2][column_idx] == 'A' &&  puzzle[line_idx + 3][column_idx] == 'S' {
        vertical_words += 1;
      }
    }
  }
  
  vertical_words
}

fn find_diagonal(line_idx: usize, puzzle_line: Vec<char>, puzzle: Vec<Vec<char>>) -> usize {
  let mut diagonal_words= 0;

  // do backwards only if there are sufficient lines above
  if line_idx >= 3 {
    for (column_idx, letter) in puzzle_line.iter().enumerate() {
      if *letter != 'X' {
        continue;
      }

      // required for backwards diagonal
      if column_idx >= 3 {
        if puzzle[line_idx - 1][column_idx - 1] == 'M' &&  puzzle[line_idx - 2][column_idx - 2] == 'A' &&  puzzle[line_idx - 3][column_idx - 3] == 'S' {
          diagonal_words += 1;
        }
      }

      // required for forwards diagonal
      if column_idx <= puzzle_line.len() - 4 {
        if puzzle[line_idx - 1][column_idx + 1] == 'M' &&  puzzle[line_idx - 2][column_idx + 2] == 'A' &&  puzzle[line_idx - 3][column_idx + 3] == 'S' {
          diagonal_words += 1;
        }
      } 
    }  
  }

  // do backwards only if there are sufficient lines above
  if puzzle.len() - 1 - line_idx >= 3 {
    for (column_idx, letter) in puzzle_line.iter().enumerate() {
      if *letter != 'X' {
        continue;
      }

      // required for backwards diagonal
      if column_idx >= 3 {
        if puzzle[line_idx + 1][column_idx - 1] == 'M' &&  puzzle[line_idx + 2][column_idx - 2] == 'A' &&  puzzle[line_idx + 3][column_idx - 3] == 'S' {
          diagonal_words += 1;
        }
      }

      // required for forwards diagonal
      if column_idx <= puzzle_line.len() - 4 {
        if puzzle[line_idx + 1][column_idx + 1] == 'M' &&  puzzle[line_idx + 2][column_idx + 2] == 'A' &&  puzzle[line_idx + 3][column_idx + 3] == 'S' {
          diagonal_words += 1;
        }
      } 
    }  
  }


  diagonal_words
}

fn find_x_mas(line_idx: usize, puzzle: Vec<Vec<char>>) -> usize {
  let mut x_mases = 0;

  for column_idx in 1..puzzle[line_idx].len() - 1 {
    if puzzle[line_idx][column_idx] != 'A' {
      continue;
    }

    if ((puzzle[line_idx - 1][column_idx - 1] == 'M' && puzzle[line_idx + 1][column_idx + 1] == 'S') || 
       (puzzle[line_idx - 1][column_idx - 1] == 'S' && puzzle[line_idx + 1][column_idx + 1] == 'M')) &&
       ((puzzle[line_idx - 1][column_idx + 1] == 'M' && puzzle[line_idx + 1][column_idx - 1] == 'S') || 
       (puzzle[line_idx - 1][column_idx + 1] == 'S' && puzzle[line_idx + 1][column_idx - 1] == 'M')) {
      x_mases += 1;
    }
  }

  x_mases
}