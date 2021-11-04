use eyre::Result;
use itertools::Itertools;

use std::fs;
use std::iter::once;

fn count_safe_tiles(input: &[u8], row_count: usize) -> usize {
    let mut line = input.iter().map(|&x| (x == b'.') as usize).collect_vec();
    let mut sum = line.iter().sum();

    let mut buf = Vec::with_capacity(line.len());
    for _ in 0..(row_count - 1) {
        buf.clear();
        buf.extend(once(line[1]).chain(line.windows(3).map(|x| (x[0] ^ x[2]) ^ 1)).chain(once(line[line.len() - 2])));
        std::mem::swap(&mut buf, &mut line);
        sum += line.iter().sum::<usize>();
    }
    sum
}

fn main() -> Result<()> {
    let input = fs::read_to_string("inputs/2016-day18.txt")?;
    let input = input.trim().as_bytes();

    assert!(input.len() >= 2, "invalid input");

    let result1 = count_safe_tiles(input, 40);
    let result2 = count_safe_tiles(input, 400000);

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
