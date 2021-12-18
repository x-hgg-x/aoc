use aoc::*;

use itertools::Itertools;

fn merge((start1, end1): (u32, u32), (start2, end2): (u32, u32)) -> Option<(u32, u32)> {
    if end1 + 1 < start2 {
        None
    } else {
        Some((start1, end1.max(end2)))
    }
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let mut ranges: Vec<_> = input
        .lines()
        .map(|line| {
            let (left, right) = line.split_at(line.find('-').value()?);
            let start = left.parse::<u32>()?;
            let end = right[1..].parse::<u32>()?;
            Result::Ok((start, end))
        })
        .try_collect()?;

    ranges.sort_unstable();

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
