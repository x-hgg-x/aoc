use aoc::*;

use itertools::Itertools;

use std::cmp::Ordering;
use std::ops::RangeInclusive;

struct Grid {
    width: usize,
    height: usize,
    x_range: RangeInclusive<i64>,
    y_range: RangeInclusive<i64>,
    tiles: Vec<(Option<usize>, i64)>,
}

impl Grid {
    fn new(x_range: RangeInclusive<i64>, y_range: RangeInclusive<i64>) -> Self {
        let width = (x_range.end() - x_range.start() + 1) as usize;
        let height = (y_range.end() - y_range.start() + 1) as usize;
        Self { width, height, x_range, y_range, tiles: vec![(None, i64::MAX); width * height] }
    }

    fn get_coordinates(index: usize, width: usize, x_range: &RangeInclusive<i64>, y_range: &RangeInclusive<i64>) -> (i64, i64) {
        let row = index / width;
        let column = index % width;
        (column as i64 + x_range.start(), row as i64 + y_range.start())
    }
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let coordinates: Vec<(i64, i64)> =
        input.lines().map(|line| line.split(',').map(|x| Ok(x.trim().parse()?)).try_process(|mut iter| iter.next_tuple())?.value()).try_collect()?;

    let (min_x, min_y, max_x, max_y) =
        coordinates.iter().fold((i64::MAX, i64::MAX, i64::MIN, i64::MIN), |(acc_min_x, acc_min_y, acc_max_x, acc_max_y), &(x, y)| {
            (acc_min_x.min(x), acc_min_y.min(y), acc_max_x.max(x), acc_max_y.max(y))
        });

    let mut grid = Grid::new(2 * min_x - max_x..=2 * max_x - min_x, 2 * min_y - max_y..=2 * max_y - min_y);

    for (index, value) in grid.tiles.iter_mut().enumerate() {
        let (x, y) = Grid::get_coordinates(index, grid.width, &grid.x_range, &grid.y_range);

        let mut distance_sum = 0;
        let mut coord_min_index = None;
        let mut distance_min = i64::MAX;

        for (coord_index, distance) in coordinates.iter().map(|&(x_pt, y_pt)| ((x - x_pt).abs() + (y - y_pt).abs())).enumerate() {
            distance_sum += distance;

            match distance.cmp(&distance_min) {
                Ordering::Less => {
                    coord_min_index = Some(coord_index);
                    distance_min = distance;
                }
                Ordering::Equal => {
                    coord_min_index = None;
                }
                _ => (),
            };
        }

        *value = (coord_min_index, distance_sum);
    }

    let mut area_counts = vec![0usize; coordinates.len()];
    for &value in grid.tiles.iter().map(|(x, _)| x).flatten() {
        area_counts[value] += 1;
    }

    let iter1 = grid.tiles[..grid.width].iter().map(|&(x, _)| x);
    let iter2 = grid.tiles[(grid.height - 1) * grid.width..].iter().map(|&(x, _)| x);
    let iter3 = grid.tiles.chunks_exact(grid.width).flat_map(|row| [row[0].0, row[grid.width - 1].0]);
    iter1.chain(iter2).chain(iter3).flatten().for_each(|side_value| area_counts[side_value] = 0);

    let result1 = area_counts.iter().max().value()?;
    let result2 = grid.tiles.iter().filter(|&&(_, x)| x < 10000).count();

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
