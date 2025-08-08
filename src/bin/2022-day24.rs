use aoc::*;

use eyre::{bail, ensure};
use itertools::Itertools;

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::iter;

const EMPTY: u8 = 0;
const WALL: u8 = 1 << 0;
const BLIZZARD_LEFT: u8 = 1 << 1;
const BLIZZARD_RIGHT: u8 = 1 << 2;
const BLIZZARD_UP: u8 = 1 << 3;
const BLIZZARD_DOWN: u8 = 1 << 4;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Tile(u8);

#[derive(Clone)]
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

    fn get_index(&self, row: usize, column: usize) -> usize {
        row * self.width + column
    }

    fn step(&self) -> Self {
        let mut new_tiles = self
            .tiles
            .iter()
            .map(|&tile| {
                if tile == Tile(WALL) {
                    tile
                } else {
                    Tile(EMPTY)
                }
            })
            .collect_vec();

        for (i_row, row) in self.tiles.chunks_exact(self.width).enumerate() {
            for (i_col, &tile) in row.iter().enumerate() {
                [
                    (tile.0 & BLIZZARD_LEFT != 0).then(|| {
                        let new_col = if i_col >= 2 {
                            i_col - 1
                        } else {
                            self.width - 2
                        };
                        (BLIZZARD_LEFT, i_row, new_col)
                    }),
                    (tile.0 & BLIZZARD_RIGHT != 0).then(|| {
                        let new_col = if i_col <= self.width - 3 {
                            i_col + 1
                        } else {
                            1
                        };
                        (BLIZZARD_RIGHT, i_row, new_col)
                    }),
                    (tile.0 & BLIZZARD_UP != 0).then(|| {
                        let new_row = if i_row >= 2 {
                            i_row - 1
                        } else {
                            self.height - 2
                        };
                        (BLIZZARD_UP, new_row, i_col)
                    }),
                    (tile.0 & BLIZZARD_DOWN != 0).then(|| {
                        let new_row = if i_row <= self.height - 3 {
                            i_row + 1
                        } else {
                            1
                        };
                        (BLIZZARD_DOWN, new_row, i_col)
                    }),
                ]
                .into_iter()
                .flatten()
                .for_each(|(blizzard, new_row, new_col)| {
                    new_tiles[self.get_index(new_row, new_col)].0 |= blizzard;
                });
            }
        }

        Self {
            width: self.width,
            height: self.height,
            tiles: new_tiles,
        }
    }
}

#[derive(Clone)]
struct State {
    position: (usize, usize),
    steps: usize,
    distance: usize,
}

impl State {
    fn new(
        position: (usize, usize),
        (goal_row, goal_column): (usize, usize),
        steps: usize,
    ) -> Self {
        let (row, column) = position;
        let distance = row.abs_diff(goal_row) + column.abs_diff(goal_column);

        Self {
            position,
            steps,
            distance,
        }
    }

    fn estimate(&self) -> usize {
        self.steps + self.distance
    }
}

impl Eq for State {}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.estimate().eq(&other.estimate())
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.estimate().cmp(&self.estimate())
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn gcd(mut x: usize, mut y: usize) -> usize {
    while y != 0 {
        (x, y) = (y, x % y);
    }
    x
}

fn lcm(x: usize, y: usize) -> usize {
    x * y / gcd(x, y)
}

fn find_shortest_path(
    start_position: (usize, usize),
    goal_position: (usize, usize),
    start_time: usize,
    cache: &[Grid],
) -> Result<usize> {
    let mut previous_states = HashSet::new();
    let mut current_states = BinaryHeap::from([State::new(start_position, goal_position, 0)]);

    loop {
        match current_states.pop() {
            None => bail!("unable to find a path to the goal"),
            Some(state) => {
                if state.position == goal_position {
                    break Ok(state.steps);
                }

                if !previous_states.insert((state.position, state.steps % cache.len())) {
                    continue;
                }

                let (row, column) = state.position;
                let grid = &cache[(start_time + state.steps + 1) % cache.len()];

                current_states.extend(
                    [
                        Some((row, column)),
                        row.checked_sub(1).map(|new_row| (new_row, column)),
                        column.checked_sub(1).map(|new_column| (row, new_column)),
                        (row < grid.height - 1).then_some((row + 1, column)),
                        (column < grid.width - 1).then_some((row, column + 1)),
                    ]
                    .into_iter()
                    .flatten()
                    .filter(|&(new_row, new_column)| {
                        grid.tiles[grid.get_index(new_row, new_column)] == Tile(EMPTY)
                    })
                    .map(|(new_row, new_column)| {
                        State::new((new_row, new_column), goal_position, state.steps + 1)
                    }),
                );
            }
        }
    }
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let tiles = input
        .lines()
        .flat_map(|line| {
            line.bytes().map(|x| match x {
                b'.' => Ok(Tile(EMPTY)),
                b'#' => Ok(Tile(WALL)),
                b'<' => Ok(Tile(BLIZZARD_LEFT)),
                b'>' => Ok(Tile(BLIZZARD_RIGHT)),
                b'^' => Ok(Tile(BLIZZARD_UP)),
                b'v' => Ok(Tile(BLIZZARD_DOWN)),
                _ => bail!("unknown tile"),
            })
        })
        .try_collect()?;

    let width = input.lines().next().value()?.len();
    let height = input.lines().count();
    let grid = Grid::new(width, height, tiles)?;

    let cycle_size = lcm(width - 2, height - 2);

    let cache = iter::successors(Some(grid.clone()), |grid| Some(grid.step()))
        .take(cycle_size)
        .collect_vec();

    let first_empty = grid.tiles.iter().position(|&x| x == Tile(EMPTY)).value()?;

    let last_empty = grid
        .tiles
        .iter()
        .rev()
        .position(|&x| x == Tile(EMPTY))
        .value()?;

    let start_position = (0, first_empty);
    let goal_position = (height - 1, width - 1 - last_empty);

    let trip1 = find_shortest_path(start_position, goal_position, 0, &cache)?;
    let trip2 = find_shortest_path(goal_position, start_position, trip1, &cache)?;
    let trip3 = find_shortest_path(start_position, goal_position, trip1 + trip2, &cache)?;

    let result1 = trip1;
    let result2 = trip1 + trip2 + trip3;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
