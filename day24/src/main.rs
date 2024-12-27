
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
use utils::{ChallengeConfig, read_puzzle_input, ChallengePart};

use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum WireState {
  One,
  Zero,
  None,
}

impl WireState {
  fn to_string(&self) -> String {
    match self {
      WireState::None => "None".to_string(),
      WireState::One => "1".to_string(),
      WireState::Zero => "0".to_string()
    }
  }
  /// Returns the numerical value of the wire or None. Useful for arithmetic operations
  fn to_val(&self) -> i32 {
    match self {
      WireState::One => 1,
      WireState::Zero => 0,
      WireState::None => panic!("Cannot operate on a not ready wire state"),
    }
  }

  /// Transforms a numerical value to the Wire enum.
  fn from_int(v: i32) -> WireState {
    if v > 0 { WireState::One } else { WireState::Zero }
  }

  fn from_str(v: &str) -> WireState {
    match v {
      "0" => WireState::Zero,
      "1" => WireState::One,
      _ => panic!("Cannot parse {v} into a wire state")
    }
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
  inputs: (String, String),
  output: String,
  operation: LogicalOp
}

impl Gate {
  fn can_operate(&self, wire_states: &HashMap<String, WireState>) -> bool {
    let input_a = wire_states.get(&self.inputs.0).unwrap();
    let input_b = wire_states.get(&self.inputs.1).unwrap();

    *input_a != WireState::None && *input_b != WireState::None  
  }

  fn has_operated(&self, wire_states: &HashMap<String, WireState>) -> bool {
    let output = wire_states.get(&self.output).unwrap();

    *output != WireState::None
  }

  fn process_output(&mut self, wire_states: &mut HashMap<String, WireState>) {
    if !self.can_operate(&wire_states) {
      panic!("One of the inputs is not ready to operate for gate with inputs: {} {}", self.inputs.0, self.inputs.1)
    }

    let result = self.operation.calculate(
      wire_states.get(&self.inputs.0).unwrap().to_val(),
      wire_states.get(&self.inputs.1).unwrap().to_val(), 
    );

    wire_states.insert(self.output.clone(), WireState::from_int(result));
  }
}

#[derive(Debug)]
struct SystemConfig {
  wires: HashMap<String, WireState>,
  gates: Vec<Gate>,
}

impl SystemConfig {
  fn new() -> Self {
    Self {
      wires: HashMap::new(),
      gates: vec![]
    }
  }

  fn print_outputs(&self) {
    let mut outputs: Vec<(String, String)> = self.wires.iter().filter_map(|(label, state)| {
      if label.starts_with("z") {
        return Some((label.clone(), state.to_string()));
      }
      None
    }).collect();

    outputs.sort();
    outputs.reverse();
    
    println!("raw collection: {outputs:?}");

    let outputs: Vec<String> = outputs.iter().map(|(_, value)| value.clone()).collect();
    println!("binary: {}", outputs.concat());
    println!("decimal: {}", u64::from_str_radix(&outputs.concat(), 2).unwrap())
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
    let parsed_line: Vec<&str> = inputs_line.split(": ").collect();

    let wire_label = parsed_line[0].to_string();
    let wire_state = WireState::from_str(parsed_line[1]);

    system_config.wires.insert(wire_label, wire_state);
  }

  // get gates configuration 
  for gates_line in read_puzzle_input(gates_file) {
    let parsed_line: Vec<&str> = gates_line.split(" ").collect();

    let input_a= parsed_line[0].to_string();
    let input_b = parsed_line[2].to_string();  
    let output = parsed_line[4].to_string();

    system_config.gates.push(Gate {
      inputs: (input_a, input_b),
      output: output.clone(),
      operation: LogicalOp::from(parsed_line[1]),
    });

    system_config.wires.entry(output).or_insert(WireState::None);

  } 

  system_config
}

fn main() {
    let challenge_config = ChallengeConfig::get(();

    let mut system_config = parse_input(challenge_config.is_test);

    match challenge_config.part {
      ChallengePart::One => {
        while system_config.wires.iter().any(|(label, state)| label.starts_with("z") && *state == WireState::None ) {
          for gate in &mut system_config.gates {
            if gate.can_operate(&system_config.wires) && !gate.has_operated(&system_config.wires) {
              gate.process_output(&mut system_config.wires);
            }
          }
        }
        system_config.print_outputs();
      
      },
      ChallengePart::Two => println!("Not implemented yet"),
    }
}