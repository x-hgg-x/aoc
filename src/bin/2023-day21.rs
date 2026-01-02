use aoc::*;

use eyre::ensure;
use itertools::Itertools;
use num_complex::Complex;

use std::collections::VecDeque;
use std::collections::hash_map::{Entry, HashMap};

const NORTH: Complex<i64> = Complex::new(0, 1);
const SOUTH: Complex<i64> = Complex::new(0, -1);
const WEST: Complex<i64> = Complex::new(-1, 0);
const EAST: Complex<i64> = Complex::new(1, 0);
const DIRECTIONS: [Complex<i64>; 4] = [NORTH, SOUTH, WEST, EAST];

#[derive(Copy, Clone)]
enum Tile {
    Start,
    Empty,
    Rock,
}

struct Grid {
    size: usize,
    tiles: Vec<Tile>,
}

impl Grid {
    fn new(size: usize, tiles: Vec<Tile>) -> Result<Self> {
        ensure!(
            size * size == tiles.len(),
            "unable to construct Grid: size * size != tiles.len()"
        );

        Ok(Self { size, tiles })
    }

    fn get_index(&self, position: &Complex<i64>) -> usize {
        (self.size - 1 - position.im as usize) * self.size + position.re as usize
    }

    fn get_position(&self, index: usize) -> Complex<i64> {
        let row = index / self.size;
        let column = index % self.size;
        Complex::new(column as i64, (self.size - 1 - row) as i64)
    }
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let width = input.lines().next().value()?.len();
    let height = input.lines().count();

    ensure!(width == height, "unsupported input");

    let size = width;

    let tiles = input
        .bytes()
        .filter_map(|x| match x {
            b'S' => Some(Tile::Start),
            b'.' => Some(Tile::Empty),
            b'#' => Some(Tile::Rock),
            _ => None,
        })
        .collect_vec();

    let grid = Grid::new(size, tiles)?;

    let start_position = grid
        .tiles
        .iter()
        .position(|tile| matches!(tile, Tile::Start))
        .map(|index| grid.get_position(index))
        .value()?;

    let mut current_states = VecDeque::from([(start_position, 0u64)]);
    let mut visited = HashMap::from([(start_position, 0)]);

    while let Some((position, steps)) = current_states.pop_front() {
        current_states.extend(
            DIRECTIONS
                .into_iter()
                .map(|direction| (position + direction, steps + 1))
                .filter(|&(new_position, _)| {
                    (0..grid.size as i64).contains(&new_position.re)
                        && (0..grid.size as i64).contains(&new_position.im)
                        && !matches!(grid.tiles[grid.get_index(&new_position)], Tile::Rock)
                })
                .filter(
                    |&(new_position, new_steps)| match visited.entry(new_position) {
                        Entry::Vacant(entry) => {
                            entry.insert(new_steps);
                            true
                        }
                        Entry::Occupied(mut entry) => {
                            if new_steps < *entry.get() {
                                entry.insert(new_steps);
                                true
                            } else {
                                false
                            }
                        }
                    },
                ),
        );
    }

    let result1 = visited
        .values()
        .filter(|&&steps| steps <= 64 && steps.is_multiple_of(2))
        .count();

    let n = (26501365 - grid.size / 2) / grid.size;

    let mut odd_tiles = 0;
    let mut even_tiles = 0;
    let mut odd_corners = 0;
    let mut even_corners = 0;

    for &steps in visited.values() {
        if steps.is_multiple_of(2) {
            even_tiles += 1;
            if steps as usize > grid.size / 2 {
                even_corners += 1;
            }
        } else {
            odd_tiles += 1;
            if steps as usize > grid.size / 2 {
                odd_corners += 1;
            }
        }
    }

    let result2 = (n + 1).pow(2) * odd_tiles + n.pow(2) * even_tiles - (n + 1) * odd_corners
        + n * even_corners;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
