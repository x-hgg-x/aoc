use itertools::Itertools;
use regex::Regex;

use std::cmp::Reverse;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("inputs/2016-day04.txt")?;

    let re = Regex::new(r#"(?m)^([\w-]+?)-(\d+)\[(\w+)\]$"#).unwrap();

    let real_rooms = re
        .captures_iter(&input)
        .filter_map(|cap| {
            let common_letters: String = cap[1]
                .split('-')
                .flat_map(|x| x.chars())
                .sorted_unstable()
                .dedup_with_count()
                .sorted_unstable_by_key(|&(count, c)| (Reverse(count), c))
                .map(|(_, c)| c)
                .take(5)
                .collect();

            (common_letters == cap[3]).then(|| {
                let id = cap[2].parse::<u32>().unwrap();

                let name: String = cap[1]
                    .chars()
                    .map(|c| {
                        c.to_digit(36)
                            .and_then(|n| std::char::from_digit((n - 10 + id) % 26 + 10, 36))
                            .unwrap_or('-')
                    })
                    .collect();

                (name, id)
            })
        })
        .collect_vec();

    let result1 = real_rooms.iter().fold(0, |acc, (_, id)| acc + id);

    let result2 = real_rooms
        .iter()
        .find(|(name, _)| name == "northpole-object-storage")
        .map(|&(_, id)| id)
        .unwrap();

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
