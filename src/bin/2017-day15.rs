use eyre::Result;

use std::fs;

fn main() -> Result<()> {
    let input = fs::read_to_string("inputs/2017-day15.txt")?;

    let mut iter = input.lines().map(|line| line.split_ascii_whitespace().last().unwrap().parse::<u64>().unwrap());
    let (start_a, start_b) = (iter.next().unwrap(), iter.next().unwrap());

    let generator_a = std::iter::successors(Some(start_a), |a| Some((a * 16807) % 2147483647)).skip(1);
    let generator_b = std::iter::successors(Some(start_b), |b| Some((b * 48271) % 2147483647)).skip(1);

    let result1 = generator_a.clone().zip(generator_b.clone()).take(40_000_000).filter(|&(a, b)| a as u16 == b as u16).count();
    let result2 = generator_a.filter(|a| a % 4 == 0).zip(generator_b.filter(|b| b % 8 == 0)).take(5_000_000).filter(|&(a, b)| a as u16 == b as u16).count();

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
