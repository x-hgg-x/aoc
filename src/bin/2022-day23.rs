use aoc::*;

use num_complex::Complex;

use std::collections::{HashMap, HashSet};
use std::iter;

const NORTH: Complex<i64> = Complex::new(0, 1);
const SOUTH: Complex<i64> = Complex::new(0, -1);
const WEST: Complex<i64> = Complex::new(-1, 0);
const EAST: Complex<i64> = Complex::new(1, 0);
const NORTH_WEST: Complex<i64> = Complex::new(-1, 1);
const NORTH_EAST: Complex<i64> = Complex::new(1, 1);
const SOUTH_WEST: Complex<i64> = Complex::new(-1, -1);
const SOUTH_EAST: Complex<i64> = Complex::new(1, -1);

const ALL_DIRECTIONS: &[Complex<i64>; 8] = &[
    NORTH, SOUTH, WEST, EAST, NORTH_WEST, NORTH_EAST, SOUTH_WEST, SOUTH_EAST,
];

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let mut elves: HashSet<_> = input
        .lines()
        .enumerate()
        .flat_map(|(i_row, line)| {
            line.bytes()
                .enumerate()
                .filter(|&(_, x)| x == b'#')
                .map(move |(i_col, _)| Complex::new(i_col as i64, -(i_row as i64)))
        })
        .collect();

    let mut checks = [NORTH, SOUTH, WEST, EAST];
    let mut neighbours = [false; 8];
    let mut proposed_coords = HashMap::<_, Vec<_>>::new();
    let mut round_10_empty_tiles = 0;
    let mut i = 1;

    let last_round = loop {
        proposed_coords.clear();

        let mut locked = true;

        for &coord in &elves {
            iter::zip(&mut neighbours, ALL_DIRECTIONS).for_each(|(neighbour, direction)| {
                *neighbour = elves.contains(&(coord + direction))
            });

            if !neighbours.iter().any(|&x| x) {
                continue;
            }

            let [n, s, w, e, nw, ne, sw, se] = neighbours;

            let proposed_direction = checks.iter().find(|&&check| match check {
                NORTH => !n && !nw && !ne,
                SOUTH => !s && !sw && !se,
                WEST => !w && !nw && !sw,
                EAST => !e && !ne && !se,
                _ => false,
            });

            if let Some(direction) = proposed_direction {
                (proposed_coords.entry(coord + direction).or_default()).push(coord);
            }
        }

        for (&new_coord, old_coords) in &proposed_coords {
            if old_coords.len() == 1 {
                locked = false;
                elves.remove(&old_coords[0]);
                elves.insert(new_coord);
            }
        }

        checks.rotate_left(1);

        if i == 10 {
            let initial_min = Complex::new(i64::MIN, i64::MIN);
            let initial_max = Complex::new(i64::MAX, i64::MAX);

            let (min, max) =
                (elves.iter()).fold((initial_max, initial_min), |(min, max), coord| {
                    (
                        Complex::new(min.re.min(coord.re), min.im.min(coord.im)),
                        Complex::new(max.re.max(coord.re), max.im.max(coord.im)),
                    )
                });

            let rect = max - min + Complex::new(1, 1);
            round_10_empty_tiles = rect.re * rect.im - elves.len() as i64;
        }

        if locked {
            break i;
        }

        i += 1;
    };

    let result1 = round_10_empty_tiles;
    let result2 = last_round;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
