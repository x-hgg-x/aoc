use eyre::Result;
use regex::Regex;

use std::collections::VecDeque;
use std::fs;

fn step(marbles: &mut VecDeque<u64>, scores: &mut [u64], marble: u64) {
    if marble % 23 == 0 {
        marbles.rotate_right(7);
        scores[marble as usize % scores.len()] += marble + marbles.pop_back().unwrap();
        marbles.rotate_left(1);
    } else {
        marbles.rotate_left(1);
        marbles.push_back(marble);
    }
}

fn main() -> Result<()> {
    let input = fs::read_to_string("inputs/2018-day09.txt")?;

    let re = Regex::new(r#"(\d+) players; last marble is worth (\d+) points"#)?;
    let cap = re.captures(&input).unwrap();
    let player_count = cap[1].parse()?;
    let last_marble = cap[2].parse()?;

    let mut scores = vec![0u64; player_count];

    let mut marbles = VecDeque::with_capacity(last_marble as usize);
    marbles.push_back(0);

    for marble in 1..=last_marble {
        step(&mut marbles, &mut scores, marble);
    }
    let result1 = *scores.iter().max().unwrap();

    for marble in last_marble + 1..=last_marble * 100 {
        step(&mut marbles, &mut scores, marble);
    }
    let result2 = *scores.iter().max().unwrap();

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
