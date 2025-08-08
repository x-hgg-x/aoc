use aoc::*;

use eyre::ensure;
use itertools::Itertools;

fn bit_criteria(values: &[u16], width: usize, f: impl Fn(usize, usize) -> bool) -> u64 {
    let mut remaining = values;

    for bit in (0..width).rev() {
        let index = remaining.partition_point(|x| x & (1 << bit) == 0);

        if f(index, remaining.len()) {
            remaining = &remaining[index..];
        } else {
            remaining = &remaining[..index];
        }

        if remaining.len() < 2 {
            break;
        }
    }

    remaining[0] as u64
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let lines = input.lines().sorted_unstable().collect_vec();

    let width = input.lines().next().value()?.len();
    let len = lines.len() as u64;

    let mut counts = vec![0; width];
    let mut values = Vec::with_capacity(lines.len());

    for line in &lines {
        ensure!(line.len() == width, "invalid input");
        let mut value = 0;
        for (index, (x, count)) in line.bytes().zip(&mut counts).rev().enumerate() {
            let bit = x - b'0';
            *count += bit as u64;
            value += (bit as u16) << index;
        }
        values.push(value);
    }

    let gamma_rate = counts
        .iter()
        .rev()
        .enumerate()
        .map(|(index, &x)| ((x > len / 2) as u64) << index)
        .sum::<u64>();

    let epsilon_rate = (!gamma_rate) & ((1 << width) - 1);
    let result1 = gamma_rate * epsilon_rate;

    let oxygen_rating = bit_criteria(&values, width, |index, len| index <= len - index);
    let co2_rating = bit_criteria(&values, width, |index, len| index > len - index);
    let result2 = oxygen_rating * co2_rating;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
