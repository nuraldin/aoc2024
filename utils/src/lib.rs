use std::fs::File;
use std::io::{self, BufRead};

pub fn read_puzzle_input(file_path: &str) -> impl Iterator<Item = io::Result<String>> {
    let file: File = File::open(file_path).expect("Couldn't open specified file");
    let reader = io::BufReader::new(file);

    reader.lines()
}
