use aoc::*;

use eyre::bail;
use itertools::Itertools;
use regex::Regex;

use std::cmp::Reverse;
use std::collections::HashMap;

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let regex_line = Regex::new(r#"^\[\d+\-\d+\-\d+ \d+:(\d+)\] (.+)$"#)?;
    let regex_guard = Regex::new(r#"^\[\d+\-\d+\-\d+ \d+:\d+\] Guard #(\d+) begins shift$"#)?;

    let lines = input.lines().sorted_unstable_by_key(|&line| Reverse(line)).collect_vec();

    let mut guards = HashMap::<_, Vec<_>>::new();

    for group in lines.split_inclusive(|&line| line.contains("begins shift")) {
        let mut group_iter = group.iter().rev();

        let cap_guard = regex_guard.captures(group_iter.next().value()?).value()?;
        let guard_id = cap_guard[1].parse::<usize>()?;

        let mut minutes_asleep = 0u64;
        let mut asleep_start = None;

        for &line in group_iter {
            let cap = regex_line.captures(line).value()?;

            match &cap[2] {
                "falls asleep" if asleep_start.is_none() => asleep_start = Some(cap[1].parse::<u64>()?),
                "wakes up" => match asleep_start {
                    Some(start) => {
                        let end = cap[1].parse::<u64>()?;
                        minutes_asleep += ((1 << start) - 1) ^ ((1 << end) - 1);
                        asleep_start = None;
                    }
                    _ => bail!("incorrect shift"),
                },
                _ => bail!("incorrect shift"),
            };
        }

        guards.entry(guard_id).or_default().push(minutes_asleep);
    }

    let max_minutes: Vec<_> = guards
        .iter()
        .map(|(&id, v)| {
            let mut minutes_count = [0u64; 60];
            for minutes_asleep in v {
                for (minute_bit, count) in minutes_count.iter_mut().enumerate() {
                    *count += (minutes_asleep >> minute_bit) & 1;
                }
            }
            let (minute, count) = minutes_count.into_iter().enumerate().max_by_key(|&(_, count)| count).value()?;

            Result::Ok((id, minute, count))
        })
        .try_collect()?;

    let id_part_1 = guards.iter().max_by_key(|&(_, v)| v.iter().map(|&x| x.count_ones()).sum::<u32>()).map(|(&id, _)| id).value()?;

    let result1 = max_minutes.iter().find(|&&(id, ..)| id == id_part_1).map(|&(id, minute, _)| id * minute).value()?;
    let result2 = max_minutes.iter().max_by_key(|&&(.., count)| count).map(|&(id, minute, _)| id * minute).value()?;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
