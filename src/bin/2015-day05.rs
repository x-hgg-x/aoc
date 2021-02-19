use itertools::Itertools;

use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("inputs/2015-day05.txt")?;

    let result1 = input
        .lines()
        .map(|line| {
            let iter1 = line
                .as_bytes()
                .iter()
                .filter(|&&x| x == b'a' || x == b'e' || x == b'i' || x == b'o' || x == b'u');

            let iter2 = line.as_bytes().windows(2).map(|x| {
                if x == [b'a', b'b'] || x == [b'c', b'd'] || x == [b'p', b'q'] || x == [b'x', b'y']
                {
                    None
                } else if x[0] == x[1] {
                    Some(true)
                } else {
                    Some(false)
                }
            });

            iter1.count() >= 3
                && !iter2.clone().any(|x| x.is_none())
                && iter2.filter_map(|x| x).any(|x| x)
        })
        .filter(|&x| x)
        .count();

    let result2 = input
        .lines()
        .map(|line| {
            line.as_bytes()
                .windows(2)
                .enumerate()
                .sorted_unstable_by_key(|(_, x)| *x)
                .dedup_by_with_count(|&(pos1, x1), &(pos2, x2)| {
                    x1 == x2 && (pos1 as isize - pos2 as isize).abs() > 1
                })
                .any(|(count, _)| count > 1)
                .then(|| line.as_bytes().windows(3).any(|x| x[0] == x[2]))
                .filter(|&x| x)
                .is_some()
        })
        .filter(|&x| x)
        .count();

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
