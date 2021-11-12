use eyre::Result;
use itertools::Itertools;

use std::fs;

struct Layer {
    depth: usize,
    range: usize,
    period: usize,
}

fn main() -> Result<()> {
    let input = fs::read_to_string("inputs/2017-day13.txt")?;

    let layers = input
        .lines()
        .map(|line| {
            let (depth, range) = line.split(": ").map(|x| x.parse().unwrap()).next_tuple().unwrap();
            let period = if range == 0 { 0 } else { (range - 1) * 2 };
            Layer { depth, range, period }
        })
        .collect_vec();

    let result1 = layers.iter().filter(|x| x.depth % x.period == 0).map(|x| x.depth * x.range).sum::<usize>();

    let mut delay = 0;
    while layers.iter().any(|x| (delay + x.depth) % x.period == 0) {
        delay += 1;
    }

    let result2 = delay;

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
