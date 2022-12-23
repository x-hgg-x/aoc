use aoc::*;

use itertools::Itertools;

use std::collections::VecDeque;

fn step(counts: &mut VecDeque<u64>, n: usize) -> Result<()> {
    for _ in 0..n {
        counts.rotate_left(1);
        counts[6] += *counts.back().value()?;
    }
    Ok(())
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);
    let input = input.trim();

    let mut list: Vec<_> = input.split(',').map(|x| x.parse()).try_collect()?;
    list.sort_unstable();

    let mut counts = VecDeque::from([0u64; 9]);
    for (count, timer) in list.into_iter().dedup_with_count() {
        counts[timer] += count as u64;
    }

    step(&mut counts, 80)?;
    let result1 = counts.iter().sum::<u64>();

    step(&mut counts, 176)?;
    let result2 = counts.iter().sum::<u64>();

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
