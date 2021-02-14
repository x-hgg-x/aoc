use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("inputs/2015-day1.txt")?;
    let input = input.trim();

    let iter = input.chars().map(|c| match c {
        '(' => 1,
        ')' => -1,
        _ => 0,
    });

    let result1: i32 = iter.clone().sum();

    let result2 = iter
        .scan(0, |state, x| {
            *state += x;
            Some(*state)
        })
        .position(|x| x == -1)
        .unwrap()
        + 1;

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
