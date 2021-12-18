use aoc::*;

use eyre::ensure;
use itertools::Itertools;
use regex::Regex;

use std::iter::once;

struct Grid {
    width: usize,
    height: usize,
    tiles: Vec<u8>,
}

impl Grid {
    fn new(width: usize, height: usize, tiles: Vec<u8>) -> Result<Self> {
        ensure!(width * height == tiles.len(), "unable to construct Grid: width * height != tiles.len()");
        Ok(Self { width, height, tiles })
    }

    fn get_index(&self, row: usize, column: usize) -> usize {
        row * self.width + column
    }
}

fn compute_bounds(positions: &[(i64, i64)], velocities: &[(i64, i64)], time: i64) -> (i64, i64, i64, i64) {
    positions.iter().zip(velocities).fold(
        (i64::MAX, i64::MAX, i64::MIN, i64::MIN),
        |(xmin, ymin, xmax, ymax), (&(mut position_x, mut position_y), &(velocity_x, velocity_y))| {
            position_x += time * velocity_x;
            position_y += time * velocity_y;
            (xmin.min(position_x), ymin.min(position_y), xmax.max(position_x), ymax.max(position_y))
        },
    )
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let re = Regex::new(r#"(?m)^position=<(.+?), (.+?)> velocity=<(.+?), (.+?)>$"#)?;

    let (mut positions, velocities): (Vec<_>, Vec<_>) = re
        .captures_iter(&input)
        .map(|cap| {
            let position_x = cap[1].trim().parse()?;
            let position_y = cap[2].trim().parse()?;
            let velocity_x = cap[3].trim().parse()?;
            let velocity_y = cap[4].trim().parse()?;

            Ok(((position_x, position_y), (velocity_x, velocity_y)))
        })
        .try_process(|iter| iter.unzip())?;

    let message_time = (0..)
        .scan((i64::MAX, i64::MAX), |(x_bounds_size, y_bounds_size), time| {
            let (new_xmin, new_ymin, new_xmax, new_ymax) = compute_bounds(&positions, &velocities, time);
            let (new_x_bounds_size, new_y_bounds_size) = (new_xmax - new_xmin, new_ymax - new_ymin);

            if new_x_bounds_size <= *x_bounds_size && new_y_bounds_size <= *y_bounds_size {
                *x_bounds_size = new_x_bounds_size.min(*x_bounds_size);
                *y_bounds_size = new_y_bounds_size.min(*y_bounds_size);
                Some(time)
            } else {
                None
            }
        })
        .last()
        .value()?;

    let (xmin, ymin, xmax, ymax) = compute_bounds(&positions, &velocities, message_time);
    for ((position_x, position_y), &(velocity_x, velocity_y)) in positions.iter_mut().zip(&velocities) {
        *position_x += message_time * velocity_x;
        *position_y += message_time * velocity_y;
    }

    let width = (xmax - xmin + 1) as usize;
    let height = (ymax - ymin + 1) as usize;
    let mut grid = Grid::new(width, height, vec![b' '; width * height])?;

    for &(position_x, position_y) in &positions {
        let index = grid.get_index((position_y - ymin) as usize, (position_x - xmin) as usize);
        grid.tiles[index] = b'#';
    }

    let message = grid.tiles.chunks_exact(grid.width).take(grid.height).flat_map(|row| row.iter().copied().chain(once(b'\n'))).collect_vec();

    let result1 = String::from_utf8_lossy(&message);
    let result2 = message_time;

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
