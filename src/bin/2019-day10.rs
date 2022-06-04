use aoc::*;

use itertools::Itertools;
use num_complex::{Complex, Complex64};

use std::f64::consts::*;

fn gcd(mut x: i64, mut y: i64) -> i64 {
    while y != 0 {
        (x, y) = (y, x % y);
    }
    x
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let asteroids = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| line.bytes().enumerate().filter(|&(_, pixel)| pixel == b'#').map(move |(x, _)| Complex::new(x as i64, y as i64)))
        .collect_vec();

    let (index_best, count) = asteroids
        .iter()
        .enumerate()
        .map(|(index, asteroid)| {
            let count = asteroids
                .iter()
                .filter(|&other_asteroid| other_asteroid != asteroid)
                .map(|other_asteroid| {
                    let mut normalized_diff = other_asteroid - asteroid;
                    normalized_diff /= gcd(normalized_diff.re.abs(), normalized_diff.im.abs());
                    normalized_diff
                })
                .sorted_unstable_by_key(|x| (x.re, x.im))
                .dedup()
                .count();

            (index, count)
        })
        .max_by_key(|&(_, count)| count)
        .value()?;

    let result1 = count;

    let best = asteroids[index_best];

    let mut sorted_asteroids = asteroids
        .iter()
        .enumerate()
        .filter(|&(index, _)| index != index_best)
        .map(|(_, asteroid)| {
            let diff = asteroid - best;
            let gcd = gcd(diff.re.abs(), diff.im.abs());
            let normalized_diff = diff / gcd;

            let mut angle = Complex64::new(normalized_diff.im as _, -normalized_diff.re as _).arg();
            angle = (angle + PI) % TAU - PI;

            (asteroid, angle, gcd)
        })
        .sorted_unstable_by(|&(_, angle_1, gcd_1), &(_, angle_2, gcd_2)| (angle_1, gcd_1).partial_cmp(&(angle_2, gcd_2)).unwrap())
        .collect_vec();

    let mut current_angle = f64::NAN;
    let mut count = 0usize;
    let mut asteroid_200 = None;

    while asteroid_200.is_none() {
        sorted_asteroids.retain(|&(asteroid, angle, _)| {
            if angle != current_angle {
                current_angle = angle;
                count += 1;
                if count == 200 {
                    asteroid_200 = Some(asteroid);
                }
                false
            } else {
                true
            }
        });
    }

    let result2 = asteroid_200.map(|x| 100 * x.re + x.im).value()?;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
