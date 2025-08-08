use aoc::*;

use eyre::ensure;
use itertools::{Itertools, iproduct};

const SAND_START_X_COORD: i64 = 500;

#[derive(Copy, Clone, Eq, PartialEq)]
enum Tile {
    Air,
    Sand,
    Rock,
}

struct Grid {
    width: usize,
    tiles: Vec<Tile>,
    min_width: i64,
    min_depth: i64,
}

impl Grid {
    fn new(
        width: usize,
        height: usize,
        tiles: Vec<Tile>,
        min_width: i64,
        min_depth: i64,
    ) -> Result<Self> {
        ensure!(
            width * height == tiles.len(),
            "unable to construct Grid: width * height != tiles.len()"
        );

        Ok(Self {
            width,
            tiles,
            min_width,
            min_depth,
        })
    }

    fn get_index(&self, x: i64, y: i64) -> Result<usize> {
        let row = usize::try_from(y - self.min_depth)?;
        let column = usize::try_from(x - self.min_width)?;
        Ok(row * self.width + column)
    }

    fn tile(&self, x: i64, y: i64) -> Result<Tile> {
        let tile_index = self.get_index(x, y)?;
        Ok(self.tiles[tile_index])
    }

    fn tile_mut(&mut self, x: i64, y: i64) -> Result<&mut Tile> {
        let tile_index = self.get_index(x, y)?;
        Ok(&mut self.tiles[tile_index])
    }

    fn sand_count(&self) -> usize {
        self.tiles.iter().filter(|&&x| x == Tile::Sand).count()
    }
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let mut rock_areas = Vec::new();

    input.lines().try_for_each(|line| {
        line.split(" -> ")
            .map(|coord| {
                coord
                    .split(',')
                    .map(|x| Ok(x.parse::<i64>()?))
                    .try_process(|mut iter| iter.next_tuple())?
                    .value()
            })
            .try_process(|iter| {
                rock_areas.extend(iter.tuple_windows().map(|((x1, y1), (x2, y2))| {
                    let range_x = if x1 <= x2 { x1..=x2 } else { x2..=x1 };
                    let range_y = if y1 <= y2 { y1..=y2 } else { y2..=y1 };
                    (range_x, range_y)
                }))
            })
    })?;

    let min_depth = 0;

    let max_depth = 1 + rock_areas
        .iter()
        .fold(i64::MIN, |max_depth, (_, y)| max_depth.max(*y.end()));

    let min_width = SAND_START_X_COORD - max_depth;
    let max_width = SAND_START_X_COORD + max_depth;

    let width = usize::try_from(max_width - min_width + 1)?;
    let height = usize::try_from(max_depth - min_depth + 1)?;
    let tiles = vec![Tile::Air; width * height];

    let mut grid = Grid::new(width, height, tiles, min_width, min_depth)?;

    for (width_range, depth_range) in rock_areas {
        for (x, y) in iproduct!(width_range, depth_range) {
            *grid.tile_mut(x, y)? = Tile::Rock;
        }
    }

    let mut sand_count_bottom = None;

    'stop: loop {
        let (mut x, mut y) = (SAND_START_X_COORD, 0);

        loop {
            if y == max_depth {
                if sand_count_bottom.is_none() {
                    sand_count_bottom = Some(grid.sand_count());
                }
                *grid.tile_mut(x, y)? = Tile::Sand;
                break;
            } else if grid.tile(x, y + 1)? == Tile::Air {
                y += 1;
            } else if grid.tile(x - 1, y + 1)? == Tile::Air {
                x -= 1;
                y += 1;
            } else if grid.tile(x + 1, y + 1)? == Tile::Air {
                x += 1;
                y += 1;
            } else {
                *grid.tile_mut(x, y)? = Tile::Sand;
                if (x, y) == (SAND_START_X_COORD, 0) {
                    break 'stop;
                } else {
                    break;
                }
            }
        }
    }

    let result1 = sand_count_bottom.value()?;
    let result2 = grid.tiles.iter().filter(|&&x| x == Tile::Sand).count();

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
