use aoc::*;

use eyre::{bail, ensure};
use itertools::{iproduct, Itertools};

#[derive(Clone)]
struct Grid {
    width: usize,
    height: usize,
    tiles: Vec<Option<bool>>,
}

impl Grid {
    fn new(width: usize, height: usize, tiles: Vec<Option<bool>>) -> Result<Self> {
        ensure!(width * height == tiles.len(), "unable to construct Grid: width * height != tiles.len()");
        Ok(Self { width, height, tiles })
    }

    fn get_index(&self, row: usize, column: usize) -> usize {
        row * self.width + column
    }
}

fn simulate(mut grid: Grid, buffer: &mut Vec<Option<bool>>, min_neighbors: usize, max_diff: usize) -> usize {
    let mut locked = false;

    while !locked {
        locked = true;

        buffer.clear();

        for (i_row, i_col) in iproduct!(0..grid.height, 0..grid.width) {
            let old_tile = grid.tiles[grid.get_index(i_row, i_col)];

            let is_occupied = match old_tile {
                Some(is_occupied) => is_occupied,
                None => {
                    buffer.push(None);
                    continue;
                }
            };

            let top = i_row + 1;
            let left = i_col + 1;
            let right = grid.width - i_col;
            let bottom = grid.height - i_row;

            let mut count = 0;
            count += (1..top.min(max_diff)).find_map(|diff| grid.tiles[grid.get_index(i_row - diff, i_col)]).unwrap_or_default() as usize;
            count += (1..left.min(max_diff)).find_map(|diff| grid.tiles[grid.get_index(i_row, i_col - diff)]).unwrap_or_default() as usize;
            count += (1..right.min(max_diff)).find_map(|diff| grid.tiles[grid.get_index(i_row, i_col + diff)]).unwrap_or_default() as usize;
            count += (1..bottom.min(max_diff)).find_map(|diff| grid.tiles[grid.get_index(i_row + diff, i_col)]).unwrap_or_default() as usize;
            count += (1..top.min(left).min(max_diff)).find_map(|diff| grid.tiles[grid.get_index(i_row - diff, i_col - diff)]).unwrap_or_default() as usize;
            count += (1..top.min(right).min(max_diff)).find_map(|diff| grid.tiles[grid.get_index(i_row - diff, i_col + diff)]).unwrap_or_default() as usize;
            count += (1..bottom.min(left).min(max_diff)).find_map(|diff| grid.tiles[grid.get_index(i_row + diff, i_col - diff)]).unwrap_or_default() as usize;
            count += (1..bottom.min(right).min(max_diff)).find_map(|diff| grid.tiles[grid.get_index(i_row + diff, i_col + diff)]).unwrap_or_default() as usize;

            if is_occupied && count >= min_neighbors || !is_occupied && count == 0 {
                locked = false;
                buffer.push(Some(!is_occupied));
            } else {
                buffer.push(Some(is_occupied));
            };
        }

        std::mem::swap(buffer, &mut grid.tiles);
    }

    grid.tiles.iter().map(|x| x.unwrap_or_default() as usize).sum()
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let tiles = input
        .lines()
        .flat_map(|line| {
            line.bytes().map(|x| match x {
                b'.' => Ok(None),
                b'L' => Ok(Some(false)),
                b'#' => Ok(Some(true)),
                _ => bail!("unknown tile"),
            })
        })
        .try_collect()?;

    let width = input.lines().next().value()?.len();
    let height = input.lines().count();

    let grid = Grid::new(width, height, tiles)?;

    let mut buffer = Vec::with_capacity(grid.tiles.len());

    let result1 = simulate(grid.clone(), &mut buffer, 4, 2);
    let result2 = simulate(grid, &mut buffer, 5, usize::MAX);

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
