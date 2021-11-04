use eyre::Result;
use itertools::Itertools;

use std::fs;

fn main() -> Result<()> {
    let input = fs::read_to_string("inputs/2016-day06.txt")?;

    let size = input.lines().next().unwrap().len();

    let letters = input.lines().join("");

    let (result1, result2): (String, String) = (0..size)
        .map(|n| {
            let counts = letters.chars().skip(n).step_by(size).sorted_unstable().dedup_with_count().sorted_unstable().collect_vec();
            (counts.last().unwrap().1, counts.first().unwrap().1)
        })
        .unzip();

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
