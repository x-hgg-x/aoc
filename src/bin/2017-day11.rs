use aoc::*;

use eyre::bail;

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);
    let input = input.trim();

    let mut q = 0i64;
    let mut r = 0i64;
    let mut distance = 0;
    let mut max_distance = 0;

    for direction in input.split(',') {
        match direction {
            "n" => r -= 1,
            "s" => r += 1,
            "se" => q += 1,
            "nw" => q -= 1,
            "sw" => {
                q -= 1;
                r += 1;
            }
            "ne" => {
                q += 1;
                r -= 1;
            }
            other => bail!("unknown direction: {other}"),
        }

        distance = (q.abs() + r.abs() + (q + r).abs()) / 2;
        max_distance = max_distance.max(distance);
    }

    let result1 = distance;
    let result2 = max_distance;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
