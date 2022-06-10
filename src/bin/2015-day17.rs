use aoc::*;

use itertools::Itertools;

const SUM: u64 = 150;

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let set = <Vec<u64>>::from_iter(input.split_ascii_whitespace().flat_map(|x| x.parse().ok()));
    let max: u64 = 1 << set.len();

    let combinations = (1..max)
        .scan((0, 0), |(sum, gray), index| {
            let new_gray = index ^ (index >> 1);
            let bit_changed = *gray ^ new_gray;
            *gray = new_gray;

            let diff = set[bit_changed.trailing_zeros() as usize];
            if new_gray & bit_changed == 0 {
                *sum -= diff;
            } else {
                *sum += diff;
            }

            Some((*sum, new_gray.count_ones()))
        })
        .filter(|&(sum, _)| sum == SUM)
        .map(|(_, size)| size)
        .collect_vec();

    let result1 = combinations.len();

    let min = combinations.iter().min().value()?;
    let result2 = combinations.iter().filter(|&x| x == min).count();

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
