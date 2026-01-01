use aoc::*;

use eyre::{bail, eyre};
use itertools::Itertools;
use num_complex::Complex;

use std::ops::RangeInclusive;

const START: Complex<i64> = Complex::new(0, 0);
const UP: Complex<i64> = Complex::new(0, 1);
const DOWN: Complex<i64> = Complex::new(0, -1);
const LEFT: Complex<i64> = Complex::new(-1, 0);
const RIGHT: Complex<i64> = Complex::new(1, 0);

#[derive(Copy, Clone)]
enum Tile {
    UpDown,
    LeftRight,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

type Tiles = Vec<(RangeInclusive<i64>, RangeInclusive<i64>, Tile)>;

fn compute_tiles(
    instructions: &[[(Complex<i64>, i64); 2]],
    instruction_index: usize,
) -> Result<Tiles> {
    let mut tiles = Vec::new();

    let mut current_position = START;

    for x in instructions.windows(2) {
        let (direction, length) = x[0][instruction_index];
        let next_direction = x[1][instruction_index].0;

        if length > 1 {
            match direction {
                LEFT | RIGHT => {
                    let x_start = current_position.re + direction.re;
                    let x_end = current_position.re + direction.re * (length - 1);
                    let y_range = current_position.im..=current_position.im;
                    let tile = Tile::LeftRight;
                    tiles.push((x_start.min(x_end)..=x_start.max(x_end), y_range, tile));
                }
                UP | DOWN => {
                    let y_start = current_position.im + direction.im;
                    let y_end = current_position.im + direction.im * (length - 1);
                    let x_range = current_position.re..=current_position.re;
                    let tile = Tile::UpDown;
                    tiles.push((x_range, y_start.min(y_end)..=y_start.max(y_end), tile));
                }
                _ => bail!("invalid direction"),
            }
        }

        current_position += direction * length;

        let corner_tile = match (direction, next_direction) {
            (RIGHT, UP) | (DOWN, LEFT) => Tile::UpLeft,
            (LEFT, UP) | (DOWN, RIGHT) => Tile::UpRight,
            (RIGHT, DOWN) | (UP, LEFT) => Tile::DownLeft,
            (LEFT, DOWN) | (UP, RIGHT) => Tile::DownRight,
            _ => bail!("invalid direction"),
        };

        let x_range = current_position.re..=current_position.re;
        let y_range = current_position.im..=current_position.im;
        tiles.push((x_range, y_range, corner_tile));
    }

    let y_edges = tiles
        .iter()
        .flat_map(|(_, y_range, _)| [*y_range.start(), *y_range.end()])
        .sorted_unstable()
        .dedup()
        .collect_vec();

    Ok(tiles
        .into_iter()
        .flat_map(|(x_range, y_range, tile)| {
            let (y_start, y_end) = (*y_range.start(), *y_range.end());

            let index = y_edges.partition_point(|&y| y < y_start);

            y_edges[index..]
                .iter()
                .take_while(move |y| y_range.contains(y))
                .tuple_windows()
                .flat_map(|(&y1, &y2)| {
                    [Some(y1..=y1), (y2 - y1 > 1).then_some(y1 + 1..=y2 - 1)]
                        .into_iter()
                        .flatten()
                })
                .chain([y_end..=y_end])
                .map(move |new_y_range| (x_range.clone(), new_y_range, tile))
        })
        .sorted_unstable_by_key(|(x_range, y_range, _)| (*y_range.start(), *x_range.start()))
        .collect())
}

fn compute_lagoon_size(tiles: &Tiles) -> Result<i64> {
    let mut iter = tiles.iter().peekable();

    let mut count = tiles
        .iter()
        .map(|(x_range, y_range, _)| (x_range.clone().count() * y_range.clone().count()) as i64)
        .sum::<i64>();

    while let Some((_, next_y_range, _)) = iter.peek() {
        count += iter
            .take_while_ref(|(_, y_range, _)| y_range == next_y_range)
            .tuple_windows()
            .scan(
                (0u64, None),
                |state, ((x1_range, y_range, tile1), (x2_range, _, tile2))| {
                    if state.0 == 0 {
                        match tile1 {
                            Tile::UpDown => state.0 += 1,
                            Tile::UpRight | Tile::DownRight => *state = (state.0 + 1, Some(tile1)),
                            _ => (),
                        }
                    }

                    let inside_count = if state.0.is_multiple_of(2) {
                        0
                    } else {
                        (x2_range.start() - x1_range.end() - 1) * y_range.clone().count() as i64
                    };

                    match (tile1, tile2) {
                        (Tile::UpDown | Tile::UpLeft | Tile::DownLeft, Tile::UpDown)
                        | (Tile::UpRight, Tile::UpLeft)
                        | (Tile::DownRight, Tile::DownLeft) => state.0 += 1,
                        (
                            Tile::UpDown | Tile::UpLeft | Tile::DownLeft,
                            Tile::UpRight | Tile::DownRight,
                        ) => {
                            *state = (state.0 + 1, Some(tile2));
                        }
                        (Tile::LeftRight, Tile::UpLeft | Tile::DownLeft) => {
                            let Some(left_tile) = state.1.take() else {
                                return Some(Err(eyre!("invalid input")));
                            };
                            if matches!(
                                (left_tile, tile2),
                                (Tile::UpRight, Tile::UpLeft) | (Tile::DownRight, Tile::DownLeft)
                            ) {
                                state.0 += 1;
                            }
                        }
                        _ => (),
                    };

                    Some(Ok(inside_count))
                },
            )
            .try_sum::<i64>()?;
    }

    Ok(count)
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let mut instructions: Vec<_> = input
        .lines()
        .map(|line| {
            let (direction, length, color) = line.split_ascii_whitespace().next_tuple().value()?;

            let direction = match direction {
                "U" => UP,
                "D" => DOWN,
                "L" => LEFT,
                "R" => RIGHT,
                _ => bail!("unknown direction: {direction}"),
            };

            let length = length.parse::<i64>()?;

            let color_length = i64::from_str_radix(&color[2..7], 16)?;

            let color_direction = match color.as_bytes()[7] {
                b'0' => RIGHT,
                b'1' => DOWN,
                b'2' => LEFT,
                b'3' => UP,
                _ => bail!("unknown direction: {direction}"),
            };

            Ok([(direction, length), (color_direction, color_length)])
        })
        .try_collect()?;

    instructions.push(*instructions.first().value()?);

    let result1 = compute_lagoon_size(&compute_tiles(&instructions, 0)?)?;
    let result2 = compute_lagoon_size(&compute_tiles(&instructions, 1)?)?;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
