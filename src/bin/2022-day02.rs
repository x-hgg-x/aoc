use aoc::*;

use eyre::bail;
use itertools::Itertools;

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let instructions: Vec<_> = input
        .lines()
        .map(|line| match line.as_bytes() {
            &[c1, b' ', c2] => Ok(((c1 - b'A') as i64, (c2 - b'X') as i64)),
            _ => bail!("invalid line: {line:?}"),
        })
        .try_collect()?;

    let result1 = instructions.iter().map(|(x, y)| 3 * (y - x + 1).rem_euclid(3) + y + 1).sum::<i64>();
    let result2 = instructions.iter().map(|(x, y)| (x - 1 + y).rem_euclid(3) + 3 * y + 1).sum::<i64>();

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
