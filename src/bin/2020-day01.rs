use aoc::*;

use itertools::Itertools;

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let list: Vec<i64> = input
        .split_ascii_whitespace()
        .map(|x| x.parse())
        .try_collect()?;

    let result1 = list
        .iter()
        .tuple_combinations()
        .find_map(|(x, y)| (x + y == 2020).then(|| x * y))
        .value()?;

    let result2 = list
        .iter()
        .tuple_combinations()
        .find_map(|(x, y, z)| (x + y + z == 2020).then(|| x * y * z))
        .value()?;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
