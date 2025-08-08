use aoc::*;

use itertools::Itertools;

fn run(n: i64) -> i64 {
    let n_sqrt_f = (n as f64).sqrt();
    let n_sqrt = n_sqrt_f as i64;

    let sum = (1..=n_sqrt)
        .filter(|&d| n % d == 0)
        .map(|d| d + n / d)
        .sum::<i64>();

    match n_sqrt_f.fract() == 0.0 {
        true => sum - n_sqrt,
        false => sum,
    }
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);
    let input = input.lines().collect_vec();

    let a = input[22]
        .split_ascii_whitespace()
        .nth(2)
        .value()?
        .parse::<i64>()?;

    let b = input[24]
        .split_ascii_whitespace()
        .nth(2)
        .value()?
        .parse::<i64>()?;

    let n1 = 836 + 22 * a + b;
    let n2 = n1 + 10_550_400;

    let result1 = run(n1);
    let result2 = run(n2);

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
