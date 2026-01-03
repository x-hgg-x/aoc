use aoc::*;

use eyre::ensure;
use itertools::Itertools;
use num_complex::Complex;
use smallvec::SmallVec;

use std::collections::{HashMap, HashSet};
use std::iter::repeat_n;

const NORTH: Complex<i64> = Complex::new(0, 1);
const SOUTH: Complex<i64> = Complex::new(0, -1);
const WEST: Complex<i64> = Complex::new(-1, 0);
const EAST: Complex<i64> = Complex::new(1, 0);
const DIRECTIONS: [Complex<i64>; 4] = [NORTH, SOUTH, WEST, EAST];

#[derive(Copy, Clone)]
enum Tile {
    Empty,
    Forest,
    SlopeNorth,
    SlopeSouth,
    SlopeWest,
    SlopeEast,
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

    fn get_index(&self, position: &Complex<i64>) -> usize {
        (self.height - 1 - position.im as usize) * self.width + position.re as usize
    }

    fn get_position(&self, index: usize) -> Complex<i64> {
        let row = index / self.width;
        let column = index % self.width;
        Complex::new(column as i64, (self.height - 1 - row) as i64)
    }

    fn moves(
        &self,
        position: &Complex<i64>,
        last_direction: &Complex<i64>,
        ignore_slopes: bool,
    ) -> SmallVec<[Complex<i64>; 3]> {
        DIRECTIONS
            .into_iter()
            .filter(move |&direction| direction != -last_direction)
            .filter(move |&direction| {
                ignore_slopes
                    || match self.tiles[self.get_index(position)] {
                        Tile::SlopeNorth => direction == NORTH,
                        Tile::SlopeSouth => direction == SOUTH,
                        Tile::SlopeWest => direction == WEST,
                        Tile::SlopeEast => direction == EAST,
                        _ => true,
                    }
            })
            .filter(|direction| {
                let new_position = position + direction;

                (0..self.width as i64).contains(&new_position.re)
                    && (0..self.height as i64).contains(&new_position.im)
                    && !matches!(self.tiles[self.get_index(&new_position)], Tile::Forest)
            })
            .collect()
    }
}

struct Graph {
    positions: Vec<Vec<(usize, u64)>>,
    start_position_index: usize,
    goal_position_index: usize,
}

impl Graph {
    fn new(
        grid: &Grid,
        start_position: Complex<i64>,
        goal_position: Complex<i64>,
        ignore_slopes: bool,
    ) -> Graph {
        let mut intersection_graph = Vec::new();
        let mut intersection_positions = HashMap::new();

        let mut insert_position = |position, next_position, steps| {
            let position_index = {
                let len = intersection_positions.len();
                *intersection_positions.entry(position).or_insert(len)
            };

            let next_position_index = {
                let len = intersection_positions.len();
                *intersection_positions.entry(next_position).or_insert(len)
            };

            let max_index = position_index.max(next_position_index);
            if intersection_graph.len() <= max_index {
                let additional = max_index + 1 - intersection_graph.len();
                intersection_graph.extend(repeat_n(Vec::new(), additional));
            }

            intersection_graph[position_index].push((next_position_index, steps));
        };

        let mut current_states = vec![(start_position, SOUTH)];
        let mut visited = HashSet::new();

        while let Some((position, mut next_direction)) = current_states.pop() {
            if !visited.insert((position, next_direction)) {
                continue;
            }

            let mut next_position = position + next_direction;

            let mut steps = 1u64;

            loop {
                match &grid.moves(&next_position, &next_direction, ignore_slopes)[..] {
                    [] => {
                        if next_position == goal_position {
                            insert_position(position, next_position, steps);
                        }
                        break;
                    }
                    &[new_direction] => {
                        next_position += new_direction;
                        next_direction = new_direction;
                    }
                    new_directions => {
                        insert_position(position, next_position, steps);

                        current_states.extend(
                            (new_directions.iter())
                                .map(|&new_direction| (next_position, new_direction))
                                .chain([(next_position, -next_direction)]),
                        );

                        break;
                    }
                }

                steps += 1;
            }
        }

        Self {
            positions: intersection_graph,
            start_position_index: intersection_positions[&start_position],
            goal_position_index: intersection_positions[&goal_position],
        }
    }

    fn compute_longest_path(&self) -> u64 {
        let mut max_steps = 0;
        let mut current_states = vec![(self.start_position_index, 0u64, 0)];

        while let Some((position_index, visited, steps)) = current_states.pop() {
            if position_index == self.goal_position_index {
                max_steps = max_steps.max(steps);
                continue;
            }

            current_states.extend(
                (self.positions[position_index].iter())
                    .filter(|&&(new_position_index, _)| visited & (1 << new_position_index) == 0)
                    .map(|&(new_position_index, distance)| {
                        (
                            new_position_index,
                            visited | (1 << new_position_index),
                            steps + distance,
                        )
                    }),
            );
        }

        max_steps
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
            b'#' => Some(Tile::Forest),
            b'^' => Some(Tile::SlopeNorth),
            b'v' => Some(Tile::SlopeSouth),
            b'<' => Some(Tile::SlopeWest),
            b'>' => Some(Tile::SlopeEast),
            _ => None,
        })
        .collect_vec();

    let grid = Grid::new(width, height, tiles)?;

    let start_position = (grid.tiles.iter())
        .position(|tile| matches!(tile, Tile::Empty))
        .map(|index| grid.get_position(index))
        .value()?;

    let goal_position = (grid.tiles.iter().enumerate().rev())
        .find(|(_, tile)| matches!(tile, Tile::Empty))
        .map(|(index, _)| grid.get_position(index))
        .value()?;

    let result1 = Graph::new(&grid, start_position, goal_position, false).compute_longest_path();
    let result2 = Graph::new(&grid, start_position, goal_position, true).compute_longest_path();

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
