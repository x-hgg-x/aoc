use aoc::*;

use eyre::bail;
use itertools::{Itertools, MinMaxResult};

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let mut sum1 = 0;
    let mut sum2 = 0;
    let mut buf = Vec::new();

    for line in input.lines() {
        buf.clear();
        line.split_ascii_whitespace().map(|x| Ok(x.parse::<i64>()?)).try_process(|iter| buf.extend(iter))?;

        sum1 += match buf.iter().minmax() {
            MinMaxResult::OneElement(_) => 0,
            MinMaxResult::MinMax(min, max) => max - min,
            MinMaxResult::NoElements => bail!("empty_line"),
        };

        sum2 += buf.iter().copied().tuple_combinations().find_map(|(x, y)| (x % y == 0).then(|| x / y).or_else(|| (y % x == 0).then(|| y / x))).value()?;
    }

    let result1 = sum1;
    let result2 = sum2;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
