use aoc::*;

use eyre::ensure;
use itertools::Itertools;

use std::collections::HashMap;
use std::iter::{self, repeat_n};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Tile {
    Empty,
    FixedRock,
    MovingRock,
}

struct Grid {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
}

impl Grid {
    fn new(width: usize, height: usize, tiles: Vec<Tile>) -> Result<Self> {
        ensure!(
            width * height == tiles.len(),
            "unable to construct Grid: width * height != tiles.len()"
        );

        Ok(Self {
            width,
            height,
            tiles,
        })
    }

    fn tilt_vertical(&mut self, up: bool) {
        for column in 0..self.width {
            let mut region_start = 0;

            while region_start < self.height {
                let mut count = 0usize;

                let len = self
                    .tiles
                    .iter()
                    .skip(column)
                    .step_by(self.width)
                    .skip(region_start)
                    .take_while(|tile| !matches!(tile, Tile::FixedRock))
                    .inspect(|tile| {
                        if matches!(tile, Tile::MovingRock) {
                            count += 1;
                        }
                    })
                    .count();

                let values = if up {
                    iter::chain(
                        repeat_n(Tile::MovingRock, count),
                        repeat_n(Tile::Empty, len - count),
                    )
                } else {
                    iter::chain(
                        repeat_n(Tile::Empty, len - count),
                        repeat_n(Tile::MovingRock, count),
                    )
                };

                let tile_iter = self
                    .tiles
                    .iter_mut()
                    .skip(column)
                    .step_by(self.width)
                    .skip(region_start);

                for (tile, value) in iter::zip(tile_iter, values) {
                    *tile = value;
                }

                region_start += len + 1;
            }
        }
    }

    fn tilt_horizontal(&mut self, left: bool) {
        for tiles in self.tiles.chunks_exact_mut(self.width) {
            for region in tiles.split_mut(|tile| matches!(tile, Tile::FixedRock)) {
                let len = region.len();

                let count = region
                    .iter()
                    .filter(|tile| matches!(tile, Tile::MovingRock))
                    .count();

                if count > 0 {
                    let values = if left {
                        iter::chain(
                            repeat_n(Tile::MovingRock, count),
                            repeat_n(Tile::Empty, len - count),
                        )
                    } else {
                        iter::chain(
                            repeat_n(Tile::Empty, len - count),
                            repeat_n(Tile::MovingRock, count),
                        )
                    };

                    for (tile, value) in iter::zip(region, values) {
                        *tile = value;
                    }
                }
            }
        }
    }

    fn cycle(&mut self) {
        self.tilt_vertical(true);
        self.tilt_horizontal(true);
        self.tilt_vertical(false);
        self.tilt_horizontal(false);
    }

    fn compute_load(&self) -> u64 {
        iter::zip(
            (1..=self.width as u64).rev(),
            self.tiles.chunks_exact(self.width),
        )
        .map(|(coeff, tiles)| {
            let count = tiles
                .iter()
                .filter(|tile| matches!(tile, Tile::MovingRock))
                .count() as u64;

            coeff * count
        })
        .sum()
    }
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let width = input.lines().next().value()?.len();
    let height = input.lines().count();

    let tiles = input
        .bytes()
        .filter_map(|x| match x {
            b'.' => Some(Tile::Empty),
            b'#' => Some(Tile::FixedRock),
            b'O' => Some(Tile::MovingRock),
            _ => None,
        })
        .collect_vec();

    let mut grid = Grid::new(width, height, tiles)?;

    let mut visited = HashMap::from([(grid.tiles.clone(), 0)]);

    grid.tilt_vertical(true);

    let result1 = grid.compute_load();

    grid.tilt_horizontal(true);
    grid.tilt_vertical(false);
    grid.tilt_horizontal(false);

    let mut steps = 1u64;

    let cycle_size = loop {
        if let Some(old_steps) = visited.insert(grid.tiles.clone(), steps) {
            break steps - old_steps;
        }

        grid.cycle();

        steps += 1;
    };

    for _ in 0..(1_000_000_000 - steps) % cycle_size {
        grid.cycle();
    }

    let result2 = grid.compute_load();

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
