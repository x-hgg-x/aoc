use regex::Regex;

use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("inputs/2015-day08.txt")?;

    let re = Regex::new(r#"(?m)(?:\\\\|\\"|\\x[0-9A-Fa-f]{2}|^"|"$)"#).unwrap();

    let result1: i32 = re
        .find_iter(&input)
        .map(|x| match x.as_str().as_bytes() {
            br#"""# => 1,
            br#"\\"# => 1,
            br#"\""# => 1,
            &[b'\\', b'x', _, _] => 3,
            _ => 0,
        })
        .sum();

    let result2: usize = input
        .lines()
        .map(|line| 2 + line.chars().filter(|&c| c == '"' || c == '\\').count())
        .sum();

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
