use aoc::*;

use eyre::{bail, ensure, eyre};
use itertools::Itertools;

use std::iter::{once, repeat_n};

#[derive(Copy, Clone)]
enum Tile {
    Empty,
    Start,
    NorthSouth,
    WestEast,
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast,
}

struct Grid {
    width: usize,
    tiles: Vec<Tile>,
}

impl Grid {
    fn new(width: usize, height: usize, tiles: Vec<Tile>) -> Result<Self> {
        ensure!(
            width * height == tiles.len(),
            "unable to construct Grid: width * height != tiles.len()"
        );

        Ok(Self { width, tiles })
    }

    fn get_index(&self, row: i64, column: i64) -> usize {
        row as usize * self.width + column as usize
    }

    fn get_position(&self, index: usize) -> (i64, i64) {
        let row = index / self.width;
        let column = index % self.width;
        (row as i64, column as i64)
    }
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let width = input.lines().next().value()?.len() + 2;
    let height = input.lines().count() + 2;

    let tiles = repeat_n(Tile::Empty, width)
        .chain(input.lines().flat_map(|line| {
            once(Tile::Empty)
                .chain(line.bytes().filter_map(|x| match x {
                    b'.' => Some(Tile::Empty),
                    b'S' => Some(Tile::Start),
                    b'|' => Some(Tile::NorthSouth),
                    b'-' => Some(Tile::WestEast),
                    b'J' => Some(Tile::NorthWest),
                    b'L' => Some(Tile::NorthEast),
                    b'7' => Some(Tile::SouthWest),
                    b'F' => Some(Tile::SouthEast),
                    _ => None,
                }))
                .chain(once(Tile::Empty))
        }))
        .chain(repeat_n(Tile::Empty, width))
        .collect_vec();

    let grid = Grid::new(width, height, tiles)?;

    let start_index = grid
        .tiles
        .iter()
        .position(|tile| matches!(tile, Tile::Start))
        .value()?;

    let (start_row, start_column) = grid.get_position(start_index);

    if !((1..height as i64 - 1).contains(&start_row)
        && (1..width as i64 - 1).contains(&start_column))
    {
        bail!("invalid start position");
    };

    let mut state = 'state: {
        for (row_diff, column_diff) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let tile = grid.tiles[grid.get_index(start_row + row_diff, start_column + column_diff)];

            if let ((-1, 0), Tile::SouthWest | Tile::SouthEast | Tile::NorthSouth)
            | ((1, 0), Tile::NorthWest | Tile::NorthEast | Tile::NorthSouth)
            | ((0, -1), Tile::NorthEast | Tile::SouthEast | Tile::WestEast)
            | ((0, 1), Tile::NorthWest | Tile::SouthWest | Tile::WestEast) =
                ((row_diff, column_diff), tile)
            {
                break 'state ((start_row, start_column), (row_diff, column_diff), 1);
            }
        }

        bail!("invalid start position");
    };

    let mut cleaned_tiles = vec![Tile::Empty; grid.tiles.len()];

    let start_diff = state.1;

    let (steps, end_diff) = loop {
        let ((row, column), (row_diff, column_diff), steps) = state;

        let (new_row, new_column) = (row + row_diff, column + column_diff);

        if (new_row, new_column) == (start_row, start_column) {
            break (steps, (-row_diff, -column_diff));
        }

        let new_index = grid.get_index(new_row, new_column);
        let new_tile = grid.tiles[grid.get_index(new_row, new_column)];

        cleaned_tiles[new_index] = new_tile;

        let (new_row_diff, new_column_diff) = match new_tile {
            Tile::NorthSouth if row_diff == -1 => (-1, 0),
            Tile::NorthSouth if row_diff == 1 => (1, 0),
            Tile::WestEast if column_diff == -1 => (0, -1),
            Tile::WestEast if column_diff == 1 => (0, 1),
            Tile::NorthWest if row_diff == 1 => (0, -1),
            Tile::NorthWest if column_diff == 1 => (-1, 0),
            Tile::NorthEast if row_diff == 1 => (0, 1),
            Tile::NorthEast if column_diff == -1 => (-1, 0),
            Tile::SouthWest if row_diff == -1 => (0, -1),
            Tile::SouthWest if column_diff == 1 => (1, 0),
            Tile::SouthEast if row_diff == -1 => (0, 1),
            Tile::SouthEast if column_diff == -1 => (1, 0),
            _ => bail!("invalid input"),
        };

        if !((0..height as i64).contains(&(new_row + new_row_diff))
            && (0..width as i64).contains(&(new_column + new_column_diff)))
        {
            bail!("invalid input");
        };

        state = (
            (new_row, new_column),
            (new_row_diff, new_column_diff),
            steps + 1,
        );
    };

    let result1 = steps / 2;

    let start_tile = match (start_diff, end_diff) {
        ((-1, 0), (1, 0)) | ((1, 0), (-1, 0)) => Tile::NorthSouth,
        ((0, -1), (0, 1)) | ((0, 1), (0, -1)) => Tile::WestEast,
        ((-1, 0), (0, -1)) | ((0, -1), (-1, 0)) => Tile::NorthWest,
        ((-1, 0), (0, 1)) | ((0, 1), (-1, 0)) => Tile::NorthEast,
        ((1, 0), (0, -1)) | ((0, -1), (1, 0)) => Tile::SouthWest,
        ((1, 0), (0, 1)) | ((0, 1), (1, 0)) => Tile::SouthEast,
        _ => bail!("invalid input"),
    };

    cleaned_tiles[start_index] = start_tile;

    let result2 = cleaned_tiles
        .chunks_exact(width)
        .flat_map(|line| {
            line.windows(2).scan((0u64, None), |state, tile| {
                match [tile[0], tile[1]] {
                    [
                        Tile::Empty | Tile::NorthSouth | Tile::NorthWest | Tile::SouthWest,
                        Tile::NorthSouth,
                    ]
                    | [Tile::NorthEast, Tile::NorthWest]
                    | [Tile::SouthEast, Tile::SouthWest] => state.0 += 1,
                    [
                        Tile::Empty | Tile::NorthSouth | Tile::NorthWest | Tile::SouthWest,
                        Tile::NorthEast | Tile::SouthEast,
                    ] => {
                        *state = (state.0 + 1, Some(tile[1]));
                    }
                    [Tile::WestEast, Tile::NorthWest | Tile::SouthWest] => {
                        let Some(left_tile) = state.1.take() else {
                            return Some(Err(eyre!("invalid input")));
                        };
                        if matches!(
                            (left_tile, tile[1]),
                            (Tile::NorthEast, Tile::NorthWest) | (Tile::SouthEast, Tile::SouthWest)
                        ) {
                            state.0 += 1;
                        }
                    }
                    _ => (),
                };

                let is_inside = matches!(tile[1], Tile::Empty) && !state.0.is_multiple_of(2);

                Some(Ok(is_inside as u64))
            })
        })
        .try_sum::<u64>()?;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
