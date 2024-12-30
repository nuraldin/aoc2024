/*
 Advent of Code 2024 Day 18: RAM run

 the input is a list of bytes that are falling.
 the memory space is a two dimensional grid with coordinates that range from 0 to 70 both horizontally and vertically.
 I start always at the 0,0 memory space and need to reach the exit at 70,70.
 I cannot leave the the boundaries of the memory space and every time a byte fall into a memory space that coordinate gets corrupted.

 Part one:

 Simulate the first kilobyte (1024 bytes) falling onto your memory space.
 Afterwards, what is the minimum number of fsteps neeeded to reach the exit?

 Part two:
*/
use utils::{ChallengeConfig, ChallengePart};

fn main() {
    let challenge_config = ChallengeConfig::get();
    
    match challenge_config.part {
      ChallengePart::One => println!("Not implemented yet"),
      ChallengePart::Two => println!("Not implemented yet"),
    }
}