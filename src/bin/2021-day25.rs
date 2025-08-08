use aoc::*;

use eyre::{bail, ensure};
use itertools::Itertools;

#[derive(Copy, Clone)]
enum Tile {
    Empty,
    EastCucumber,
    SouthCucumber,
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

    fn get_index(&self, row: usize, column: usize) -> usize {
        row * self.width + column
    }
}

fn step_east(grid: &mut Grid, buf: &mut Vec<Tile>) -> bool {
    buf.clear();
    buf.extend_from_slice(&grid.tiles);

    let mut locked = true;

    for (i_row, row) in grid.tiles.chunks_exact(grid.width).enumerate() {
        for (i_col, (x1, x2)) in row
            .iter()
            .cycle()
            .tuple_windows()
            .take(grid.width)
            .enumerate()
        {
            if let (Tile::EastCucumber, Tile::Empty) = (x1, x2) {
                let index1 = grid.get_index(i_row, i_col);
                let index2 = grid.get_index(i_row, (i_col + 1) % grid.width);
                buf.swap(index1, index2);
                locked = false;
            }
        }
    }

    std::mem::swap(&mut grid.tiles, buf);

    locked
}

fn step_south(grid: &mut Grid, buf: &mut Vec<Tile>) -> bool {
    buf.clear();
    buf.extend_from_slice(&grid.tiles);

    let mut locked = true;

    for (i_row, (row1, row2)) in grid
        .tiles
        .chunks_exact(grid.width)
        .cycle()
        .tuple_windows()
        .take(grid.height)
        .enumerate()
    {
        for (i_col, (x1, x2)) in row1.iter().zip(row2).enumerate() {
            if let (Tile::SouthCucumber, Tile::Empty) = (x1, x2) {
                let index1 = grid.get_index(i_row, i_col);
                let index2 = grid.get_index((i_row + 1) % grid.height, i_col);
                buf.swap(index1, index2);
                locked = false;
            }
        }
    }

    std::mem::swap(&mut grid.tiles, buf);

    locked
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let tiles = input
        .lines()
        .flat_map(|line| {
            line.bytes().map(|x| match x {
                b'.' => Ok(Tile::Empty),
                b'>' => Ok(Tile::EastCucumber),
                b'v' => Ok(Tile::SouthCucumber),
                _ => bail!("unknown tile"),
            })
        })
        .try_collect()?;

    let width = input.lines().next().value()?.len();
    let height = input.lines().count();

    let mut grid = Grid::new(width, height, tiles)?;
    let mut buf = Vec::with_capacity(grid.tiles.len());

    let mut step_count = 0;
    let mut locked_east = false;
    let mut locked_south = false;

    while !locked_east || !locked_south {
        locked_east = step_east(&mut grid, &mut buf);
        locked_south = step_south(&mut grid, &mut buf);
        step_count += 1;
    }

    let result = step_count;

    println!("{result}");
    Ok(())
}
