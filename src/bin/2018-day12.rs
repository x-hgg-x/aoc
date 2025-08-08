use aoc::*;

use eyre::{bail, ensure};
use itertools::Itertools;
use regex::bytes::Regex;

use std::collections::VecDeque;

struct Plants {
    pots: VecDeque<u8>,
    current_start_index: i64,
}

fn read_token(token: u8) -> Result<u8> {
    match token {
        b'.' => Ok(0),
        b'#' => Ok(1),
        _ => bail!("unknown state"),
    }
}

fn compute_rule_index(iter: impl IntoIterator<Item = u8>) -> usize {
    iter.into_iter().enumerate().map(|(index, x)| x as usize * (1 << index)).sum()
}

fn step(plants: &mut Plants, buf: &mut VecDeque<u8>, rules: &[u8; 32]) {
    while plants.pots.front() == Some(&0) {
        plants.pots.pop_front();
        plants.current_start_index += 1;
    }

    while plants.pots.back() == Some(&0) {
        plants.pots.pop_back();
    }

    let iter = [0; 4].into_iter().chain(plants.pots.iter().copied()).chain([0; 4]).tuple_windows().map(|(x0, x1, x2, x3, x4)| {
        let rule_index = compute_rule_index([x0, x1, x2, x3, x4]);
        rules[rule_index]
    });

    buf.clear();
    buf.extend(iter);
    std::mem::swap(buf, &mut plants.pots);
    plants.current_start_index -= 2;
}

fn compute_sum(plants: &mut Plants) -> i64 {
    (plants.current_start_index..).zip(&plants.pots).filter(|&(_, &pot)| pot == 1).map(|(index, _)| index).sum()
}

fn main() -> Result<()> {
    let input = setup(file!())?;

    let regex_start = Regex::new(r#"(?m)^initial state: ([#.]+)$"#)?;
    let regex_rule = Regex::new(r#"(?m)^([#.]{5}) => ([#.]$)"#)?;

    let mut rules = [0u8; 32];
    for cap in regex_rule.captures_iter(&input) {
        let rule_index = cap[1].iter().copied().map(read_token).try_process(|iter| compute_rule_index(iter))?;
        rules[rule_index] = read_token(cap[2][0])?;
    }

    ensure!(rules[0] == 0, "unsupported rule");

    let mut plants = Plants { pots: regex_start.captures(&input).value()?[1].iter().copied().map(read_token).try_collect()?, current_start_index: 0 };

    let mut buf = VecDeque::new();

    let n0 = 20;
    let n1 = 9000;
    let n2 = 10000;
    let n3 = 50_000_000_000;

    for _ in 0..n0 {
        step(&mut plants, &mut buf, &rules);
    }
    let result1 = compute_sum(&mut plants);

    for _ in 0..n1 - n0 {
        step(&mut plants, &mut buf, &rules);
    }
    let count1 = compute_sum(&mut plants);

    for _ in 0..n2 - n1 {
        step(&mut plants, &mut buf, &rules);
    }
    let count2 = compute_sum(&mut plants);

    let result2 = count2 + (count2 - count1) / (n2 - n1) * (n3 - n2);

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
