use aoc::*;

use itertools::Itertools;

fn check_triangle(a: u32, b: u32, c: u32) -> bool {
    a + b > c && a + c > b && b + c > a
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let sides: Vec<_> = input.split_ascii_whitespace().map(|x| x.parse::<u32>()).try_collect()?;

    let result1 = sides.chunks_exact(3).filter(|x| check_triangle(x[0], x[1], x[2])).count();
    let result2: usize = (0..3).map(|n| sides.iter().skip(n).step_by(3).tuples().filter(|&(a, b, c)| check_triangle(*a, *b, *c)).count()).sum();

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
