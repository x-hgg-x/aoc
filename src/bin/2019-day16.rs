use aoc::*;

use eyre::ensure;
use itertools::Itertools;

use std::iter::{repeat, repeat_n};

fn compute_fft(initial_array: &[u8]) -> String {
    let mut array = initial_array.to_vec();
    let mut buffer = Vec::with_capacity(initial_array.len());

    let pattern: [i64; 4] = [0, 1, 0, -1];

    for _ in 0..100 {
        buffer.clear();

        buffer.extend((1..=array.len()).map(|repetition| {
            let pattern_iter = repeat(pattern.into_iter().flat_map(|y| repeat_n(y, repetition)))
                .flatten()
                .skip(1);

            let value = array
                .iter()
                .zip(pattern_iter)
                .map(|(&x, y)| x as i64 * y)
                .sum::<i64>()
                .abs();

            (value % 10) as u8
        }));

        std::mem::swap(&mut array, &mut buffer);
    }

    array.iter().map(|&x| (x + b'0') as char).take(8).collect()
}

fn compute_real_fft(initial_array: &[u8]) -> Result<String> {
    let start = (initial_array[..7].iter()).fold(0usize, |acc, &x| acc * 10 + x as usize);

    let end = initial_array.len() * 10000;

    ensure!(
        start >= initial_array.len() * 5000,
        "start must be greater than input half size"
    );

    let mut array = initial_array
        .iter()
        .copied()
        .cycle()
        .skip(start)
        .take(end - start)
        .collect_vec();

    for _ in 0..100 {
        let mut sum = array.iter().copied().map_into::<i64>().sum::<i64>();

        for x in &mut array {
            let v = *x as i64;
            *x = (sum % 10) as u8;
            sum -= v;
        }
    }

    Ok(array.iter().map(|&x| (x + b'0') as char).take(8).collect())
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let initial_array = input.trim().bytes().map(|x| x - b'0').collect_vec();

    let result1 = compute_fft(&initial_array);
    let result2 = compute_real_fft(&initial_array)?;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
