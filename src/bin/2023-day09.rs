use aoc::*;

use itertools::Itertools;

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let sequences: Vec<Vec<i64>> = input
        .lines()
        .map(|line| (line.split_ascii_whitespace().map(|x| x.parse())).try_collect())
        .try_collect()?;

    let mut current_buffer = Vec::new();
    let mut new_buffer = Vec::new();
    let mut first_values = Vec::new();

    let mut last_values_sum = 0i64;
    let mut first_values_sum = 0i64;

    for sequence in sequences.iter() {
        first_values.clear();

        current_buffer.clear();
        current_buffer.extend_from_slice(sequence);

        while !current_buffer.iter().all(|&x| x == 0) {
            last_values_sum += current_buffer.last().value()?;
            first_values.push(*current_buffer.first().value()?);

            new_buffer.clear();
            new_buffer.extend(current_buffer.windows(2).map(|x| x[1] - x[0]));

            std::mem::swap(&mut new_buffer, &mut current_buffer);
        }

        first_values_sum += first_values.iter().rev().fold(0, |acc, x| x - acc);
    }

    let result1 = last_values_sum;
    let result2 = first_values_sum;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
