use itertools::Itertools;

use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("inputs/2015-day03.txt")?;
    let input = input.trim();

    let iter = vec![(0, 0)]
        .into_iter()
        .chain(input.chars().map(|c| match c {
            '^' => (0, 1),
            'v' => (0, -1),
            '<' => (-1, 0),
            '>' => (1, 0),
            _ => (0, 0),
        }));

    let result1 = iter
        .clone()
        .scan((0, 0), |state, direction| {
            state.0 += direction.0;
            state.1 += direction.1;
            Some(*state)
        })
        .sorted_unstable()
        .dedup()
        .count();

    let result2 = iter
        .tuples()
        .scan(vec![(0, 0); 2], |state, (direction1, direction2)| {
            state[0].0 += direction1.0;
            state[0].1 += direction1.1;
            state[1].0 += direction2.0;
            state[1].1 += direction2.1;
            Some(state.clone())
        })
        .flatten()
        .sorted_unstable()
        .dedup()
        .count();

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
