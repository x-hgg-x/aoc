use itertools::Itertools;
use regex::Regex;

use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("inputs/2015-day14.txt")?;

    let re = Regex::new(r#"fly (\d+) km/s for (\d+) seconds.*?rest for (\d+)"#).unwrap();

    const TIME: i32 = 2503;

    let reindeers = re
        .captures_iter(&input)
        .map(|cap| {
            let v_fly: i32 = cap[1].parse().unwrap();
            let t_fly: i32 = cap[2].parse().unwrap();
            let t_rest: i32 = cap[3].parse().unwrap();
            (v_fly, t_fly, t_rest)
        })
        .collect_vec();

    let race = (1..=TIME)
        .map(|time| {
            reindeers
                .iter()
                .map(|&(v_fly, t_fly, t_rest)| {
                    v_fly * (time / (t_fly + t_rest) * t_fly + t_fly.min(time % (t_fly + t_rest)))
                })
                .enumerate()
                .max_by_key(|&(_, d)| d)
                .unwrap()
        })
        .collect_vec();

    let result1 = race.last().unwrap().1;

    let result2 = race
        .iter()
        .map(|(pos, _)| pos)
        .sorted_unstable()
        .dedup_with_count()
        .map(|(count, _)| count)
        .max()
        .unwrap();

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
