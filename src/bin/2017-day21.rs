use eyre::{bail, Result};
use itertools::Itertools;
use smallvec::{Array, SmallVec};

use std::fs;
use std::ops::Deref;

type Mat2x2 = SmallVec<[bool; 4]>;
type Mat3x3 = SmallVec<[bool; 9]>;
type Mat4x4 = SmallVec<[bool; 16]>;

trait SmallVecBool: Deref<Target = [bool]> {
    fn new() -> Self;
    fn extend_from_slice(&mut self, slice: &[bool]);
}

impl<T: Array<Item = bool>> SmallVecBool for SmallVec<T> {
    fn new() -> Self {
        Self::new()
    }
    fn extend_from_slice(&mut self, slice: &[bool]) {
        self.extend_from_slice(slice)
    }
}

struct Grid {
    size: usize,
    tiles: Vec<bool>,
}

fn parse<T: FromIterator<bool>>(pattern: &str) -> T {
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
    array.iter().copied().enumerate().filter(|&(_, x)| x).map(|(index, _)| 1 << index).sum()
}

fn transpose(array: &mut [bool], size: usize) -> &mut [bool] {
    for i in 0..size {
        for j in i + 1..size {
            array.swap(i + size * j, j + size * i);
        }
    }
    array
}

fn flip(array: &mut [bool], size: usize) -> &mut [bool] {
    for i in 0..size {
        for j in 0..(size + 1) / 2 {
            array.swap(i + size * j, i + size * (size - 1 - j));
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

fn apply_rules<Src: SmallVecBool, Dst: SmallVecBool>(grid: &mut Grid, buf: &mut Vec<bool>, rules: &[Option<Dst>], block_size: usize, new_block_size: usize) {
    let block_size_sq = block_size.pow(2);
    let new_block_size_sq = new_block_size.pow(2);

    let block_count = grid.size / block_size;
    let new_grid_size = new_block_size * block_count;

    buf.clear();
    buf.resize(new_grid_size.pow(2), false);

    for i_block in 0..block_count {
        for j_block in 0..block_count {
            let mut array = Src::new();

            let start = i_block * block_size_sq * block_count + j_block * block_size;
            for i in 0..block_size {
                let start_i = start + i * grid.size;
                array.extend_from_slice(&grid.tiles[start_i..start_i + block_size]);
            }

            let new_array = rules[reduce(&array)].as_deref().unwrap();

            let new_start = i_block * new_block_size_sq * block_count + j_block * new_block_size;
            for i in 0..new_block_size {
                let new_start_i = new_start + i * new_grid_size;
                let src_range_i = i * new_block_size..(i + 1) * new_block_size;
                buf[new_start_i..new_start_i + new_block_size].copy_from_slice(&new_array[src_range_i]);
            }
        }
    }

    grid.size = new_grid_size;
    std::mem::swap(buf, &mut grid.tiles);
}

fn run(grid: &mut Grid, buf: &mut Vec<bool>, rules_2x2: &[Option<Mat3x3>], rules_3x3: &[Option<Mat4x4>]) {
    if grid.size % 2 == 0 {
        apply_rules::<Mat2x2, _>(grid, buf, rules_2x2, 2, 3);
    } else {
        apply_rules::<Mat3x3, _>(grid, buf, rules_3x3, 3, 4);
    }
}

fn main() -> Result<()> {
    let input = fs::read_to_string("inputs/2017-day21.txt")?;

    let mut rules_2x2 = vec![None; 1 << 4];
    let mut rules_3x3 = vec![None; 1 << 9];

    for line in input.lines() {
        let (before, after) = line.split(" => ").next_tuple().unwrap();

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
        run(&mut grid, &mut buf, &rules_2x2, &rules_3x3);
    }
    let result1 = grid.tiles.iter().filter(|&&x| x).count();

    for _ in 0..13 {
        run(&mut grid, &mut buf, &rules_2x2, &rules_3x3);
    }
    let result2 = grid.tiles.iter().filter(|&&x| x).count();

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
