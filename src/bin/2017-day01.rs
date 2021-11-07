use eyre::Result;

use std::fs;

fn count(input: &[u8], shift: usize) -> u64 {
    input.iter().zip(input.iter().cycle().skip(shift)).filter_map(|(x, y)| (x == y).then(|| (x - b'0') as u64)).sum()
}

fn main() -> Result<()> {
    let input = fs::read_to_string("inputs/2017-day01.txt")?;
    let input = input.trim().as_bytes();

    let result1 = count(input, 1);
    let result2 = count(input, input.len() / 2);

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
