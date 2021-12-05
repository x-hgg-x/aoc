use eyre::Result;
use itertools::Itertools;

use std::fs;
use std::iter::once;

fn main() -> Result<()> {
    let input = fs::read("inputs/2015-day03.txt")?;

    let locations = once((0i64, 0i64))
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
        .scan((0, 0), |(x, y), &(direction_x, direction_y)| {
            *x += direction_x;
            *y += direction_y;
            Some((*x, *y))
        })
        .sorted_unstable()
        .dedup()
        .count();

    let result2 = locations
        .chunks(2)
        .scan([(0, 0); 2], |[(x1, y1), (x2, y2)], directions| {
            match directions {
                [(direction_1_x, direction_1_y), (direction_2_x, direction_2_y)] => {
                    *x1 += direction_1_x;
                    *y1 += direction_1_y;
                    *x2 += direction_2_x;
                    *y2 += direction_2_y;
                }
                [(direction_x, direction_y)] => {
                    *x1 += direction_x;
                    *y1 += direction_y;
                }
                _ => unreachable!(),
            };

            Some([(*x1, *y1), (*x2, *y2)])
        })
        .flatten()
        .sorted_unstable()
        .dedup()
        .count();

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
