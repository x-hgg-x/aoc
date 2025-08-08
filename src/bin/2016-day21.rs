use aoc::*;

use eyre::bail;
use itertools::Itertools;
use regex::{Regex, RegexSet};

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
    fn execute(&self, password: &mut [u8]) -> Result<()> {
        match *self {
            Operation::SwapPosition(pos1, pos2) => password.swap(pos1, pos2),
            Operation::SwapLetter(c1, c2) => {
                let pos1 = password.iter().position(|&x| x == c1).value()?;
                let pos2 = password.iter().position(|&x| x == c2).value()?;
                password.swap(pos1, pos2);
            }
            Operation::RotateLeft(count) => password.rotate_left(count),
            Operation::RotateRight(count) => password.rotate_right(count),
            Operation::RotatePosition(c) => {
                let pos = password.iter().position(|&x| x == c).value()?;
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

        Ok(())
    }

    fn cancel(&self, password: &mut [u8]) -> Result<()> {
        match *self {
            Operation::SwapPosition(pos1, pos2) => password.swap(pos1, pos2),
            Operation::SwapLetter(c1, c2) => {
                let pos1 = password.iter().position(|&x| x == c1).value()?;
                let pos2 = password.iter().position(|&x| x == c2).value()?;
                password.swap(pos1, pos2);
            }
            Operation::RotateLeft(count) => password.rotate_right(count),
            Operation::RotateRight(count) => password.rotate_left(count),
            Operation::RotatePosition(c) => {
                let pos = password.iter().position(|&x| x == c).value()?;
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

        Ok(())
    }
}

struct ParseRegex {
    set: RegexSet,
    regex_swap_position: Regex,
    regex_swap_letter: Regex,
    regex_rotate_left: Regex,
    regex_rotate_right: Regex,
    regex_rotate_position: Regex,
    regex_reverse_position: Regex,
    regex_move_position: Regex,
}

impl ParseRegex {
    const REGEX_SWAP_POSITION: usize = 0;
    const REGEX_SWAP_LETTER: usize = 1;
    const REGEX_ROTATE_LEFT: usize = 2;
    const REGEX_ROTATE_RIGHT: usize = 3;
    const REGEX_ROTATE_POSITION: usize = 4;
    const REGEX_REVERSE_POSITION: usize = 5;
    const REGEX_MOVE_POSITION: usize = 6;

    fn new(
        regex_swap_position: Regex,
        regex_swap_letter: Regex,
        regex_rotate_left: Regex,
        regex_rotate_right: Regex,
        regex_rotate_position: Regex,
        regex_reverse_position: Regex,
        regex_move_position: Regex,
    ) -> Result<Self> {
        Ok(Self {
            set: RegexSet::new([
                regex_swap_position.as_str(),
                regex_swap_letter.as_str(),
                regex_rotate_left.as_str(),
                regex_rotate_right.as_str(),
                regex_rotate_position.as_str(),
                regex_reverse_position.as_str(),
                regex_move_position.as_str(),
            ])?,
            regex_swap_position,
            regex_swap_letter,
            regex_rotate_left,
            regex_rotate_right,
            regex_rotate_position,
            regex_reverse_position,
            regex_move_position,
        })
    }

    fn parse(&self, line: &str) -> Result<Operation> {
        match self.set.matches(line).iter().next() {
            Some(Self::REGEX_SWAP_POSITION) => {
                let cap = self.regex_swap_position.captures(line).value()?;
                Ok(Operation::SwapPosition(cap[1].parse()?, cap[2].parse()?))
            }
            Some(Self::REGEX_SWAP_LETTER) => {
                let cap = self.regex_swap_letter.captures(line).value()?;
                Ok(Operation::SwapLetter(
                    cap[1].as_bytes()[0],
                    cap[2].as_bytes()[0],
                ))
            }
            Some(Self::REGEX_ROTATE_LEFT) => {
                let cap = self.regex_rotate_left.captures(line).value()?;
                Ok(Operation::RotateLeft(cap[1].parse()?))
            }
            Some(Self::REGEX_ROTATE_RIGHT) => {
                let cap = self.regex_rotate_right.captures(line).value()?;
                Ok(Operation::RotateRight(cap[1].parse()?))
            }
            Some(Self::REGEX_ROTATE_POSITION) => {
                let cap = self.regex_rotate_position.captures(line).value()?;
                Ok(Operation::RotatePosition(cap[1].as_bytes()[0]))
            }
            Some(Self::REGEX_REVERSE_POSITION) => {
                let cap = self.regex_reverse_position.captures(line).value()?;
                Ok(Operation::ReversePosition(cap[1].parse()?, cap[2].parse()?))
            }
            Some(Self::REGEX_MOVE_POSITION) => {
                let cap = self.regex_move_position.captures(line).value()?;
                Ok(Operation::MovePosition(cap[1].parse()?, cap[2].parse()?))
            }
            _ => bail!("unknown operation: {line}"),
        }
    }
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let parse_regex = ParseRegex::new(
        Regex::new(r#"^swap position (\d+) with position (\d+)$"#)?,
        Regex::new(r#"^swap letter (\w) with letter (\w)$"#)?,
        Regex::new(r#"^rotate left (\d+) steps?$"#)?,
        Regex::new(r#"^rotate right (\d+) steps?$"#)?,
        Regex::new(r#"^rotate based on position of letter (\w)$"#)?,
        Regex::new(r#"^reverse positions (\d+) through (\d+)$"#)?,
        Regex::new(r#"^move position (\d+) to position (\d+)$"#)?,
    )?;

    let operations: Vec<_> = input
        .lines()
        .map(|line| parse_regex.parse(line))
        .try_collect()?;

    let mut password = *b"abcdefgh";
    for operation in &operations {
        operation.execute(&mut password)?;
    }
    let result1 = String::from_utf8_lossy(&password);

    let mut password = *b"fbgdceah";
    for operation in operations.iter().rev() {
        operation.cancel(&mut password)?;
    }
    let result2 = String::from_utf8_lossy(&password);

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
