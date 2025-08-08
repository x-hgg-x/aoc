use aoc::*;

use itertools::Itertools;
use num_complex::Complex;
use regex::Regex;

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let re = Regex::new(r#"([RL])(\d+)"#)?;

    let blocks = re
        .captures_iter(&input)
        .map(|cap| Ok((cap.get(1).value()?.as_str(), cap[2].parse::<usize>()?)))
        .try_process(|iter| {
            iter.scan(
                (Complex::new(0, 1), Complex::new(0, 0)),
                |(direction, block), (turn, step)| {
                    let new_direction = match turn {
                        "R" => *direction * Complex::new(0, -1),
                        "L" => *direction * Complex::new(0, 1),
                        _ => *direction,
                    };

                    let current_block = *block;

                    let intermediate_blocks =
                        (1..=step).map(move |i| current_block + i as i64 * new_direction);

                    *direction = new_direction;
                    *block += step as i64 * new_direction;

                    Some(intermediate_blocks)
                },
            )
            .flatten()
            .collect_vec()
        })?;

    let result1 = blocks.last().value()?.l1_norm();

    let result2 = blocks
        .iter()
        .enumerate()
        .sorted_unstable_by_key(|&(_, block)| (block.re, block.im))
        .dedup_by_with_count(|(_, block1), (_, block2)| block1 == block2)
        .filter(|&(count, _)| count > 1)
        .min_by_key(|&(_, (pos, _))| pos)
        .map(|(_, (_, block))| block.l1_norm())
        .value()?;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
