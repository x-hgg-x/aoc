use aoc::*;

use eyre::ensure;
use itertools::Itertools;

use std::collections::{HashSet, VecDeque};

struct Grid {
    width: i64,
    height: i64,
    tiles: Vec<u8>,
}

impl Grid {
    fn new(width: i64, height: i64, tiles: Vec<u8>) -> Result<Self> {
        ensure!(
            width * height == tiles.len() as i64,
            "unable to construct Grid: width * height != tiles.len()"
        );

        Ok(Self {
            width,
            height,
            tiles,
        })
    }

    fn get_index(&self, row: i64, column: i64) -> i64 {
        row * self.width + column
    }

    fn get_position(&self, index: i64) -> (i64, i64) {
        let row = index / self.width;
        let column = index % self.width;
        (row, column)
    }
}

fn shortest_path(grid: &Grid, goal: (i64, i64), start_tile: u8) -> i64 {
    let mut current_states: VecDeque<_> = grid
        .tiles
        .iter()
        .enumerate()
        .filter(|&(_, &x)| x == start_tile)
        .map(|(index, _)| (grid.get_position(index as i64), 0, 0))
        .collect();

    let mut previous_states: HashSet<_> = current_states
        .iter()
        .map(|&(position, _, _)| position)
        .collect();

    loop {
        if let Some(((row, column), elevation, steps)) = current_states.pop_front() {
            if (row, column) == goal {
                return steps;
            }

            let moves = [
                (row > 0).then_some((row - 1, column)),
                (row < grid.height - 1).then_some((row + 1, column)),
                (column > 0).then_some((row, column - 1)),
                (column < grid.width - 1).then_some((row, column + 1)),
            ];

            current_states.extend((moves.into_iter().flatten()).flat_map(
                |(new_row, new_column)| {
                    let new_elevation =
                        match grid.tiles[grid.get_index(new_row, new_column) as usize] {
                            b'S' => 0,
                            b'E' => 25,
                            x => (x - b'a') as i64,
                        };

                    if new_elevation <= elevation + 1
                        && previous_states.insert((new_row, new_column))
                    {
                        Some(((new_row, new_column), new_elevation, steps + 1))
                    } else {
                        None
                    }
                },
            ));
        }
    }
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let width = input.lines().next().value()?.len() as i64;
    let height = input.lines().count() as i64;
    let tiles = input.lines().flat_map(|x| x.bytes()).collect_vec();
    let grid = Grid::new(width, height, tiles)?;

    let goal = grid.get_position(grid.tiles.iter().position(|&x| x == b'E').value()? as i64);

    let result1 = shortest_path(&grid, goal, b'S');
    let result2 = shortest_path(&grid, goal, b'a');

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
