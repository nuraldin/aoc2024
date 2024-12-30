/*
 Advent of Code 2024 Day 17: Chronospatial Computer

 I'm in a 3-bit computer. It has three registers A,B,C, these can contain any integers.
 The computer knows 8 instructions, each identified by a 3 bit number called opcode.
 Each instruction reads a 3 bit number after it as an input, called operand.
 A number called the instrucction pointer identiifies the position in the program.
 the instruction pointer increases by 2 after each instruction is processed (except for jump instructions)
 if the computer tries to read an opcode past the end of the program, it halts.
 There are two types of operands,each instruction specifies the type of its operand.
 The value of a literal operand is the operand itself, e.g. 7 is just the number 7. 
 The value of a combo operand is:
  - 0 to 3: a literal value.
  - 4 is the value of register A
  - 5 the value of register B
  - 6 teh value of register C
  - 7 is reserved and will not appear in a valid program.

  These are the instructions:
  - adv (opcode 0): performs division. The numerator is the A register the denominator is 2 raised to a combo operand. The divsion is trucnated and stored in A
  - bxl (opcode 1): bitwise XOR of register B and a literal operand. Stores the result in B
  - bst (opcode 2): value of combo operand modulo 8 the nwrites to the B register.
  - jnz (opcode 3): nothing if the register A is 0, otherwise jumps by setting the instruction pointer to hte value of the literal operand.
  - bxc (opcode 4): bitwise XOR of register B and register C. store the result in register B. The operand is ignored.
  - out (opcode 5): result of combo operand modulo 8, then outputs that value (values are outputed separated by commas)
  - bdv (opcode 6): same as adv but sotring the result in B register.
  - cdv (opcode 7): same as adv  but result is storedin C register.
  
 Part one:
 
 What do I get if I use commas to join the values the program outputs into a single string?

 Part two:

 What is the lowest possible initial value for register A that causes the program to output a copy of itself?

 Solution:

 for part one I just needed to setup the operations and a computer and run the program. The part two was tricky.
 Brute forcing for part two was not possible, so seeing other solutions and thinking about it it was clear that the number of A is just just the lowest possible 
 between numbers that are increasing in powers of 8 fashion, that is shifting 3 times. The best way is to reverse the calculation starting from hte last printed value
 collect all possible A values that would produce that and then shift each of them 3 times and see the next 7 possible values.
 That makes it super fast.

*/
use std::collections::HashMap;
use regex::Regex;
use utils::{ChallengeConfig, ChallengePart};

fn main() {
    let challenge_config = ChallengeConfig::get();
    
    let mut computer = parse_input(&challenge_config);
    println!("{computer:?}");

    match challenge_config.part {
      ChallengePart::One => { 
        computer.run_program();
        println!("The output of the program is: {:?}", computer.flush());
      }
      ChallengePart::Two => println!("The lowest possible value of A that casues a program t o output a copy of itself is: {}", computer.run_until_copy())
    }
}

#[derive(Debug, Eq, PartialEq, Hash,)]
enum Register {
  A, B, C
}

#[derive(Debug)]
struct Computer {
  registers: HashMap<Register, u64>,
  ip: usize,
  output_buffer: Vec<i32>,
  program: Vec<i32>,
}

impl Computer {
  fn new() -> Self {
    let mut registers = HashMap::new();
    registers.insert(Register::A, 0);
    registers.insert(Register::B, 0);
    registers.insert(Register::C, 0);

    Self {
      registers,
      output_buffer: vec![],
      ip: 0,
      program: vec![],
    }
  }

  fn flush(&self) -> String {
    let numbers: Vec<String> = self.output_buffer.iter().map(|n| n.to_string()).collect();
    numbers.join(",")
  }

  // if truncate bool is up, I will compare substrings to finish the run fast.
  fn run_program(&mut self) {
    loop {
      // if the instruction pointer is past the end of the program, halt.
      if self.ip >= self.program.len() {
        break;
      }

      // get next opcode and operand
      let opcode = self.program[self.ip];
      let operand = self.program[self.ip + 1];

      // do operation and sets the next instruction pointer
      self.operate(opcode, operand);
    }
  }

  fn operate(&mut self, opcode: i32, operand: i32) {
    // some opcodes have combo operands that need to be decombized
    let operand = if [0, 2, 5, 6, 7].contains(&opcode) { 
      self.decombize(operand) 
    } else { 
      operand as u64
    };

    match opcode {
      // adv
      0 => {
        self
          .registers
          .entry(Register::A)
          .and_modify(|register| *register /= 2u64.pow(operand as u32));

        self.ip += 2;
      },
      // bxl
      1 => {
        self
          .registers
          .entry(Register::B)
          .and_modify(|register| *register ^= operand);

        self.ip += 2;
      },
      // bst
      2 => {
        self
          .registers
          .insert(Register::B, operand.rem_euclid(8));
      
        self.ip += 2;
      },
      // jnz
      3 => {
        self.ip = if self.registers[&Register::A] != 0 { 
          operand as usize 
        } else {
          self.ip + 2
        }
      },
      // bxc
      4 => {
        let operand = self.registers[&Register::C];
          
        self
          .registers
          .entry(Register::B)
          .and_modify(|register| *register ^= operand);
        
        self.ip += 2;
      },
      // out
      5 => {
        self.output_buffer.push(operand.rem_euclid(8) as i32);
        self.ip += 2;
      }
      // bdv
      6 => {
        let operator = self.registers[&Register::A];

        self
          .registers
          .entry(Register::B)
          .and_modify(|register| *register = operator / 2u64.pow(operand as u32));
      
        self.ip += 2;
      },
      // cdv
      7 => {
        let operator = self.registers[&Register::A];

        self
          .registers
          .entry(Register::C)
          .and_modify(|register| *register = operator / 2u64.pow(operand as u32));
        
        self.ip += 2;
      },
      _ => unreachable!()  
    }; 
  }

