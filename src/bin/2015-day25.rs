use regex::Regex;

use std::fs;

struct Generator {
    number: u64,
}

impl Generator {
    fn new() -> Self {
        Self { number: 20151125 }
    }
}

impl Iterator for Generator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.number = (self.number * 252533) % 33554393;
        Some(self.number)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("inputs/2015-day25.txt")?;

    let re = Regex::new(r#"row (\d+), column (\d+)"#).unwrap();

    let (row, column) = re
        .captures(&input)
        .map(|cap| {
            (
                cap[1].parse::<usize>().unwrap(),
                cap[2].parse::<usize>().unwrap(),
            )
        })
        .unwrap();

    let sum = row - 1 + column - 1;
    let n = sum * (sum + 1) / 2 + column - 1;

    let result = Generator::new().nth(n - 1).unwrap();

    println!("{}", result);
    Ok(())
}
