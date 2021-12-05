use eyre::Result;
use itertools::Itertools;
use num_complex::{self, Complex};
use regex::Regex;

use std::fs;

fn main() -> Result<()> {
    let input = fs::read_to_string("inputs/2016-day01.txt")?;

    let re = Regex::new(r#"([RL])(\d+)"#)?;

    let blocks = re
        .captures_iter(&input)
        .map(|cap| (cap.get(1).unwrap().as_str(), cap[2].parse::<i64>().unwrap()))
        .scan((Complex::new(0, 1), Complex::new(0, 0)), |(direction, block), (turn, step)| {
            *direction *= match turn {
                "R" => Complex::new(0, -1),
                "L" => Complex::new(0, 1),
                _ => Complex::new(1, 0),
            };

            let intermediate_blocks = (1..=step).map(|i| *block + Complex::new(i, 0) * *direction).collect_vec();

            *block = *intermediate_blocks.last().unwrap();

            Some(intermediate_blocks)
        })
        .flatten()
        .collect_vec();

    let result1 = blocks.last().unwrap().l1_norm();

    let result2 = blocks
        .iter()
        .enumerate()
        .sorted_unstable_by_key(|&(_, block)| (block.re, block.im))
        .dedup_by_with_count(|(_, block1), (_, block2)| block1 == block2)
        .filter(|&(count, _)| count > 1)
        .min_by_key(|&(_, (pos, _))| pos)
        .map(|(_, (_, block))| block.l1_norm())
        .unwrap();

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
