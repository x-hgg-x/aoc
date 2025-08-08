use aoc::*;

use itertools::Itertools;
use regex::Regex;

use std::cmp::Reverse;

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let re = Regex::new(r#"(?m)^([\w-]+?)-(\d+)\[(\w+)\]$"#)?;

    let real_rooms: Vec<_> = re
        .captures_iter(&input)
        .filter_map(|cap| {
            (|| {
                let common_letters = String::from_iter(
                    cap[1]
                        .split('-')
                        .flat_map(|x| x.chars())
                        .sorted_unstable()
                        .dedup_with_count()
                        .sorted_unstable_by_key(|&(count, c)| (Reverse(count), c))
                        .map(|(_, c)| c)
                        .take(5),
                );

                if common_letters == cap[3] {
                    let id = cap[2].parse::<u64>()?;

                    let name = String::from_iter(cap[1].chars().map(|c| {
                        c.to_digit(36)
                            .and_then(|n| char::from_digit((n - 10 + id as u32) % 26 + 10, 36))
                            .unwrap_or('-')
                    }));

                    Result::Ok(Some((name, id)))
                } else {
                    Result::Ok(None)
                }
            })()
            .transpose()
        })
        .try_collect()?;

    let result1 = real_rooms.iter().fold(0, |acc, (_, id)| acc + id);

    let result2 = real_rooms
        .iter()
        .find(|(name, _)| name == "northpole-object-storage")
        .map(|&(_, id)| id)
        .value()?;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
