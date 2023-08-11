use aoc::*;

use eyre::{bail, ensure};
use itertools::{iproduct, Itertools};
use regex::Regex;

const WATER_SPRING_X_COORD: i64 = 500;

#[derive(Copy, Clone, Eq, PartialEq)]
enum Tile {
    Sand,
    Clay,
    UnstableWater,
    StableWater,
}

struct Grid {
    width: usize,
    tiles: Vec<Tile>,
    min_width: i64,
    max_width: i64,
    min_depth: i64,
    max_depth: i64,
}

impl Grid {
    fn new(width: usize, height: usize, tiles: Vec<Tile>, min_width: i64, max_width: i64, min_depth: i64, max_depth: i64) -> Result<Self> {
        ensure!(width * height == tiles.len(), "unable to construct Grid: width * height != tiles.len()");
        Ok(Self { width, tiles, min_width, max_width, min_depth, max_depth })
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
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let re = Regex::new(r#"(?m)^(x|y)=(\d+), (x|y)=(\d+)\.\.(\d+)$"#)?;

    let clay_areas: Vec<_> = re
        .captures_iter(&input)
        .map(|cap| {
            let single_coord = cap[2].parse()?;
            let range_coord = cap[4].parse()?..=cap[5].parse()?;

            match (&cap[1], &cap[3]) {
                ("x", "y") => Ok((single_coord..=single_coord, range_coord)),
                ("y", "x") => Ok((range_coord, single_coord..=single_coord)),
                _ => bail!("unable to parse line"),
            }
        })
        .try_collect()?;

    let x_y_min_max = clay_areas.iter().fold(
        (WATER_SPRING_X_COORD, WATER_SPRING_X_COORD, i64::MAX, i64::MIN),
        |(min_width, max_width, min_depth, max_depth), (width, depth)| {
            (min_width.min(*width.start()), max_width.max(*width.end()), min_depth.min(*depth.start()), max_depth.max(*depth.end()))
        },
    );

    let min_width = x_y_min_max.0 - 1;
    let max_width = x_y_min_max.1 + 1;
    let min_depth = x_y_min_max.2 - 1;
    let max_depth = x_y_min_max.3 + 1;

    let width = usize::try_from(max_width - min_width + 1)?;
    let height = usize::try_from(max_depth - min_depth + 1)?;
    let tiles = vec![Tile::Sand; width * height];

    let mut grid = Grid::new(width, height, tiles, min_width, max_width, min_depth, max_depth)?;

    for (width_range, depth_range) in clay_areas {
        for (x, y) in iproduct!(width_range, depth_range) {
            *grid.tile_mut(x, y)? = Tile::Clay;
        }
    }

    let mut flows = vec![(WATER_SPRING_X_COORD, grid.min_depth)];
    *grid.tile_mut(WATER_SPRING_X_COORD, grid.min_depth)? = Tile::UnstableWater;

    'flow: while let Some(flow) = flows.pop() {
        let (x, mut y) = flow;

        if grid.tile(x, y)? == Tile::StableWater {
            continue;
        }

        loop {
            if y == grid.max_depth {
                continue 'flow;
            }

            if grid.tile(x, y + 1)? != Tile::Sand {
                break;
            }

            *grid.tile_mut(x, y + 1)? = Tile::UnstableWater;
            y += 1;
        }

        loop {
            let mut x_left = x;
            let left_blocked = loop {
                if x_left == grid.min_width {
                    break false;
                }

                match grid.tile(x_left - 1, y)? {
                    Tile::Sand | Tile::UnstableWater => match grid.tile(x_left - 1, y + 1)? {
                        Tile::Clay | Tile::StableWater => {
                            *grid.tile_mut(x_left - 1, y)? = Tile::UnstableWater;
                            x_left -= 1;
                        }
                        Tile::Sand => {
                            *grid.tile_mut(x_left - 1, y)? = Tile::UnstableWater;
                            flows.push((x_left - 1, y));
                            break false;
                        }
                        Tile::UnstableWater => break false,
                    },
                    Tile::Clay => break true,
                    Tile::StableWater => bail!("unable to follow flow"),
                }
            };

            let mut x_right = x;
            let right_blocked = loop {
                if x_right == grid.max_width {
                    break false;
                }

                match grid.tile(x_right + 1, y)? {
                    Tile::Sand | Tile::UnstableWater => match grid.tile(x_right + 1, y + 1)? {
                        Tile::Clay | Tile::StableWater => {
                            *grid.tile_mut(x_right + 1, y)? = Tile::UnstableWater;
                            x_right += 1;
                        }
                        Tile::Sand => {
                            *grid.tile_mut(x_right + 1, y)? = Tile::UnstableWater;
                            flows.push((x_right + 1, y));
                            break false;
                        }
                        Tile::UnstableWater => break false,
                    },
                    Tile::Clay => break true,
                    Tile::StableWater => bail!("unable to follow flow"),
                }
            };

            if !left_blocked || !right_blocked {
                break;
            }

            for x_between in x_left..=x_right {
                *grid.tile_mut(x_between, y)? = Tile::StableWater;
            }

            if y == grid.min_depth {
                break;
            }

            y -= 1;
            *grid.tile_mut(x, y)? = Tile::UnstableWater;
        }
    }

    let included_tiles = &grid.tiles[width..grid.tiles.len() - width];

    let (water, stable_water) = included_tiles.iter().fold((0usize, 0usize), |(water, stable_water), tile| match tile {
        Tile::UnstableWater => (water + 1, stable_water),
        Tile::StableWater => (water + 1, stable_water + 1),
        _ => (water, stable_water),
    });

    let result1 = water;
    let result2 = stable_water;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
