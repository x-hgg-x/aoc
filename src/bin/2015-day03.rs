use aoc::*;

use itertools::Itertools;

use std::iter::once;

fn main() -> Result<()> {
    let input = setup(file!())?;

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
            let (direction_1_x, direction_1_y) = directions[0];
            *x1 += direction_1_x;
            *y1 += direction_1_y;

            if let Some((direction_2_x, direction_2_y)) = directions.get(1) {
                *x2 += direction_2_x;
                *y2 += direction_2_y;
            }

            Some([(*x1, *y1), (*x2, *y2)])
        })
        .flatten()
        .sorted_unstable()
        .dedup()
        .count();

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
