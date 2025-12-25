use aoc::*;

use eyre::ensure;
use itertools::Itertools;

use std::collections::HashMap;
use std::iter::{once, repeat_n};

struct Grid {
    width: usize,
    tiles: Vec<u8>,
}

impl Grid {
    fn new(width: usize, height: usize, tiles: Vec<u8>) -> Result<Self> {
        ensure!(
            width * height == tiles.len(),
            "unable to construct Grid: width * height != tiles.len()"
        );

        Ok(Self { width, tiles })
    }

    fn get_index(&self, row: usize, column: usize) -> usize {
        row * self.width + column
    }

    fn get_position(&self, index: usize) -> (usize, usize) {
        let row = index / self.width;
        let column = index % self.width;
        (row, column)
    }
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let width = input.lines().next().value()?.len() + 2;
    let height = input.lines().count() + 2;

    let tiles = repeat_n(b'.', width)
        .chain((input.lines()).flat_map(|line| once(b'.').chain(line.bytes()).chain(once(b'.'))))
        .chain(repeat_n(b'.', width))
        .collect_vec();

    let grid = Grid::new(width, height, tiles)?;

    let numbers = grid
        .tiles
        .windows(2)
        .enumerate()
        .filter(|(_, x)| !x[0].is_ascii_digit() && x[1].is_ascii_digit())
        .filter_map(|(idx, _)| {
            let size = (grid.tiles[idx + 1..].iter()).position(|x| !x.is_ascii_digit())?;
            Some((idx + 1, size))
        })
        .map(|(idx, size)| {
            let value = grid.tiles[idx..idx + size]
                .iter()
                .enumerate()
                .map(|(pos, digit)| 10u64.pow((size - 1 - pos) as u32) * (digit - b'0') as u64)
                .sum::<u64>();

            (grid.get_position(idx), size, value)
        })
        .collect_vec();

    let mut parts_sum = 0u64;
    let mut asterisk_parts = HashMap::<_, Vec<_>>::new();

    for &((row, column), size, value) in &numbers {
        let top_left_idx = grid.get_index(row - 1, column - 1);
        let top_right_idx = top_left_idx + size + 1;
        let left_idx = top_left_idx + width;
        let right_idx = top_right_idx + width;
        let bottom_left_idx = left_idx + width;
        let bottom_right_idx = right_idx + width;

        let mut is_part = false;

        (top_left_idx..=top_right_idx)
            .chain([left_idx, right_idx])
            .chain(bottom_left_idx..=bottom_right_idx)
            .filter(|&idx| !matches!(grid.tiles[idx], b'0'..=b'9' | b'.'))
            .for_each(|idx| {
                is_part = true;
                if grid.tiles[idx] == b'*' {
                    asterisk_parts.entry(idx).or_default().push(value);
                }
            });

        if is_part {
            parts_sum += value;
        }
    }

    let result1 = parts_sum;

    let result2 = asterisk_parts
        .values()
        .filter_map(|v| <&[_; _]>::try_from(&v[..]).ok())
        .map(|[n1, n2]| n1 * n2)
        .sum::<u64>();

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
