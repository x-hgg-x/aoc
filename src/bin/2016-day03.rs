use itertools::Itertools;

use std::fs;

fn check_triangle(a: u32, b: u32, c: u32) -> bool {
    a + b > c && a + c > b && b + c > a
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("inputs/2016-day03.txt")?;

    let sides = input.split_ascii_whitespace().map(|x| x.parse::<u32>().unwrap()).collect_vec();

    let result1 = sides.chunks_exact(3).filter(|x| check_triangle(x[0], x[1], x[2])).count();
    let result2: usize = (0..3).map(|n| sides.iter().skip(n).step_by(3).tuples().filter(|&(a, b, c)| check_triangle(*a, *b, *c)).count()).sum();

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
