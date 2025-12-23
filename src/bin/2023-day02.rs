use aoc::*;

use eyre::bail;
use itertools::Itertools;

use std::iter;

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let mut result1 = 0u64;
    let mut result2 = 0u64;

    for line in input.lines() {
        let (game, data) = line.split(": ").next_tuple().value()?;
        let id = (game.split_ascii_whitespace().nth(1).value()?).parse::<u64>()?;

        let mut is_possible = true;
        let mut max_rgb = [0u8; 3];

        for set in data.split("; ") {
            let mut rgb = [0u8; 3];

            for cubes in set.split(", ") {
                let (number, kind) = cubes.split(" ").next_tuple().value()?;
                let number = number.parse()?;
                match kind {
                    "red" => rgb[0] = number,
                    "green" => rgb[1] = number,
                    "blue" => rgb[2] = number,
                    _ => bail!("unknown cube kind: {kind}"),
                }
            }

            if !iter::zip(rgb, [12, 13, 14]).all(|(n, max)| n <= max) {
                is_possible = false;
            }

            for (n, max) in iter::zip(rgb, &mut max_rgb) {
                *max = n.max(*max);
            }
        }

        if is_possible {
            result1 += id;
        }

        result2 += max_rgb.iter().map(|x| *x as u64).product::<u64>();
    }

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
