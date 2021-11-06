use eyre::Result;
use itertools::Itertools;

use std::fs;
use std::iter::once;

fn main() -> Result<()> {
    let input = fs::read("inputs/2015-day03.txt")?;

    let locations = once((0, 0))
        .chain(input.iter().filter_map(|x| match x {
            b'^' => Some((0, 1)),
            b'v' => Some((0, -1)),
            b'<' => Some((-1, 0)),
            b'>' => Some((1, 0)),
            _ => None,
        }))
        .collect_vec();

    let result1 = locations
        .iter()
        .scan((0, 0), |state, direction| {
            state.0 += direction.0;
            state.1 += direction.1;
            Some(*state)
        })
        .sorted_unstable()
        .dedup()
        .count();

    let result2 = locations
        .iter()
        .tuples()
        .scan([(0, 0); 2], |state, (direction1, direction2)| {
            state[0].0 += direction1.0;
            state[0].1 += direction1.1;
            state[1].0 += direction2.0;
            state[1].1 += direction2.1;
            Some(*state)
        })
        .flatten()
        .sorted_unstable()
        .dedup()
        .count();

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
