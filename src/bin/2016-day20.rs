use eyre::Result;
use itertools::Itertools;

use std::fs;

fn merge((start1, end1): (u32, u32), (start2, end2): (u32, u32)) -> Option<(u32, u32)> {
    if end1 + 1 < start2 {
        None
    } else {
        Some((start1, end1.max(end2)))
    }
}

fn main() -> Result<()> {
    let input = fs::read_to_string("inputs/2016-day20.txt")?;

    let ranges = input
        .lines()
        .map(|line| {
            let (left, right) = line.split_at(line.find('-').unwrap());
            let start = left.parse::<u32>().unwrap();
            let end = right[1..].parse::<u32>().unwrap();
            (start, end)
        })
        .sorted_unstable()
        .collect_vec();

    let mut forbidden_ranges = Vec::new();
    let mut current_forbidden_range = ranges[0];

    for &range in &ranges[1..] {
        match merge(current_forbidden_range, range) {
            Some(new_forbidden_range) => current_forbidden_range = new_forbidden_range,
            None => {
                forbidden_ranges.push(current_forbidden_range);
                current_forbidden_range = range;
            }
        }
    }

    let result1 = forbidden_ranges[0].1 + 1;
    let result2 = forbidden_ranges.windows(2).map(|x| x[1].0 - x[0].1 - 1).sum::<u32>();

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
