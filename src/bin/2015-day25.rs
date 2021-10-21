use regex::Regex;

use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("inputs/2015-day25.txt")?;

    let re = Regex::new(r#"row (\d+), column (\d+)"#).unwrap();
    let (row, column) = re.captures(&input).map(|cap| (cap[1].parse::<usize>().unwrap(), cap[2].parse::<usize>().unwrap())).unwrap();

    let sum = row - 1 + column - 1;
    let n = sum * (sum + 1) / 2 + column - 1;

    let mut generator = std::iter::successors(Some(20151125_u64), |number| Some((number * 252533) % 33554393));

    let result = generator.nth(n).unwrap();

    println!("{}", result);
    Ok(())
}
