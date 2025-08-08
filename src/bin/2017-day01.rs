use aoc::*;

fn count(input: &[u8], shift: usize) -> u64 {
    input
        .iter()
        .zip(input.iter().cycle().skip(shift))
        .filter(|&(x, y)| x == y)
        .map(|(x, _)| (x - b'0') as u64)
        .sum()
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);
    let input = input.trim().as_bytes();

    let result1 = count(input, 1);
    let result2 = count(input, input.len() / 2);

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
