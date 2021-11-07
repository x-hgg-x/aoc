use eyre::{bail, Result};
use itertools::{Itertools, MinMaxResult};

use std::fs;

fn main() -> Result<()> {
    let input = fs::read_to_string("inputs/2017-day02.txt")?;

    let mut sum1 = 0;
    let mut sum2 = 0;
    let mut buf = Vec::new();

    for line in input.lines() {
        buf.clear();
        buf.extend(line.split_ascii_whitespace().map(|x| x.parse::<i64>().unwrap()));

        sum1 += match buf.iter().minmax() {
            MinMaxResult::OneElement(_) => 0,
            MinMaxResult::MinMax(min, max) => max - min,
            MinMaxResult::NoElements => bail!("empty_line"),
        };

        sum2 += buf.iter().copied().tuple_combinations().find_map(|(x, y)| (x % y == 0).then(|| x / y).or_else(|| (y % x == 0).then(|| y / x))).unwrap();
    }

    let result1 = sum1;
    let result2 = sum2;

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
