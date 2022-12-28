use aoc::*;

use eyre::ensure;
use itertools::{izip, Itertools};

use std::iter::{once, repeat};

struct Grid {
    width: usize,
    tiles: Vec<u8>,
}

impl Grid {
    fn new(width: usize, height: usize, tiles: Vec<u8>) -> Result<Self> {
        ensure!(width * height == tiles.len(), "unable to construct Grid: width * height != tiles.len()");
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

    let base_width = input.lines().next().value()?.len();
    let base_height = input.lines().count();

    let additional_line = repeat(9).take(base_width + 2);

    let tiles = additional_line
        .clone()
        .chain(input.lines().flat_map(|line| once(9).chain(line.bytes().map(|x| x - b'0')).chain(once(9))))
        .chain(additional_line)
        .collect_vec();

    let grid = Grid::new(base_width + 2, base_height + 2, tiles)?;

    let (count, sum) = grid
        .tiles
        .chunks_exact(grid.width)
        .tuple_windows()
        .flat_map(|(row_0, row_1, row_2)| {
            izip!(row_0.windows(3), row_1.windows(3), row_2.windows(3)).filter_map(|(x0, x1, x2)| {
                let center = x1[1];
                (center < x0[1] && center < x1[0] && center < x1[2] && center < x2[1]).then_some(center)
            })
        })
        .fold((0, 0), |(count, sum), x| (count + 1u64, sum + x as u64));

    let result1 = count + sum;

    let mut index = 0;
    let mut counts = Vec::new();
    let mut queue = Vec::new();

    let mut processed = grid.tiles.iter().map(|&x| x != 9).collect_vec();

    while let Some(position) = processed.iter().skip(index).position(|&x| x) {
        index += position;
        processed[index] = false;

        queue.clear();
        queue.push(index);

        let mut count = 0;

        while let Some(index) = queue.pop() {
            let (row, column) = grid.get_position(index);

            queue.extend([(row - 1, column), (row + 1, column), (row, column - 1), (row, column + 1)].into_iter().filter_map(|(new_row, new_column)| {
                let new_index = grid.get_index(new_row, new_column);
                if grid.tiles[new_index] != 9 {
                    if let Some(x) = processed.get_mut(new_index) {
                        if *x {
                            *x = false;
                            return Some(new_index);
                        }
                    }
                }
                None
            }));

            count += 1;
        }

        counts.push(count);
    }

    counts.sort_unstable();

    let result2 = counts.iter().rev().next_tuple().map(|(x0, x1, x2)| x0 * x1 * x2).value()?;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
