use aoc::*;

use itertools::Itertools;

use std::collections::HashSet;

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let list: Vec<_> = input.split_ascii_whitespace().map(|x| x.parse::<i64>()).try_collect()?;
    let result1 = list.iter().sum::<i64>();

    let mut previous_sums = HashSet::new();
    let mut cumulative_sum = 0;
    let mut iter = list.iter().cycle();

    let result2 = loop {
        if let Some(&value) = iter.next() {
            cumulative_sum += value;
            if !previous_sums.insert(cumulative_sum) {
                break cumulative_sum;
            }
        }
    };

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
