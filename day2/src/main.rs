/*
 Advent of Code 2024 Day 2

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
use utils::{get_challenge_part, read_puzzle_input, ChallengePart};

fn main() {
  let reports = parse_input();
  let mut safe_reports: u32 = 0;

  match get_challenge_part() {
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

fn parse_input() -> Vec<Vec<u32>> {
  let mut reports: Vec<Vec<u32>> = Vec::new();

  for line in read_puzzle_input("./src/example_input.txt") {
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
    let mut report_copy = report.clone();
    report_copy.reverse();
    let decreasing = report_copy.is_sorted();

    // println!("Report {:?} passes the inc_dec_rule?: {}", report, increasing || decreasing);
    increasing || decreasing
}

fn differ_rule(report: Vec<u32>) -> bool {
    let mut pass = true;

    for idx in 0..report.len() - 1 {
        let diff = report[idx].abs_diff(report[idx + 1]);

        if diff < 1 || diff > 3 {
            pass = false;
            break;
        }
    }

    // println!("Report {:?} passes the differ rule?: {}", report, pass);

    pass
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
