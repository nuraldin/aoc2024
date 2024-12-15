/*
 Advent of Code 2024 Day 15: Warehouse Woes

 The input is a map of a warehouse and a list of movements a robot will attempt to make.
 The movements will not always succeed as the warehouse has boxes that are shifted around.
 If the robot collides with a box (0) it will try to move it, if it is a wall (#) it will not move.
 The movements are (^:up <: left, >:right, v: down). and the list is a giant sequence in order.
 The boxes have GPS coordinates to track the mwhich is 100 times the distance from the top edge plus ist distance from the edge of the map. i.e. 100x + y.

 Part one:

 What is the sum of all boxes' GPS coordinates?

 Part two:

 Solution: 

 
*/
use utils::{get_challenge_config, ChallengePart};

fn main() {
    let challenge_config = get_challenge_config();
    
    match challenge_config.part {
      ChallengePart::One => println!("Not implemented yet"),
      ChallengePart::Two => println!("Not implemented yet"),
    }
}