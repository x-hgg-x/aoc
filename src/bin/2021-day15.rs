use aoc::*;

use eyre::ensure;
use itertools::Itertools;

use std::cmp::Ordering;
use std::collections::hash_map::{Entry, HashMap};
use std::collections::BinaryHeap;

struct Grid {
    width: i64,
    height: i64,
    tiles: Vec<u8>,
}

impl Grid {
    fn new(width: i64, height: i64, tiles: Vec<u8>) -> Result<Self> {
        ensure!((width * height) as usize == tiles.len(), "unable to construct Grid: width * height != tiles.len()");
        Ok(Self { width, height, tiles })
    }

    fn get_index(&self, row: usize, column: usize) -> usize {
        row * self.width as usize + column
    }
}

struct State {
    x: i64,
    y: i64,
    risk: i64,
    goal_distance: i64,
}

impl State {
    fn new(x: i64, y: i64, risk: i64, goal_x: i64, goal_y: i64) -> Self {
        let goal_distance = goal_x - x + goal_y - y;
        Self { x, y, risk, goal_distance }
    }

    fn estimate(&self) -> (i64, i64) {
        (self.risk, self.goal_distance)
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

fn lowest_risk_path(grid: &Grid, goal_x: i64, goal_y: i64) -> i64 {
    let mut previous_states = HashMap::new();
    let mut current_states = BinaryHeap::from([State::new(0, 0, 0, goal_x, goal_y)]);

    loop {
        if let Some(state) = current_states.pop() {
            if (state.x, state.y) == (goal_x, goal_y) {
                break state.risk;
            }

            match previous_states.entry((state.x, state.y)) {
                Entry::Occupied(mut entry) => {
                    let risk = entry.get_mut();
                    if *risk <= state.risk {
                        continue;
                    }
                    *risk = state.risk;
                }
                Entry::Vacant(entry) => {
                    entry.insert(state.risk);
                }
            }

            let moves = [
                (state.x > 0).then_some((state.x - 1, state.y)),
                (state.x < goal_x).then_some((state.x + 1, state.y)),
                (state.y > 0).then_some((state.x, state.y - 1)),
                (state.y < goal_y).then_some((state.x, state.y + 1)),
            ];

            current_states.extend(moves.into_iter().flatten().map(|(x, y)| {
                let row = (y % grid.height) as usize;
                let column = (x % grid.width) as usize;
                let tile_risk = grid.tiles[grid.get_index(row, column)] as i64;
                let risk = state.risk + (tile_risk + x / grid.width + y / grid.height - 1) % 9 + 1;
                State::new(x, y, risk, goal_x, goal_y)
            }));
        }
    }
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let width = input.lines().next().value()?.len() as i64;
    let height = input.lines().count() as i64;
    let tiles = input.lines().flat_map(|line| line.bytes().map(|x| x - b'0')).collect_vec();
    let grid = Grid::new(width, height, tiles)?;

    let result1 = lowest_risk_path(&grid, width - 1, height - 1);
    let result2 = lowest_risk_path(&grid, 5 * width - 1, 5 * height - 1);

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
