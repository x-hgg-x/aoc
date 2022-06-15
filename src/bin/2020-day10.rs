use aoc::*;

use itertools::Itertools;

use std::iter::{once, repeat};

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let mut joltages: Vec<usize> = input.split_ascii_whitespace().map(|x| x.parse()).try_collect()?;
    joltages.push(0);
    joltages.sort_unstable();
    joltages.dedup();
    joltages.push(joltages.last().value()? + 3);

    let mut diff_1_count = 0;
    let mut diff_3_count = 0;

    for x in joltages.windows(2) {
        match x[1] - x[0] {
            1 => diff_1_count += 1,
            3 => diff_3_count += 1,
            _ => (),
        }
    }

    let result1 = diff_1_count * diff_3_count;

    let mut counts = once(1u64).chain(repeat(0).take(*joltages.last().value()?)).collect_vec();
    for &joltage in &joltages[1..] {
        counts[joltage] = counts[joltage.saturating_sub(3)..joltage].iter().sum();
    }

    let result2 = *counts.last().value()?;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
