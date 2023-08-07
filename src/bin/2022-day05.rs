use aoc::*;

use itertools::Itertools;
use regex::Regex;

use std::iter;

fn move_crates(stacks: &mut [Vec<u8>], n: usize, from: usize, to: usize, reverse: bool) -> Result<()> {
    let mut to_stack = std::mem::take(stacks.get_mut(to).value()?);
    let from_stack = stacks.get_mut(from).value()?;

    let drain_iter = from_stack.drain(from_stack.len() - n..);

    if reverse {
        to_stack.extend(drain_iter.rev());
    } else {
        to_stack.extend(drain_iter);
    }

    *stacks.get_mut(to).value()? = to_stack;

    Ok(())
}

fn top(stacks: &[Vec<u8>]) -> String {
    stacks.iter().flat_map(|stack| stack.iter().last()).map(|&x| x as char).collect()
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let (input_crates, input_moves) = input.split("\n\n").next_tuple().value()?;

    let size = input_crates.split_ascii_whitespace().next_back().value()?.parse()?;
    let mut stacks_1 = vec![vec![]; size];

    for line in input_crates.lines() {
        if line.starts_with('[') || line.starts_with("   ") {
            for (stack, x) in iter::zip(&mut stacks_1, line.as_bytes().chunks(4)) {
                if let [b'[', c, b']', ..] = *x {
                    stack.push(c);
                }
            }
        }
    }

    for stack in &mut stacks_1 {
        stack.reverse();
    }

    let mut stacks_n = stacks_1.clone();

    let regex_moves = Regex::new(r#"(?m)^move (\d+) from (\d+) to (\d+)$"#)?;

    for cap in regex_moves.captures_iter(input_moves) {
        let n = cap[1].parse::<usize>()?;
        let from = cap[2].parse::<usize>()? - 1;
        let to = cap[3].parse::<usize>()? - 1;

        move_crates(&mut stacks_1, n, from, to, true)?;
        move_crates(&mut stacks_n, n, from, to, false)?;
    }

    let result1 = top(&stacks_1);
    let result2 = top(&stacks_n);

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
