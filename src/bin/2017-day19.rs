use eyre::{bail, ensure, Result};
use itertools::Itertools;

use std::fs;
use std::iter::repeat;

enum Tile {
    Empty,
    Letter(u8),
    HorizontalLine,
    VerticalLine,
    Intersection,
}

struct Grid {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
}

impl Grid {
    fn new(width: usize, height: usize, tiles: Vec<Tile>) -> Result<Self> {
        ensure!(width * height == tiles.len(), "unable to construct Grid: width * height != tiles.len()");
        Ok(Self { width, height, tiles })
    }

    fn get_index(&self, row: usize, column: usize) -> usize {
        row * self.width + column
    }
}

fn main() -> Result<()> {
    let input = fs::read_to_string("inputs/2017-day19.txt")?;

    let start_index = i64::try_from(input.bytes().position(|x| x == b'|').unwrap())?;
    let width = input.lines().map(|line| line.len()).max().unwrap();
    let height = input.lines().count();

    let tiles = input
        .lines()
        .flat_map(|line| line.bytes().chain(repeat(b' ')).take(width))
        .filter_map(|x| match x {
            b' ' => Some(Tile::Empty),
            x @ b'A'..=b'Z' => Some(Tile::Letter(x)),
            b'-' => Some(Tile::HorizontalLine),
            b'|' => Some(Tile::VerticalLine),
            b'+' => Some(Tile::Intersection),
            _ => None,
        })
        .collect_vec();

    let grid = Grid::new(width, height, tiles)?;

    let mut letters = Vec::new();
    let mut count = 0usize;

    let mut current_row = 0i64;
    let mut current_column = start_index;
    let mut row_direction = 1;
    let mut column_direction = 0;

    loop {
        current_row += row_direction;
        current_column += column_direction;
        count += 1;

        let row = current_row.try_into()?;
        let column = current_column.try_into()?;

        match grid.tiles[grid.get_index(row, column)] {
            Tile::Empty => break,
            Tile::Letter(c) => letters.push(c),
            Tile::Intersection => {
                if row_direction != 0 {
                    if column > 0 && matches!(grid.tiles[grid.get_index(row, column - 1)], Tile::HorizontalLine | Tile::Letter(_)) {
                        row_direction = 0;
                        column_direction = -1;
                        continue;
                    }
                    if column < grid.width - 1 && matches!(grid.tiles[grid.get_index(row, column + 1)], Tile::HorizontalLine | Tile::Letter(_)) {
                        row_direction = 0;
                        column_direction = 1;
                        continue;
                    }
                }

                if column_direction != 0 {
                    if row > 0 && matches!(grid.tiles[grid.get_index(row - 1, column)], Tile::VerticalLine | Tile::Letter(_)) {
                        row_direction = -1;
                        column_direction = 0;
                        continue;
                    }
                    if row < grid.height - 1 && matches!(grid.tiles[grid.get_index(row + 1, column)], Tile::VerticalLine | Tile::Letter(_)) {
                        row_direction = 1;
                        column_direction = 0;
                        continue;
                    }
                }

                bail!("unable to follow path at (row, column) = {:?}", (row, column))
            }
            _ => (),
        }
    }

    let result1 = String::from_utf8_lossy(&letters);
    let result2 = count;

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
