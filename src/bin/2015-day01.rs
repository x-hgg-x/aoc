use itertools::Itertools;

use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("inputs/2015-day01.txt")?;
    let input = input.trim();

    let floors = input
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        })
        .collect_vec();

    let result1: i32 = floors.iter().sum();

    let result2 = 1 + floors
        .iter()
        .scan(0, |state, x| {
            *state += x;
            Some(*state)
        })
        .position(|x| x == -1)
        .unwrap();

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
