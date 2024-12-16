/*
 Advent of Code 2024 Day 2: Red-Nose Reports

 Part one:

 Our data are `reports`, one report per line. Each report is a list of numbers called `levels`, separated by spaces.
 I need to figure out which report is `safe`.
 A report is `safe` if both are true:
   - all `levels` are increasing or decreasing
   - Any two adjacent `levels` differ by at least one and at most three.

 Part two:

 There is a problem dampener that allows to have one single bad level.
 If removing the bad level makes the report safe, then it is counted as safe.
 With the same rules as before
*/
use utils::{get_challenge_config, read_puzzle_input, ChallengePart};

fn main() {
  let challenge_config = get_challenge_config();

  let reports = parse_input(challenge_config.is_test);
  let mut safe_reports: u32 = 0;

  match challenge_config.part {
    ChallengePart::One => {
      for report in reports {
        if inc_dec_rule(report.clone()) && differ_rule(report.clone()) {
          safe_reports += 1;
        }
      }
    },
    ChallengePart::Two => {
      for report in reports {
        if inc_dec_rule(report.clone()) && differ_rule(report.clone()) {
            safe_reports += 1;
        } else if problem_dampener(report.clone()) {
            safe_reports += 1;
        }
      }
    }
  }
    
  println!("The number of safe reports is {}", safe_reports);
}

fn parse_input(is_test: bool) -> Vec<Vec<u32>> {
  let mut reports: Vec<Vec<u32>> = Vec::new();

  let file_path = if is_test { "./src/example_input.txt" } else { "./src/puzzle_input.txt" };

  for line in read_puzzle_input(file_path) {
      let mut report: Vec<u32> = Vec::new();

      for level in line.trim().split_ascii_whitespace() {
          report.push(level.parse().expect("Couldn't parse level"))
      }

      // println!("Read a report: {:?}", report);

      reports.push(report);
  }

  reports
}

fn inc_dec_rule(report: Vec<u32>) -> bool {
  let increasing = report.is_sorted();
  let decreasing = report
    .iter()
    .rev()
    .is_sorted();

  // println!("Report {:?} passes the inc_dec_rule?: {}", report, increasing || decreasing);
  increasing || decreasing
}

fn differ_rule(report: Vec<u32>) -> bool {
  for idx in 0..report.len() - 1 {
      let diff = report[idx].abs_diff(report[idx + 1]);

      if diff < 1 || diff > 3 {
          return false;
      }
  }

  // println!("Report {:?} passes the differ rule?: {}", report, pass);
  true
}

fn problem_dampener(report: Vec<u32>) -> bool {
    let mut pass: bool = false;

    for idx in 0..report.len() {
        let mut report_copy = report.clone();
        report_copy.remove(idx);

        if inc_dec_rule(report_copy.clone()) && differ_rule(report_copy.clone()) {
            pass = true;
        }
    }

    pass
}
