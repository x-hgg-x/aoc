use itertools::Itertools;

use std::fs;

fn apply(input: String) -> String {
    input
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .dedup_with_count()
        .map(|(count, digit)| format!("{}{}", count, digit))
        .collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("inputs/2015-day10.txt")?;
    let input = input.trim();

    let mut data = input.to_owned();
    for _ in 0..40 {
        data = apply(data);
    }
    let result1 = data.len();

    for _ in 0..10 {
        data = apply(data);
    }
    let result2 = data.len();

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
