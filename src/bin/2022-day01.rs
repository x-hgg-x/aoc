use aoc::*;

use itertools::Itertools;

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let calories = input
        .split("\n\n")
        .flat_map(|group| {
            group
                .lines()
                .map(|x| Ok(x.parse::<u64>()?))
                .try_sum::<u64>()
        })
        .sorted_unstable()
        .collect_vec();

    let (c1, c2, c3) = calories.iter().rev().next_tuple().value()?;
    let result1 = c1;
    let result2 = c1 + c2 + c3;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
