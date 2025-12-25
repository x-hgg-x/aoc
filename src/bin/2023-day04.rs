use aoc::*;

use itertools::Itertools;

use std::collections::{HashSet, VecDeque};

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let mut card_counts = VecDeque::from(vec![1; input.lines().count()]);

    let mut result1 = 0u64;
    let mut result2 = 0u64;

    for line in input.lines() {
        let card_count = card_counts.pop_front().value()?;

        let (winning, current) = line
            .split(':')
            .nth(1)
            .and_then(|line| line.split('|').next_tuple())
            .value()?;

        let winning: HashSet<_> = winning
            .split_ascii_whitespace()
            .map(|x| x.parse::<u64>())
            .try_collect()?;

        let current: HashSet<_> = current
            .split_ascii_whitespace()
            .map(|x| x.parse::<u64>())
            .try_collect()?;

        let matching = winning.intersection(&current).count();

        if matching > 0 {
            result1 += 2u64.pow(matching as u32 - 1);

            (card_counts.iter_mut().take(matching)).for_each(|x| *x += card_count);
        }

        result2 += card_count;
    }

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
