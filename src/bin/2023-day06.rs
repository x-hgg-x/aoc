use aoc::*;

use itertools::Itertools;
use regex::Regex;

use std::iter;

fn parse_list(s: &str) -> Result<Vec<u64>> {
    s.trim()
        .split_ascii_whitespace()
        .map(|x| Ok(x.parse::<u64>()?))
        .try_collect()
}

fn parse_single(s: &str) -> u64 {
    s.bytes()
        .rev()
        .filter(|x| x.is_ascii_digit())
        .enumerate()
        .map(|(pos, digit)| 10u64.pow(pos as u32) * (digit - b'0') as u64)
        .sum()
}

fn compute_ways_count(times: &[u64], distances: &[u64]) -> u64 {
    iter::zip(times, distances)
        .map(|(&time, &distance)| {
            let Some(delta) = (time * time).checked_sub(4 * distance) else {
                return 0;
            };

            let offset = (time - delta.isqrt() - 1) / 2;

            (offset..)
                .position(|x| x * (time - x) > distance)
                .map(|x| time - 1 - 2 * (offset + x as u64 - 1))
                .unwrap_or(0)
        })
        .product()
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let re = Regex::new("(?s)Time:(.+)Distance:(.+)")?;

    let cap = re.captures(&input).value()?;

    let times = parse_list(&cap[1])?;
    let distances = parse_list(&cap[2])?;

    let concatenated_times = parse_single(&cap[1]);
    let concatenated_distances = parse_single(&cap[2]);

    let result1 = compute_ways_count(&times, &distances);
    let result2 = compute_ways_count(&[concatenated_times], &[concatenated_distances]);

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
