
/*
 Advent of Code 2024 Day 24: Crossed Wires

 The puzzle input is info about logical gate connections and initial wire values.
 There are no loops. Gates wait until both inputs are received before producing a value.
 Wires can carry 0,1 or no value at all. Outputs do not change until the whole system is reset.
 Each wire is connected to at most one gate output, but can be connected to many gate inputs.
 There are only three types of gates: AND, OR and XOR.

 Part one:

 What decimal number does the system of gatest and wires output on the wires starting with z?

 Part two:
*/
use utils::{get_challenge_config, read_puzzle_input, ChallengePart};

use std::collections::HashMap;

#[derive(Debug)]
struct Wire {
  value: WireState,
  label: String
}


#[derive(Debug, PartialEq)]
enum WireState {
  One,
  Zero,
  None,
}

impl WireState {
  /// Returns the numerical value of the wire or None. Useful for arithmetic operations
  fn to_val(&self) -> i32 {
    match self {
      WireState::One => 1,
      WireState::Zero => 0,
      WireState::None => panic!("Cannot operate on a not ready wire state"),
    }
  }

  /// Transforms a numerical value to the Wire enum.
  fn to_enum(val: i32) -> WireState {
    if val > 0 { WireState::One } else { WireState::Zero }
  }
}

#[derive(Debug)]
enum LogicalOp {
  AND,
  OR,
  XOR,
}

impl LogicalOp {
  fn from(v: &str) -> LogicalOp {
    match v {
      "AND" => LogicalOp::AND, 
      "OR" => LogicalOp::OR,
      "XOR" => LogicalOp::XOR,
      _ => panic!("Cannot make {v} into a logical operation")
    }
  }

  fn calculate(&self, a: i32, b: i32) -> i32 {
    match self {
      LogicalOp::AND => a & b,
      LogicalOp::OR => a | b,
      LogicalOp::XOR => a ^ b,
    }
  }
}

#[derive(Debug)]
struct Gate {
  inputs: (Wire, Wire),
  output: Wire,
  operation: LogicalOp
}

impl Gate {
  fn can_operate(&self) -> bool {
    self.inputs.0.value != WireState::None && self.inputs.1.value != WireState::None  
  }

  fn process_output(&mut self) {
    if !self.can_operate() {
      panic!("One of the inputs is not ready to operate for gate with inputs: {} {}", self.inputs.0.label, self.inputs.1.label)
    }

    let result = self.operation.calculate(
      self.inputs.0.value.to_val(),
      self.inputs.1.value.to_val(), 
    );

    self.output.value = WireState::to_enum(result);
  }
}

#[derive(Debug)]
struct SystemConfig {
  wires: HashMap<String, Wire>,
  gates: Vec<Gate>,
}

impl SystemConfig {
  fn new() -> Self {
    Self {
      wires: HashMap::new(),
      gates: vec![]
    }
  }
}

fn parse_input(is_test: bool) -> SystemConfig {
  let mut system_config = SystemConfig::new();

  let (gates_file, inputs_file) = if is_test { 
    ("./src/example_gates.txt", "./src/example_inputs.txt") 
  } else {
    ("./src/puzzle_gates.txt", "./src/puzzle_inputs.txt")
  };

  // get inputs starting configuration
  for inputs_line in read_puzzle_input(inputs_file) {

  }

  // get gates configuration 
  for gates_line in read_puzzle_input(gates_file) {

  } 

  system_config
}

fn main() {
    let challenge_config = get_challenge_config();

    let system_config = parse_input(challenge_config.is_test);
    println!("sysem config: {system_config:?}");
    
    match challenge_config.part {
      ChallengePart::One => println!("Not implemented yet"),
      ChallengePart::Two => println!("Not implemented yet"),
    }
}