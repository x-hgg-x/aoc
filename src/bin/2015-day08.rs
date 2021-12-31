use aoc::*;

use regex::Regex;

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let re = Regex::new(r#"(?m)(?:\\\\|\\"|\\x[0-9A-Fa-f]{2}|^"|"$)"#)?;

    let result1: i64 = re
        .find_iter(&input)
        .map(|x| match *x.as_str().as_bytes() {
            [b'"'] | [b'\\', b'\\'] | [b'\\', b'"'] => 1,
            [b'\\', b'x', ..] => 3,
            _ => 0,
        })
        .sum();

    let result2: usize = input.lines().map(|line| 2 + line.bytes().filter(|&x| x == b'"' || x == b'\\').count()).sum();

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
