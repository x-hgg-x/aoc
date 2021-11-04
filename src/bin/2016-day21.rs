use eyre::{eyre, Result};
use itertools::Itertools;
use regex::Regex;

use std::fs;

enum Operation {
    SwapPosition(usize, usize),
    SwapLetter(u8, u8),
    RotateLeft(usize),
    RotateRight(usize),
    RotatePosition(u8),
    ReversePosition(usize, usize),
    MovePosition(usize, usize),
}

impl Operation {
    fn execute(&self, password: &mut [u8]) {
        match *self {
            Operation::SwapPosition(pos1, pos2) => password.swap(pos1, pos2),
            Operation::SwapLetter(c1, c2) => {
                let pos1 = password.iter().position(|&x| x == c1).unwrap();
                let pos2 = password.iter().position(|&x| x == c2).unwrap();
                password.swap(pos1, pos2);
            }
            Operation::RotateLeft(count) => password.rotate_left(count),
            Operation::RotateRight(count) => password.rotate_right(count),
            Operation::RotatePosition(c) => {
                let pos = password.iter().position(|&x| x == c).unwrap();
                let count = [1, 2, 3, 4, 6, 7, 0, 1];
                password.rotate_right(count[pos]);
            }
            Operation::ReversePosition(pos1, pos2) => password[pos1..=pos2].reverse(),
            Operation::MovePosition(pos1, pos2) => {
                if pos1 <= pos2 {
                    password[pos1..=pos2].rotate_left(1);
                } else {
                    password[pos2..=pos1].rotate_right(1);
                }
            }
        }
    }

    fn cancel(&self, password: &mut [u8]) {
        match *self {
            Operation::SwapPosition(pos1, pos2) => password.swap(pos1, pos2),
            Operation::SwapLetter(c1, c2) => {
                let pos1 = password.iter().position(|&x| x == c1).unwrap();
                let pos2 = password.iter().position(|&x| x == c2).unwrap();
                password.swap(pos1, pos2);
            }
            Operation::RotateLeft(count) => password.rotate_right(count),
            Operation::RotateRight(count) => password.rotate_left(count),
            Operation::RotatePosition(c) => {
                let pos = password.iter().position(|&x| x == c).unwrap();
                let count = [7, 7, 2, 6, 1, 5, 0, 4];
                password.rotate_right(count[pos]);
            }
            Operation::ReversePosition(pos1, pos2) => password[pos1..=pos2].reverse(),
            Operation::MovePosition(pos1, pos2) => {
                if pos1 <= pos2 {
                    password[pos1..=pos2].rotate_right(1);
                } else {
                    password[pos2..=pos1].rotate_left(1);
                }
            }
        }
    }
}

struct ParseRegex {
    regex_swap_position: Regex,
    regex_swap_letter: Regex,
    regex_rotate_left: Regex,
    regex_rotate_right: Regex,
    regex_rotate_position: Regex,
    regex_reverse_position: Regex,
    regex_move_position: Regex,
}

impl ParseRegex {
    fn parse(&self, line: &str) -> Result<Operation> {
        if let Some(cap) = self.regex_swap_position.captures(line) {
            Ok(Operation::SwapPosition(cap[1].parse()?, cap[2].parse()?))
        } else if let Some(cap) = self.regex_swap_letter.captures(line) {
            Ok(Operation::SwapLetter(cap[1].as_bytes()[0], cap[2].as_bytes()[0]))
        } else if let Some(cap) = self.regex_rotate_left.captures(line) {
            Ok(Operation::RotateLeft(cap[1].parse()?))
        } else if let Some(cap) = self.regex_rotate_right.captures(line) {
            Ok(Operation::RotateRight(cap[1].parse()?))
        } else if let Some(cap) = self.regex_rotate_position.captures(line) {
            Ok(Operation::RotatePosition(cap[1].as_bytes()[0]))
        } else if let Some(cap) = self.regex_reverse_position.captures(line) {
            Ok(Operation::ReversePosition(cap[1].parse()?, cap[2].parse()?))
        } else if let Some(cap) = self.regex_move_position.captures(line) {
            Ok(Operation::MovePosition(cap[1].parse()?, cap[2].parse()?))
        } else {
            Err(eyre!("unknown operation: {}", line))
        }
    }
}

fn main() -> Result<()> {
    let input = fs::read_to_string("inputs/2016-day21.txt")?;

    let parse_regex = ParseRegex {
        regex_swap_position: Regex::new(r#"^swap position (\d+) with position (\d+)$"#)?,
        regex_swap_letter: Regex::new(r#"^swap letter (\w) with letter (\w)$"#)?,
        regex_rotate_left: Regex::new(r#"^rotate left (\d+) steps?$"#)?,
        regex_rotate_right: Regex::new(r#"^rotate right (\d+) steps?$"#)?,
        regex_rotate_position: Regex::new(r#"^rotate based on position of letter (\w)$"#)?,
        regex_reverse_position: Regex::new(r#"^reverse positions (\d+) through (\d+)$"#)?,
        regex_move_position: Regex::new(r#"^move position (\d+) to position (\d+)$"#)?,
    };

    let operations = input.lines().map(|line| parse_regex.parse(line).unwrap()).collect_vec();

    let mut password = *b"abcdefgh";
    for operation in &operations {
        operation.execute(&mut password);
    }
    let result1 = String::from_utf8_lossy(&password);

    let mut password = *b"fbgdceah";
    for operation in operations.iter().rev() {
        operation.cancel(&mut password);
    }
    let result2 = String::from_utf8_lossy(&password);

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
