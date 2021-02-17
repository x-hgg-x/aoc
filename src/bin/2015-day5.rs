use itertools::Itertools;

use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("inputs/2015-day5.txt")?;

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
                .map(|(i, x)| (i as isize, x))
                .sorted_unstable_by_key(|x| x.1)
                .dedup_by_with_count(|&x, &y| x.1 == y.1 && (x.0 - y.0).abs() > 1)
                .any(|x| x.0 > 1)
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
