use aoc::*;

use eyre::bail;
use itertools::Itertools;

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let seats = input
        .lines()
        .map(|line| {
            line.bytes()
                .rev()
                .enumerate()
                .map(|(index, x)| match x {
                    b'F' | b'L' => Ok(0),
                    b'B' | b'R' => Ok(1 << index),
                    _ => bail!("unknown seat region"),
                })
                .try_process(|iter| iter.sum::<u16>())
        })
        .try_process(|iter| iter.sorted_unstable().collect_vec())?;

    let result1 = seats.last().value()?;
    let result2 = seats.windows(2).find(|x| x[1] == x[0] + 2).map(|x| x[0] + 1).value()?;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
