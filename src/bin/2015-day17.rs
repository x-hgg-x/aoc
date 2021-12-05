use eyre::Result;
use itertools::Itertools;

use std::fs;

fn main() -> Result<()> {
    let input = fs::read_to_string("inputs/2015-day17.txt")?;

    let set = <Vec<u64>>::from_iter(input.split_ascii_whitespace().flat_map(|x| x.parse().ok()));
    let max: u64 = 1 << set.len();

    const SUM: u64 = 150;

    let combinations = (1..max)
        .scan((0, 0), |(sum, gray), index| {
            let new_gray = index ^ (index >> 1);
            let bit_changed = *gray ^ new_gray;
            let diff = set[bit_changed.trailing_zeros() as usize];
            if new_gray & bit_changed == 0 {
                *sum -= diff;
            } else {
                *sum += diff;
            }
            *gray = new_gray;

            Some((*sum, new_gray.count_ones()))
        })
        .filter(|&(sum, _)| sum == SUM)
        .map(|(_, size)| size)
        .collect_vec();

    let result1 = combinations.len();

    let min = combinations.iter().min().unwrap();
    let result2 = combinations.iter().filter(|&x| x == min).count();

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
