use eyre::Result;
use regex::Regex;

use std::fs;

fn main() -> Result<()> {
    let input = fs::read_to_string("inputs/2015-day08.txt")?;

    let re = Regex::new(r#"(?m)(?:\\\\|\\"|\\x[0-9A-Fa-f]{2}|^"|"$)"#)?;

    let result1: i32 = re
        .find_iter(&input)
        .map(|x| match *x.as_str().as_bytes() {
            [b'"'] | [b'\\', b'\\'] | [b'\\', b'"'] => 1,
            [b'\\', b'x', _, _] => 3,
            _ => 0,
        })
        .sum();

    let result2: usize = input.lines().map(|line| 2 + line.bytes().filter(|&x| x == b'"' || x == b'\\').count()).sum();

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
