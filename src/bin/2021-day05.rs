use aoc::*;

use eyre::ensure;
use itertools::Itertools;
use regex::Regex;

type Line = ((i64, i64), (i64, i64));

fn count_overlap(iter: impl Iterator<Item = Line>) -> usize {
    iter.flat_map(|((x1, y1), (nx, ny))| {
        let n = nx.abs().max(ny.abs());
        let (sx, sy) = (nx.signum(), ny.signum());
        (0..=n).map(|i| (x1 + i * sx, y1 + i * sy)).collect_vec()
    })
    .sorted_unstable()
    .dedup_with_count()
    .filter(|&(count, _)| count > 1)
    .count()
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let re = Regex::new(r#"(?m)^(\d+),(\d+) -> (\d+),(\d+)$"#)?;

    let lines: Vec<Line> = re
        .captures_iter(&input)
        .map(|cap| {
            let x1 = cap[1].parse::<i64>()?;
            let y1 = cap[2].parse::<i64>()?;
            let x2 = cap[3].parse::<i64>()?;
            let y2 = cap[4].parse::<i64>()?;

            let (nx, ny) = (x2 - x1, y2 - y1);
            ensure!(nx * ny == 0 || nx.abs() == ny.abs(), "invalid input");

            Result::Ok(((x1, y1), (nx, ny)))
        })
        .try_collect()?;

    let result1 = count_overlap(lines.iter().copied().filter(|&(_, (nx, ny))| nx * ny == 0));
    let result2 = count_overlap(lines.iter().copied());

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
