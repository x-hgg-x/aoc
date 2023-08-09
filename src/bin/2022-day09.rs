use aoc::*;

use eyre::bail;
use itertools::Itertools;
use num_complex::Complex;

use std::collections::HashSet;

fn simulate(moves: &[(Complex<i64>, usize)], len: usize) -> usize {
    let mut knots = vec![Complex::new(0, 0); len];
    let mut visited = HashSet::from([Complex::new(0, 0)]);

    for &(direction, steps) in moves {
        for _ in 0..steps {
            knots[0] += direction;

            let mut prev = knots[0];

            for next in knots.iter_mut().skip(1) {
                let diff = prev - *next;
                if diff.re.abs() > 1 || diff.im.abs() > 1 {
                    *next += Complex::new(diff.re.signum(), diff.im.signum());
                }
                prev = *next;
            }

            visited.insert(knots[len - 1]);
        }
    }

    visited.len()
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let moves: Vec<_> = input
        .lines()
        .map(|line| {
            let (c, n) = line.split_ascii_whitespace().next_tuple().value()?;
            let steps = n.parse::<usize>()?;
            let direction = match c {
                "L" => Complex::<i64>::new(-1, 0),
                "R" => Complex::<i64>::new(1, 0),
                "U" => Complex::<i64>::new(0, 1),
                "D" => Complex::<i64>::new(0, -1),
                other => bail!("unknown direction: {other}"),
            };
            Ok((direction, steps))
        })
        .try_collect()?;

    let result1 = simulate(&moves, 2);
    let result2 = simulate(&moves, 10);

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
