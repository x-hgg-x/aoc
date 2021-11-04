use eyre::Result;
use itertools::Itertools;
use regex::Regex;

use std::fs;

fn main() -> Result<()> {
    let input = fs::read_to_string("inputs/2016-day01.txt")?;

    let re = Regex::new(r#"([RL])(\d+)"#)?;

    let blocks = re
        .captures_iter(&input)
        .map(|cap| (cap.get(1).unwrap().as_str(), cap[2].parse::<i32>().unwrap()))
        .scan(((0, 1), (0, 0)), |state, (turn, step)| {
            let direction = &mut state.0;
            let block = &mut state.1;

            *direction = match turn {
                "R" => (direction.1, -direction.0),
                "L" => (-direction.1, direction.0),
                _ => *direction,
            };

            let intermediate_blocks = (1..=step).map(|i| (block.0 + i * direction.0, block.1 + i * direction.1)).collect_vec();

            *block = *intermediate_blocks.last().unwrap();

            Some(intermediate_blocks)
        })
        .flatten()
        .collect_vec();

    let result1 = blocks.last().map(|(x, y)| x.abs() + y.abs()).unwrap();

    let result2 = blocks
        .iter()
        .enumerate()
        .sorted_unstable_by_key(|&(_, block)| block)
        .dedup_by_with_count(|(_, block1), (_, block2)| block1 == block2)
        .filter(|&(count, _)| count > 1)
        .min_by_key(|&(_, (pos, _))| pos)
        .map(|(_, (_, (x, y)))| x.abs() + y.abs())
        .unwrap();

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
