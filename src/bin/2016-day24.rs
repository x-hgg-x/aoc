use aoc::*;

use eyre::ensure;
use itertools::Itertools;
use smallvec::SmallVec;

use std::cmp::Ordering;
use std::collections::hash_map::{Entry, HashMap};
use std::collections::BinaryHeap;
use std::iter::once;

struct Permutations<'a, T, const N: usize> {
    data: &'a [T],
    available: SmallVec<[T; N]>,
    buf: SmallVec<[T; N]>,
    factorials: Vec<i64>,
    factorial_index: i64,
}

impl<'a, T, const N: usize> Permutations<'a, T, N> {
    fn new(data: &'a [T]) -> Self {
        Self { data, available: SmallVec::new(), buf: SmallVec::new(), factorials: Self::compute_factorials(data.len() as i64), factorial_index: 0 }
    }

    fn compute_factorials(num: i64) -> Vec<i64> {
        once(1)
            .chain((1..=num).scan(1, |state, x| {
                *state *= x;
                Some(*state)
            }))
            .collect_vec()
    }
}

impl<'a, T: Copy, const N: usize> Iterator for Permutations<'a, T, N> {
    type Item = SmallVec<[T; N]>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.factorial_index >= self.factorials[self.data.len()] {
            return None;
        }

        let mut x = self.factorial_index;

        self.buf.clear();
        self.available = SmallVec::from_slice(self.data);

        self.buf.extend(self.factorials[..self.data.len()].iter().rev().map(|&place_value| {
            let index = x / place_value;
            x -= index * place_value;
            self.available.remove(index.rem_euclid(self.available.len() as i64) as usize)
        }));

        self.factorial_index += 1;

        Some(self.buf.clone())
    }
}

struct Grid {
    width: usize,
    height: usize,
    tiles: Vec<bool>,
}

impl Grid {
    fn new(width: usize, height: usize, tiles: Vec<bool>) -> Result<Self> {
        ensure!(width * height == tiles.len(), "unable to construct Grid: width * height != tiles.len()");
        Ok(Self { width, height, tiles })
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

struct State {
    position: (usize, usize),
    steps: usize,
    distance: usize,
}

impl State {
    fn new(position: (usize, usize), (goal_row, goal_column): (usize, usize), steps: usize) -> Self {
        let (row, column) = position;
        let distance = row.abs_diff(goal_row) + column.abs_diff(goal_column);
        Self { position, steps, distance }
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

fn compute_shortest_distance(grid: &Grid, initial_position: (usize, usize), goal_position: (usize, usize)) -> usize {
    let (initial_row, initial_column) = initial_position;
    let initial_index = grid.get_index(initial_row, initial_column);
    let initial_state = State::new(initial_position, goal_position, 0);

    let mut previous_positions = HashMap::from([(initial_index, initial_state.steps)]);
    let mut current_states = BinaryHeap::from([initial_state]);

    loop {
        if let Some(state) = current_states.pop() {
            if state.position == goal_position {
                break state.steps;
            }

            let (state_row, state_column) = state.position;

            let mut process_neighbors = |new_row, new_column| {
                let new_position = (new_row, new_column);
                let new_index = grid.get_index(new_row, new_column);
                let new_steps = state.steps + 1;

                if !grid.tiles[new_index] {
                    return;
                }

                match previous_positions.entry(new_index) {
                    Entry::Occupied(mut entry) => {
                        let old_steps = entry.get_mut();
                        if new_steps >= *old_steps {
                            return;
                        }
                        *old_steps = new_steps;
                    }
                    Entry::Vacant(entry) => {
                        entry.insert(new_steps);
                    }
                }

                current_states.push(State::new(new_position, goal_position, new_steps));
            };

            if state_row > 0 {
                process_neighbors(state_row - 1, state_column);
            }
            if state_row < grid.height - 1 {
                process_neighbors(state_row + 1, state_column);
            }
            if state_column > 0 {
                process_neighbors(state_row, state_column - 1);
            }
            if state_column < grid.width - 1 {
                process_neighbors(state_row, state_column + 1);
            }
        }
    }
}

fn compute_shortest_path<'a, I>(permutations: &'a [SmallVec<[u8; 8]>], distances: &HashMap<(u8, u8), usize>, iter_func: impl Fn(&'a [u8]) -> I) -> Result<usize>
where
    I: Iterator<Item = u8> + 'a,
{
    permutations
        .iter()
        .map(|path| {
            iter_func(path)
                .tuple_windows()
                .map(|(initial_location, goal_location)| {
                    distances[&if initial_location < goal_location { (initial_location, goal_location) } else { (goal_location, initial_location) }]
                })
                .sum::<usize>()
        })
        .min()
        .value()
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let mut locations_indices = Vec::new();
    let mut tiles = Vec::with_capacity(input.len());
    let mut index = 0;
    for x in input.bytes() {
        match x {
            b'#' => tiles.push(false),
            b'.' => tiles.push(true),
            x @ b'0'..=b'9' => {
                tiles.push(true);
                locations_indices.push((x - b'0', index));
            }
            _ => continue,
        }
        index += 1;
    }
    locations_indices.sort_unstable();

    let width = input.lines().next().value()?.len();
    let height = input.lines().count();
    let grid = Grid::new(width, height, tiles)?;

    let locations_positions = locations_indices.into_iter().map(|(n, index)| (n, grid.get_position(index))).collect_vec();

    let (first_location, _) = locations_positions[0];
    let other_locations = locations_positions[1..].iter().map(|&(location, _)| location).collect_vec();
    ensure!(first_location == 0, "unable to found first location");

    let distances: HashMap<_, _> = locations_positions
        .iter()
        .tuple_combinations()
        .map(|(&(initial_location, initial_position), &(goal_location, goal_position))| {
            let steps = compute_shortest_distance(&grid, initial_position, goal_position);
            let (location1, location2) = if initial_location < goal_location { (initial_location, goal_location) } else { (goal_location, initial_location) };
            ((location1, location2), steps)
        })
        .collect();

    let permutations = Permutations::<_, 8>::new(&other_locations).collect_vec();

    let result1 = compute_shortest_path(&permutations, &distances, |path| once(first_location).chain(path.iter().copied()))?;
    let result2 = compute_shortest_path(&permutations, &distances, |path| once(first_location).chain(path.iter().copied()).chain(once(first_location)))?;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
