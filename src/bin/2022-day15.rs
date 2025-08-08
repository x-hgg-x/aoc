use aoc::*;

use itertools::Itertools;
use regex::Regex;

type Point = (i64, i64);

fn find_x_range(
    buffer: &mut Vec<Point>,
    sensors_with_distance: &[(Point, i64)],
    y: i64,
) -> [Option<Point>; 2] {
    buffer.clear();

    buffer.extend(
        (sensors_with_distance.iter()).flat_map(|((sx, sy), distance)| {
            let diff_x = distance - (y - sy).abs();
            (diff_x >= 0).then_some((sx - diff_x, sx + diff_x))
        }),
    );

    buffer.sort_unstable();

    let mut iter = buffer.iter().copied().coalesce(|r1, r2| {
        if r2.0 <= r1.1 + 1 {
            Ok((r1.0, r1.1.max(r2.1)))
        } else {
            Err((r1, r2))
        }
    });

    [iter.next(), iter.next()]
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let re =
        Regex::new(r#"(?m)^Sensor at x=(.+?), y=(.+?): closest beacon is at x=(.+?), y=(.+?)$"#)?;

    let (sensors_with_distance, mut beacons): (Vec<_>, Vec<_>) = re
        .captures_iter(&input)
        .map(|cap| {
            let sx = cap[1].parse::<i64>()?;
            let sy = cap[2].parse::<i64>()?;
            let bx = cap[3].parse::<i64>()?;
            let by = cap[4].parse::<i64>()?;
            let distance = (bx - sx).abs() + (by - sy).abs();
            Result::Ok((((sx, sy), distance), (bx, by)))
        })
        .try_process(|iter| iter.unzip())?;

    beacons.sort_unstable();
    beacons.dedup();

    let mut buffer = Vec::new();

    let middle_y = 2_000_000;

    let x_count = find_x_range(&mut buffer, &sensors_with_distance, middle_y)
        .into_iter()
        .flatten()
        .map(|(min, max)| max - min + 1)
        .sum::<i64>();

    let y_beacons = beacons.iter().filter(|&&(_, by)| by == middle_y).count() as i64;
    let result1 = x_count - y_beacons;

    let max_y = 4_000_000;

    let result2 = (0..=max_y)
        .find_map(
            |y| match find_x_range(&mut buffer, &sensors_with_distance, y) {
                [Some((_, left_max)), Some(_)] => Some((left_max + 1) * max_y + y),
                _ => None,
            },
        )
        .value()?;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
