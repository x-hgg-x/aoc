use eyre::Result;
use itertools::Itertools;

use std::fs;

fn main() -> Result<()> {
    let input = fs::read("inputs/2015-day01.txt")?;

    let floors = input
        .iter()
        .filter_map(|&x| match x {
            b'(' => Some(1),
            b')' => Some(-1),
            _ => None,
        })
        .collect_vec();

    let result1 = floors.iter().sum::<i64>();

    let result2 = 1 + floors
        .iter()
        .scan(0, |position, x| {
            *position += x;
            Some(*position)
        })
        .position(|x| x == -1)
        .unwrap();

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
