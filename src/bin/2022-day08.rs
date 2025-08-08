use aoc::*;

use eyre::ensure;
use itertools::{Itertools, iproduct};

use std::iter;

struct Grid {
    width: usize,
    height: usize,
    tiles: Vec<i8>,
}

impl Grid {
    fn new(width: usize, height: usize, tiles: Vec<i8>) -> Result<Self> {
        ensure!(width * height == tiles.len(), "unable to construct Grid: width * height != tiles.len()");
        Ok(Self { width, height, tiles })
    }

    fn get_index(&self, row: usize, column: usize) -> usize {
        row * self.width + column
    }
}

fn compute_visible<'a>(zip_iter: impl Iterator<Item = (&'a mut bool, &'a i8)>) {
    let mut max = -1;
    for (visible, &tile) in zip_iter {
        if tile > max {
            *visible = true;
            max = tile;
        }
    }
}

fn compute_all_visible(grid: &Grid) -> usize {
    let width = grid.width;
    let height = grid.height;
    let tiles = &grid.tiles;

    let mut visible_trees = vec![false; tiles.len()];
    for index in [0, width - 1, width * (height - 1), width * height - 1] {
        visible_trees[index] = true;
    }

    for (visible_row, tile_row) in visible_trees.chunks_exact_mut(width).zip(tiles.chunks_exact(width)).skip(1).take(height - 2) {
        compute_visible(iter::zip(&mut *visible_row, tile_row));
        compute_visible(iter::zip(&mut *visible_row, tile_row).rev());
    }

    for i_col in 1..width - 1 {
        compute_visible(iter::zip(visible_trees.iter_mut().skip(i_col).step_by(width), tiles.iter().skip(i_col).step_by(width)));
        compute_visible(iter::zip(visible_trees.iter_mut().skip(i_col).step_by(width), tiles.iter().skip(i_col).step_by(width)).rev());
    }

    visible_trees.iter().filter(|&&x| x).count()
}

fn compute_scenic_score(grid: &Grid) -> Result<usize> {
    iproduct!(0..grid.height, 0..grid.width)
        .map(|(i_row, i_col)| {
            let tree_size = grid.tiles[grid.get_index(i_row, i_col)];

            let top = i_row;
            let left = i_col;
            let right = grid.width - 1 - i_col;
            let bottom = grid.height - 1 - i_row;

            let top_score = (1..top).position(|diff| grid.tiles[grid.get_index(i_row - diff, i_col)] >= tree_size).map(|x| x + 1).unwrap_or(top);
            let left_score = (1..left).position(|diff| grid.tiles[grid.get_index(i_row, i_col - diff)] >= tree_size).map(|x| x + 1).unwrap_or(left);
            let right_score = (1..right).position(|diff| grid.tiles[grid.get_index(i_row, i_col + diff)] >= tree_size).map(|x| x + 1).unwrap_or(right);
            let bottom_score = (1..bottom).position(|diff| grid.tiles[grid.get_index(i_row + diff, i_col)] >= tree_size).map(|x| x + 1).unwrap_or(bottom);

            top_score * left_score * right_score * bottom_score
        })
        .max()
        .value()
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let tiles = input.lines().flat_map(|line| line.bytes().map(|x| (x - b'0') as i8)).collect_vec();

    let width = input.lines().next().value()?.len();
    let height = input.lines().count();

    let grid = Grid::new(width, height, tiles)?;

    let result1 = compute_all_visible(&grid);
    let result2 = compute_scenic_score(&grid)?;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
