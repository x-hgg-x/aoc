use aoc::*;

use eyre::{bail, ensure};
use itertools::Itertools;
use num_complex::Complex;

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::hash_map::{Entry, HashMap};

const NORTH: Complex<i64> = Complex::new(0, 1);
const SOUTH: Complex<i64> = Complex::new(0, -1);
const WEST: Complex<i64> = Complex::new(-1, 0);
const EAST: Complex<i64> = Complex::new(1, 0);
const DIRECTIONS: [Complex<i64>; 4] = [NORTH, SOUTH, WEST, EAST];

struct Grid {
    width: usize,
    height: usize,
    tiles: Vec<u8>,
}

impl Grid {
    fn new(width: usize, height: usize, tiles: Vec<u8>) -> Result<Self> {
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

    fn get_index(&self, position: &Complex<i64>) -> usize {
        (self.height - 1 - position.im as usize) * self.width + position.re as usize
    }
}

#[derive(Clone)]
struct State {
    position: Complex<i64>,
    direction: Complex<i64>,
    direction_count: usize,
    total_heat_loss: i64,
}

impl State {
    fn estimate(&self) -> i64 {
        self.total_heat_loss
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

fn filter_map_normal_crucible_direction(
    state: &State,
    new_direction: Complex<i64>,
) -> Option<(Complex<i64>, usize)> {
    if new_direction != state.direction {
        Some((new_direction, 0))
    } else {
        (state.direction_count < 2).then_some((new_direction, state.direction_count + 1))
    }
}

fn filter_map_ultra_crucible_direction(
    state: &State,
    new_direction: Complex<i64>,
) -> Option<(Complex<i64>, usize)> {
    if new_direction == state.direction {
        (state.direction_count < 9).then_some((new_direction, state.direction_count + 1))
    } else {
        (state.direction_count > 2).then_some((new_direction, 0))
    }
}

type FilterMapFn = fn(&State, Complex<i64>) -> Option<(Complex<i64>, usize)>;

fn compute_min_total_heat_loss(
    current_states: &mut BinaryHeap<State>,
    visited: &mut HashMap<(Complex<i64>, Complex<i64>, usize), i64>,
    grid: &Grid,
    filter_map: FilterMapFn,
) -> Result<i64> {
    let goal = Complex::new(grid.width as i64 - 1, 0);

    let initial_position = Complex::new(0, grid.height as i64 - 1);

    visited.clear();

    current_states.clear();
    current_states.extend([EAST, SOUTH].map(|direction| State {
        position: initial_position,
        direction,
        direction_count: 0,
        total_heat_loss: 0,
    }));

    loop {
        match current_states.pop() {
            None => bail!("unable to find a path to the goal"),
            Some(state) => {
                if state.position == goal {
                    break Ok(state.total_heat_loss);
                }

                match visited.entry((state.position, state.direction, state.direction_count)) {
                    Entry::Vacant(entry) => {
                        entry.insert(state.total_heat_loss);
                    }
                    Entry::Occupied(mut entry) => {
                        if state.total_heat_loss < *entry.get() {
                            entry.insert(state.total_heat_loss);
                        } else {
                            continue;
                        }
                    }
                };

                let new_position = state.position + state.direction;
                let heat_loss = grid.tiles[grid.get_index(&new_position)] as i64;

                current_states.extend(
                    DIRECTIONS
                        .into_iter()
                        .filter(|&new_direction| new_direction != -state.direction)
                        .filter(|new_direction| {
                            let next_position = new_position + new_direction;
                            (0..grid.width as i64).contains(&next_position.re)
                                && (0..grid.height as i64).contains(&next_position.im)
                        })
                        .filter_map(|new_direction| filter_map(&state, new_direction))
                        .map(|(new_direction, new_direction_count)| State {
                            position: new_position,
                            direction: new_direction,
                            direction_count: new_direction_count,
                            total_heat_loss: state.total_heat_loss + heat_loss,
                        }),
                );
            }
        }
    }
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let width = input.lines().next().value()?.len();
    let height = input.lines().count();

    let tiles = input
        .lines()
        .flat_map(|line| line.bytes().map(|x| x - b'0'))
        .collect_vec();

    let grid = Grid::new(width, height, tiles)?;

    let mut current_states = BinaryHeap::new();
    let mut visited = HashMap::new();

    let result1 = compute_min_total_heat_loss(
        &mut current_states,
        &mut visited,
        &grid,
        filter_map_normal_crucible_direction,
    )?;

    let result2 = compute_min_total_heat_loss(
        &mut current_states,
        &mut visited,
        &grid,
        filter_map_ultra_crucible_direction,
    )?;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
