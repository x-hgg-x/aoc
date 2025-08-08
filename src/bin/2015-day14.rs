use aoc::*;

use itertools::Itertools;
use regex::Regex;

const TIME: i64 = 2503;

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let re = Regex::new(r#"fly (\d+) km/s for (\d+) seconds.*?rest for (\d+)"#)?;

    let reindeers: Vec<_> = re
        .captures_iter(&input)
        .map(|cap| {
            let v_fly: i64 = cap[1].parse()?;
            let t_fly: i64 = cap[2].parse()?;
            let t_rest: i64 = cap[3].parse()?;
            Result::Ok((v_fly, t_fly, t_rest))
        })
        .try_collect()?;

    let race: Vec<_> = (1..=TIME)
        .map(|time| {
            reindeers
                .iter()
                .map(|&(v_fly, t_fly, t_rest)| {
                    v_fly * (time / (t_fly + t_rest) * t_fly + t_fly.min(time % (t_fly + t_rest)))
                })
                .enumerate()
                .max_by_key(|&(_, d)| d)
                .value()
        })
        .try_collect()?;

    let result1 = race.last().value()?.1;

    let result2 = race
        .iter()
        .map(|(pos, _)| pos)
        .sorted_unstable()
        .dedup_with_count()
        .map(|(count, _)| count)
        .max()
        .value()?;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
