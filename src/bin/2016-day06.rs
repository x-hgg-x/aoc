use aoc::*;

use itertools::Itertools;

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let size = input.lines().next().value()?.len();

    let letters = input.lines().join("");

    let (result1, result2): (String, String) = (0..size)
        .map(|n| {
            let counts = letters.chars().skip(n).step_by(size).sorted_unstable().dedup_with_count().sorted_unstable().collect_vec();
            Ok((counts.last().value()?.1, counts.first().value()?.1))
        })
        .try_process(|iter| iter.unzip())?;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
