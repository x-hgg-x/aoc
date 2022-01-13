use aoc::*;

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let n = input.trim().parse::<u64>()?;

    let zeros = n.leading_zeros();
    let result1 = (n << (zeros + 1) >> zeros) | 1;

    let p = 3u64.pow((n as f64 - 1.0).log(3.0).floor() as u32);
    let result2 = n - p + n.saturating_sub(2 * p);

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
