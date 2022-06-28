use aoc::*;

use itertools::Itertools;

fn solve(input: &[u8], size: usize, turns: usize) -> Result<Vec<usize>> {
    let cups = input.iter().map(|x| (x - b'1') as usize).collect_vec();
    let mut current_cup = cups[0];

    let mut successors = (1..size).chain(cups.first().copied()).collect_vec();
    for i in 0..cups.len() - 1 {
        successors[cups[i]] = cups[(i + 1) % cups.len()];
    }
    successors[*cups.last().value()?] = if size > cups.len() { cups.len() } else { cups[0] };

    for _ in 0..turns {
        let s1 = successors[current_cup];
        let s2 = successors[s1];
        let s3 = successors[s2];

        let destination_cup = (0..size).rev().cycle().skip(size - current_cup).find(|x| ![s1, s2, s3].contains(x)).value()?;
        (successors[current_cup], successors[destination_cup], successors[s3]) = (successors[s3], s1, successors[destination_cup]);
        current_cup = successors[current_cup];
    }

    Ok(successors)
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);
    let input = input.trim();

    let successors = solve(input.as_bytes(), 9, 100)?;

    let mut result1 = String::new();
    let mut cup = successors[0];
    for _ in 0..8 {
        result1.push((b'1' + cup as u8) as char);
        cup = successors[cup];
    }

    let successors = solve(input.as_bytes(), 1_000_000, 10_000_000)?;

    let cup1 = successors[0];
    let cup2 = successors[cup1];
    let result2 = (cup1 + 1) * (cup2 + 1);

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
