use aoc::*;

use eyre::ensure;
use itertools::{iproduct, Itertools};

const SIZE: usize = 10;

struct Grid {
    tiles: Vec<u8>,
}

impl Grid {
    fn new(tiles: Vec<u8>) -> Result<Self> {
        ensure!(SIZE * SIZE == tiles.len(), "unable to construct Grid: width * height != tiles.len()");
        Ok(Self { tiles })
    }

    fn get_index(row: usize, column: usize) -> usize {
        row * SIZE + column
    }

    fn get_position(index: usize) -> (usize, usize) {
        let row = index / SIZE;
        let column = index % SIZE;
        (row, column)
    }
}

fn step(grid: &mut Grid, queue: &mut Vec<(usize, usize)>) -> (usize, bool) {
    let mut count = 0;

    queue.extend((0..SIZE * SIZE).map(Grid::get_position));

    while let Some((row, column)) = queue.pop() {
        let tile = &mut grid.tiles[Grid::get_index(row, column)];
        *tile += 1;

        if *tile == 10 {
            let new_rows = [Some(row), row.checked_sub(1), (row < SIZE - 1).then_some(row + 1)];
            let new_columns = [Some(column), column.checked_sub(1), (column < SIZE - 1).then_some(column + 1)];

            queue.extend(iproduct!(new_rows.into_iter().flatten(), new_columns.into_iter().flatten()).skip(1));
            count += 1;
        }
    }

    for tile in &mut grid.tiles {
        if *tile >= 10 {
            *tile = 0;
        }
    }

    let is_sync = grid.tiles.iter().all(|&x| x == 0);

    (count, is_sync)
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let tiles = input.lines().flat_map(|line| line.bytes().map(|x| x - b'0')).collect_vec();
    let mut grid = Grid::new(tiles)?;

    let mut total_count = 0usize;
    let mut sync_step = None;
    let mut queue = Vec::new();

    for i in 1..101 {
        let (count, is_sync) = step(&mut grid, &mut queue);
        total_count += count;

        if is_sync && sync_step.is_none() {
            sync_step = Some(i);
        }
    }

    if sync_step.is_none() {
        for i in 101usize.. {
            let (_, is_sync) = step(&mut grid, &mut queue);

            if is_sync {
                sync_step = Some(i);
                break;
            }
        }
    }

    let result1 = total_count;
    let result2 = sync_step.value()?;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