  fn decombize(&self, operand: i32) -> u64 {
    match operand {
      0 | 1 | 2 | 3 => operand as u64,
      4 => self.registers[&Register::A],
      5 => self.registers[&Register::B],
      6 => self.registers[&Register::C],
      _ => unreachable!() 
    }
  }

  fn reset(&mut self) {
    self.ip = 0;
    self.output_buffer = vec![];
    self.registers.insert(Register::A,0);
    self.registers.insert(Register::B,0);
    self.registers.insert(Register::C,0);
  }

    // Returns the value of register A that satisfies the output buffer to be equal to the program condition
    // This version is about going from the end to the beginning;
    fn run_until_copy(&mut self) -> u64 {
      let pattern = [0b000, 0b001, 0b010, 0b011, 0b100, 0b101, 0b110, 0b111].to_vec();
      let mut possible_numbers = pattern.clone();
      let mut compared = 0;
  
      loop {
        let mut successful = Vec::new();

        for next_number in &possible_numbers {
          if compared == 0 && *next_number == 0 {
            compared += 1;
            continue;
          }
          self.reset();
          self.registers.insert(Register::A,*next_number);
          self.run_program();
          
          // println!("next_number: {next_number:o}");
          // println!("buf: {:?}", self.output_buffer);

          let (_, last) = self.program.split_at(self.program.len() - self.output_buffer.len());
          if self.output_buffer == last {
            successful.push(next_number);
          }
        }
        
        if self.output_buffer.len() == self.program.len() {
          return **successful.iter().min().unwrap();
        }

        let mut next_possible_numbers = vec![];
        for pos_n in &pattern {
          for suc_n in &successful {
            next_possible_numbers.push((*suc_n << 3 ) + pos_n);
          }
        }
        possible_numbers = next_possible_numbers.clone();
        // println!("next pos n: {:?}", possible_numbers.iter().map(|n| format!("{:o}", n)).collect::<Vec<String>>());
      }
    }
}

fn parse_input(config: &ChallengeConfig) -> Computer  {
  let mut computer = Computer::new();
  let register_pattern = Regex::new(r"Register A: (\d+)Register B: (\d+)Register C: (\d+)").unwrap();
  let program_pattern = Regex::new(r"Program:\s([\d,]+)").unwrap();
  
  let lines: Vec<String> = config.read_puzzle_input(None).collect();
  let file = lines.concat();

  let register_captures = register_pattern.captures(&file).unwrap();
  computer.registers.insert(Register::A, register_captures.get(1).unwrap().as_str().parse().unwrap());
  computer.registers.insert(Register::B, register_captures.get(2).unwrap().as_str().parse().unwrap());
  computer.registers.insert(Register::C, register_captures.get(3).unwrap().as_str().parse().unwrap());

  let program_captures = program_pattern.captures(&file).unwrap();
  computer.program = program_captures.get(1).unwrap().as_str().split(",").map(|number| number.parse().unwrap()).collect();

  computer
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn example_1() {
    let mut computer = Computer::new();
    computer.registers.insert(Register::C,  9);
    computer.program = [2,6].to_vec();

    computer.run_program();
    
    assert_eq!(computer.registers[&Register::B], 1);
  }

  #[test]
  fn example_2() {
    let mut computer = Computer::new();
    computer.registers.insert(Register::A,  10);
    computer.program = [5,0,5,1,5,4].to_vec();

    computer.run_program();

    assert_eq!("0,1,2", computer.flush());
  }

  #[test]
  fn example_3() {
    let mut computer = Computer::new();
    computer.registers.insert(Register::A,  2024);
    computer.program = [0,1,5,4,3,0].to_vec();
    
    computer.run_program();

    assert_eq!("4,2,5,6,7,7,7,7,3,1,0", computer.flush());
    assert_eq!(computer.registers[&Register::A], 0);
  }


  #[test]
  fn example_4() {
    let mut computer = Computer::new();
    computer.registers.insert(Register::B,  29);
    computer.program = [1,7].to_vec();
    
    computer.run_program();
    
    assert_eq!(computer.registers[&Register::B], 26);
  }


  #[test]
  fn example_5() {
    let mut computer = Computer::new();
    computer.registers.insert(Register::B,  2024);
    computer.registers.insert(Register::C,  43690);
    computer.program = [4,0].to_vec();
    
    computer.run_program();
    
    assert_eq!(computer.registers[&Register::B], 44354);
  }

  #[test]
  fn example_input_produces_correct_output() {
    let mut computer = Computer::new();
    computer.registers.insert(Register::A,  729);
    computer.registers.insert(Register::B,  0);
    computer.registers.insert(Register::C,  0);
    computer.program = [0, 1, 5, 4, 3, 0].to_vec();

    computer.run_program();    

    assert_eq!("4,6,3,5,6,3,5,2,1,0", computer.flush());
  }

  #[test]
  fn run_until_copy_with_example_input() {
    let mut computer = Computer::new();
    computer.program = [0, 3, 5, 4, 3, 0].to_vec();

    assert_eq!(computer.run_until_copy(), 117440);
  }
}