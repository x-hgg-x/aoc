use aoc::*;

use itertools::Itertools;

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let result1 = input
        .lines()
        .map(|line| {
            let vowels_count = line.matches(|c| "aeiou".contains(c)).count();

            let doubles = line
                .as_bytes()
                .windows(2)
                .map(|x| {
                    if x == b"ab" || x == b"cd" || x == b"pq" || x == b"xy" {
                        None
                    } else if x[0] == x[1] {
                        Some(true)
                    } else {
                        Some(false)
                    }
                })
                .collect_vec();

            vowels_count >= 3 && !doubles.iter().any(|x| x.is_none()) && doubles.iter().any(|&x| x == Some(true))
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
                .dedup_by_with_count(|&(pos1, x1), &(pos2, x2)| x1 == x2 && (pos1 as isize - pos2 as isize).abs() > 1)
                .any(|(count, _)| count > 1)
                .then(|| line.as_bytes().windows(3).any(|x| x[0] == x[2]))
                .filter(|&x| x)
                .is_some()
        })
        .filter(|&x| x)
        .count();

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
