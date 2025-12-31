use aoc::*;

use eyre::{bail, ensure};
use itertools::Itertools;
use num_complex::Complex;

use std::iter;

const NORTH: Complex<i64> = Complex::new(0, 1);
const SOUTH: Complex<i64> = Complex::new(0, -1);
const WEST: Complex<i64> = Complex::new(-1, 0);
const EAST: Complex<i64> = Complex::new(1, 0);

#[derive(Copy, Clone)]
#[repr(u8)]
enum BeamFlag {
    North = 1 << 0,
    South = 1 << 1,
    West = 1 << 2,
    East = 1 << 3,
}

impl BeamFlag {
    fn try_from_direction(c: &Complex<i64>) -> Result<Self> {
        match *c {
            NORTH => Ok(Self::North),
            SOUTH => Ok(Self::South),
            WEST => Ok(Self::West),
            EAST => Ok(Self::East),
            _ => bail!("invalid direction"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Tile {
    Empty,
    SplitHorizontal,
    SplitVertical,
    SlashMirror,
    BackslashMirror,
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
}

fn compute_energized_tile_count(
    grid: &Grid,
    visited_beams: &mut [u8],
    current_states: &mut Vec<(Complex<i64>, Complex<i64>)>,
) -> Result<u64> {
    while let Some((position, direction)) = current_states.pop() {
        if !(0..grid.width as i64).contains(&position.re)
            || !(0..grid.height as i64).contains(&position.im)
        {
            continue;
        }

        let index = grid.get_index(&position);
        let beam_flag = BeamFlag::try_from_direction(&direction)?;

        let visited_beam = &mut visited_beams[index];
        let new_visited_beam = *visited_beam | beam_flag as u8;

        if *visited_beam != new_visited_beam {
            *visited_beam = new_visited_beam;
        } else {
            continue;
        }

        match grid.tiles[index] {
            Tile::Empty => current_states.push((position + direction, direction)),
            Tile::SplitHorizontal => {
                if direction.re != 0 {
                    current_states.push((position + direction, direction));
                } else {
                    current_states
                        .extend_from_slice(&[(position + WEST, WEST), (position + EAST, EAST)]);
                }
            }
            Tile::SplitVertical => {
                if direction.im != 0 {
                    current_states.push((position + direction, direction));
                } else {
                    current_states
                        .extend_from_slice(&[(position + NORTH, NORTH), (position + SOUTH, SOUTH)]);
                }
            }
            Tile::SlashMirror => {
                let new_direction = match direction {
                    NORTH => EAST,
                    SOUTH => WEST,
                    WEST => SOUTH,
                    EAST => NORTH,
                    _ => bail!("invalid direction"),
                };
                current_states.push((position + new_direction, new_direction));
            }
            Tile::BackslashMirror => {
                let new_direction = match direction {
                    NORTH => WEST,
                    SOUTH => EAST,
                    WEST => NORTH,
                    EAST => SOUTH,
                    _ => bail!("invalid direction"),
                };
                current_states.push((position + new_direction, new_direction));
            }
        }
    }

    Ok(visited_beams.iter().filter(|&&x| x != 0).count() as u64)
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
            b'-' => Some(Tile::SplitHorizontal),
            b'|' => Some(Tile::SplitVertical),
            b'/' => Some(Tile::SlashMirror),
            b'\\' => Some(Tile::BackslashMirror),
            _ => None,
        })
        .collect_vec();

    let grid = Grid::new(width, height, tiles)?;

    let mut visited_beams = vec![0; grid.tiles.len()];
    let mut current_states = vec![(Complex::new(0, grid.height as i64 - 1), EAST)];

    let result1 = compute_energized_tile_count(&grid, &mut visited_beams, &mut current_states)?;

    let result2 = iter::chain(
        (0..grid.width as i64).flat_map(|re| {
            [
                (Complex::new(re, 0), NORTH),
                (Complex::new(re, grid.height as i64 - 1), SOUTH),
            ]
        }),
        (0..grid.height as i64).flat_map(|im| {
            [
                (Complex::new(0, im), EAST),
                (Complex::new(grid.width as i64 - 1, im), WEST),
            ]
        }),
    )
    .map(|initial_state| {
        visited_beams.fill(0);
        current_states.clear();
        current_states.push(initial_state);

        compute_energized_tile_count(&grid, &mut visited_beams, &mut current_states)
    })
    .try_process(|iter| iter.max())?
    .value()?;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
