use aoc::*;

use eyre::bail;
use itertools::Itertools;
use smallvec::{Array, SmallVec};

use std::ops::Deref;

type Mat2x2 = SmallVec<[bool; 4]>;
type Mat3x3 = SmallVec<[bool; 9]>;
type Mat4x4 = SmallVec<[bool; 16]>;

trait SmallVecBool: Deref<Target = [bool]> + FromIterator<bool> {}

impl<T: Array<Item = bool>> SmallVecBool for SmallVec<T> {}

struct Grid {
    size: usize,
    tiles: Vec<bool>,
}

fn parse<T: SmallVecBool>(pattern: &str) -> T {
    pattern
        .bytes()
        .filter_map(|x| match x {
            b'.' => Some(false),
            b'#' => Some(true),
            _ => None,
        })
        .collect()
}

fn reduce(array: &[bool]) -> usize {
    array
        .iter()
        .copied()
        .enumerate()
        .filter(|&(_, x)| x)
        .map(|(index, _)| 1 << index)
        .sum()
}

fn transpose(array: &mut [bool], size: usize) -> &mut [bool] {
    for i in 0..size {
        let offset_i = size * i;
        for j in i + 1..size {
            let offset_j = size * j;
            array.swap(i + offset_j, j + offset_i);
        }
    }
    array
}

fn flip(array: &mut [bool], size: usize) -> &mut [bool] {
    for row in array.chunks_exact_mut(size) {
        let mut iter = row.iter_mut();
        while let (Some(first), Some(last)) = (iter.next(), iter.next_back()) {
            std::mem::swap(first, last);
        }
    }
    array
}

fn transformations(array: &mut [bool], size: usize) -> [usize; 8] {
    [
        reduce(array),
        reduce(transpose(array, size)),
        reduce(flip(array, size)),
        reduce(transpose(array, size)),
        reduce(flip(array, size)),
        reduce(transpose(array, size)),
        reduce(flip(array, size)),
        reduce(transpose(array, size)),
    ]
}

fn apply_rules<Src: SmallVecBool, Dst: SmallVecBool>(
    grid: &mut Grid,
    buf: &mut Vec<bool>,
    rules: &[Option<Dst>],
    block_size: usize,
    new_block_size: usize,
) -> Result<()> {
    let block_count = grid.size / block_size;
    let new_grid_size = new_block_size * block_count;

    buf.clear();
    buf.resize(new_grid_size.pow(2), false);

    for i_block in 0..block_count {
        for j_block in 0..block_count {
            let array: Src = grid
                .tiles
                .chunks_exact(grid.size)
                .skip(i_block * block_size)
                .take(block_size)
                .flat_map(|line| line.iter().skip(j_block * block_size).take(block_size))
                .copied()
                .collect();

            let new_array = rules[reduce(&array)].as_deref().value()?;

            buf.chunks_exact_mut(new_grid_size)
                .skip(i_block * new_block_size)
                .take(new_block_size)
                .zip(new_array.chunks_exact(new_block_size))
                .for_each(|(buf_line, new_array_line)| {
                    buf_line[j_block * new_block_size..j_block * new_block_size + new_block_size]
                        .copy_from_slice(new_array_line)
                });
        }
    }

    grid.size = new_grid_size;
    std::mem::swap(buf, &mut grid.tiles);

    Ok(())
}

fn run(
    grid: &mut Grid,
    buf: &mut Vec<bool>,
    rules_2x2: &[Option<Mat3x3>],
    rules_3x3: &[Option<Mat4x4>],
) -> Result<()> {
    if grid.size % 2 == 0 {
        apply_rules::<Mat2x2, _>(grid, buf, rules_2x2, 2, 3)?;
    } else {
        apply_rules::<Mat3x3, _>(grid, buf, rules_3x3, 3, 4)?;
    }

    Ok(())
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let mut rules_2x2 = vec![None; 1 << 4];
    let mut rules_3x3 = vec![None; 1 << 9];

    for line in input.lines() {
        let (before, after) = line.split(" => ").next_tuple().value()?;

        if before.len() == 5 {
            let v = parse::<Mat3x3>(after);

            let mut before_pattern = parse::<Mat2x2>(before);
            for k in transformations(&mut before_pattern, 2) {
                rules_2x2[k] = Some(v.clone());
            }
        } else if before.len() == 11 {
            let v = parse::<Mat4x4>(after);

            let mut before_pattern = parse::<Mat3x3>(before);
            for k in transformations(&mut before_pattern, 3) {
                rules_3x3[k] = Some(v.clone());
            }
        } else {
            bail!("unable to parse input");
        }
    }

    let tiles = vec![false, true, false, false, false, true, true, true, true];
    let mut grid = Grid { size: 3, tiles };

    let mut buf = Vec::new();

    for _ in 0..5 {
        run(&mut grid, &mut buf, &rules_2x2, &rules_3x3)?;
    }
    let result1 = grid.tiles.iter().filter(|&&x| x).count();

    for _ in 0..13 {
        run(&mut grid, &mut buf, &rules_2x2, &rules_3x3)?;
    }
    let result2 = grid.tiles.iter().filter(|&&x| x).count();

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
