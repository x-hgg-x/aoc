use aoc::*;

use eyre::ensure;
use itertools::Itertools;
use regex::Regex;

use std::collections::HashMap;

fn gcd(mut x: u64, mut y: u64) -> u64 {
    while y != 0 {
        (x, y) = (y, x % y);
    }
    x
}

fn lcm(x: u64, y: u64) -> u64 {
    x * y / gcd(x, y)
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let re = Regex::new(r#"(?m)^(\w+) = \((\w+), (\w+)\)$"#)?;

    let instructions = (input.lines().next().value()?.trim().bytes())
        .filter_map(|x| match x {
            b'L' => Some(0u8),
            b'R' => Some(1u8),
            _ => None,
        })
        .collect_vec();

    let nodes: HashMap<_, _> = re
        .captures_iter(&input)
        .map(|cap| {
            let name = cap.get(1).value()?.as_str();
            let left = cap.get(2).value()?.as_str();
            let right = cap.get(3).value()?.as_str();
            Result::Ok((name, [left, right]))
        })
        .try_collect()?;

    let result1 = (instructions.iter().cycle())
        .scan((0u64, "AAA"), |state, &instruction| {
            state.0 += 1;
            state.1 = nodes[state.1][instruction as usize];
            Some(*state)
        })
        .take_while_inclusive(|&(_, node)| node != "ZZZ")
        .last()
        .map(|(steps, _)| steps)
        .value()?;

    let result2 = nodes
        .keys()
        .filter(|name| name.ends_with('A'))
        .map(|&a_node| {
            let mut initial_steps = 0u64;
            let mut cycle_size = None;
            let mut current_node = a_node;

            for &instruction in instructions.iter().cycle() {
                current_node = nodes[current_node][instruction as usize];
                if let Some(cycle_size) = &mut cycle_size {
                    *cycle_size += 1;
                } else {
                    initial_steps += 1;
                }
                if current_node.ends_with('Z') {
                    if cycle_size.is_some() {
                        break;
                    } else {
                        cycle_size = Some(0);
                    }
                }
            }

            let cycle_size = cycle_size.value()?;

            ensure!(cycle_size == initial_steps, "unsupported input");

            Ok(cycle_size)
        })
        .try_process(|iter| iter.fold(1, lcm))?;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
