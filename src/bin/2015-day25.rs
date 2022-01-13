use aoc::*;

use regex::Regex;

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let re = Regex::new(r#"row (\d+), column (\d+)"#)?;
    let cap = re.captures(&input).value()?;
    let row: usize = cap[1].parse()?;
    let column: usize = cap[2].parse()?;

    let sum = row - 1 + column - 1;
    let n = sum * (sum + 1) / 2 + column - 1;

    let mut generator = std::iter::successors(Some(20151125_u64), |number| Some((number * 252533) % 33554393));

    let result = generator.nth(n).value()?;

    println!("{result}");
    Ok(())
}
