use aoc::*;

use itertools::Itertools;

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let mut sum1 = 0;
    let mut sum2 = 0;
    let mut buf = Vec::new();

    for line in input.lines() {
        buf.clear();

        line.split_ascii_whitespace()
            .map(|x| Ok(x.parse::<i64>()?))
            .try_process(|iter| buf.extend(iter))?;

        let (min, max) = buf.iter().minmax().into_option().value()?;
        sum1 += max - min;

        sum2 += buf
            .iter()
            .copied()
            .tuple_combinations()
            .find_map(|(x, y)| {
                (x % y == 0)
                    .then_some(x / y)
                    .or_else(|| (y % x == 0).then_some(y / x))
            })
            .value()?;
    }

    let result1 = sum1;
    let result2 = sum2;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
