use aoc::*;

use itertools::Itertools;

fn mix(numbers: &[i64], n: usize) -> Result<i64> {
    let mut indices = (0..numbers.len()).collect_vec();

    for _ in 0..n {
        for (index, &number) in numbers.iter().enumerate() {
            let index_index = indices.iter().position(|&x| x == index).value()?;

            indices.remove(index_index);

            indices.insert(
                (index_index as i64 + number).rem_euclid(indices.len() as i64) as usize,
                index,
            );
        }
    }

    let zero_index = numbers.iter().position(|&x| x == 0).value()?;
    let zero_index_index = indices.iter().position(|&x| x == zero_index).value()?;

    Ok([1000, 2000, 3000]
        .iter()
        .map(|offset| numbers[indices[(zero_index_index + offset) % indices.len()]])
        .sum())
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let mut numbers: Vec<i64> = input
        .split_ascii_whitespace()
        .map(|x| x.parse())
        .try_collect()?;

    let result1 = mix(&numbers, 1)?;

    numbers.iter_mut().for_each(|x| *x *= 811_589_153);
    let result2 = mix(&numbers, 10)?;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
