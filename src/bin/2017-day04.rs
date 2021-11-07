use eyre::Result;
use itertools::Itertools;

use std::fs;

fn main() -> Result<()> {
    let input = fs::read_to_string("inputs/2017-day04.txt")?;
    let input = input.trim();

    let mut sum1 = 0;
    let mut sum2 = 0;
    let mut buf = Vec::new();

    for line in input.lines() {
        buf.clear();
        buf.extend_from_slice(line.as_bytes());

        let mut words = buf.split_mut(|&x: &u8| x == b' ').collect_vec();

        sum1 += words.iter().tuple_combinations().all(|(x, y)| x != y) as usize;

        for word in &mut words {
            word.sort_unstable();
        }

        sum2 += words.iter().tuple_combinations().all(|(x, y)| x != y) as usize;
    }

    let result1 = sum1;
    let result2 = sum2;

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
