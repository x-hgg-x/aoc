use std::collections::{HashSet, VecDeque};

use aoc::*;

use eyre::bail;
use itertools::iproduct;

const SIZE: usize = 5;

const CENTER_UP_BUG: u32 = 1 << 7;
const CENTER_DOWN_BUG: u32 = 1 << 17;
const CENTER_LEFT_BUG: u32 = 1 << 11;
const CENTER_RIGHT_BUG: u32 = 1 << 13;

const UP_CENTER_BUGS: u32 = (1 << 1) + (1 << 2) + (1 << 3);
const DOWN_CENTER_BUGS: u32 = (1 << 21) + (1 << 22) + (1 << 23);
const LEFT_CENTER_BUGS: u32 = (1 << 5) + (1 << 10) + (1 << 15);
const RIGHT_CENTER_BUGS: u32 = (1 << 9) + (1 << 14) + (1 << 19);

const UP_LEFT_CORNER_BUG: u32 = 1 << 0;
const UP_RIGHT_CORNER_BUG: u32 = 1 << 4;
const DOWN_LEFT_CORNER_BUG: u32 = 1 << 20;
const DOWN_RIGHT_CORNER_BUG: u32 = 1 << 24;

const UP_BUGS: u32 = UP_LEFT_CORNER_BUG | UP_CENTER_BUGS | UP_RIGHT_CORNER_BUG;
const DOWN_BUGS: u32 = DOWN_LEFT_CORNER_BUG | DOWN_CENTER_BUGS | DOWN_RIGHT_CORNER_BUG;
const LEFT_BUGS: u32 = UP_LEFT_CORNER_BUG | LEFT_CENTER_BUGS | DOWN_LEFT_CORNER_BUG;
const RIGHT_BUGS: u32 = UP_RIGHT_CORNER_BUG | RIGHT_CENTER_BUGS | DOWN_RIGHT_CORNER_BUG;

const INSIDE_BUGS_LIST: [u32; 4] = [1 << 6, 1 << 8, 1 << 16, 1 << 18];
const UP_CENTER_BUGS_LIST: [u32; 3] = [1 << 1, 1 << 2, 1 << 3];
const DOWN_CENTER_BUGS_LIST: [u32; 3] = [1 << 21, 1 << 22, 1 << 23];
const LEFT_CENTER_BUGS_LIST: [u32; 3] = [1 << 5, 1 << 10, 1 << 15];
const RIGHT_CENTER_BUGS_LIST: [u32; 3] = [1 << 9, 1 << 14, 1 << 19];

const CENTER_UP_BUG_NEIGHBORS: u32 =
    CENTER_UP_BUG >> SIZE | CENTER_UP_BUG >> 1 | CENTER_UP_BUG << 1;

const CENTER_DOWN_BUG_NEIGHBORS: u32 =
    CENTER_DOWN_BUG >> 1 | CENTER_DOWN_BUG << 1 | CENTER_DOWN_BUG << SIZE;

const CENTER_LEFT_BUG_NEIGHBORS: u32 =
    CENTER_LEFT_BUG >> SIZE | CENTER_LEFT_BUG >> 1 | CENTER_LEFT_BUG << SIZE;

const CENTER_RIGHT_BUG_NEIGHBORS: u32 =
    CENTER_RIGHT_BUG >> SIZE | CENTER_RIGHT_BUG << 1 | CENTER_RIGHT_BUG << SIZE;

const UP_LEFT_CORNER_BUG_NEIGHBORS: u32 = UP_LEFT_CORNER_BUG << 1 | UP_LEFT_CORNER_BUG << SIZE;
const UP_RIGHT_CORNER_BUG_NEIGHBORS: u32 = UP_RIGHT_CORNER_BUG >> 1 | UP_RIGHT_CORNER_BUG << SIZE;

const DOWN_LEFT_CORNER_BUG_NEIGHBORS: u32 =
    DOWN_LEFT_CORNER_BUG >> SIZE | DOWN_LEFT_CORNER_BUG << 1;

const DOWN_RIGHT_CORNER_BUG_NEIGHBORS: u32 =
    DOWN_RIGHT_CORNER_BUG >> SIZE | DOWN_RIGHT_CORNER_BUG >> 1;

fn update_bug(mut buffer: u32, bug: u32, neighbors_count: u32) -> u32 {
    if (buffer & bug) != 0 {
        if neighbors_count != 1 {
            buffer &= !bug;
        }
    } else if neighbors_count == 1 || neighbors_count == 2 {
        buffer |= bug;
    }
    buffer
}

fn step_without_recursion(grid: u32) -> u32 {
    let mut buffer = grid;

    for (i_row, i_col) in iproduct!(0..SIZE, 0..SIZE) {
        let bug = 1 << (i_row * SIZE + i_col);

        let mut neighbors = 0u32;

        if i_row > 0 {
            neighbors |= bug >> SIZE;
        }
        if i_row < SIZE - 1 {
            neighbors |= bug << SIZE;
        }
        if i_col > 0 {
            neighbors |= bug >> 1;
        }
        if i_col < SIZE - 1 {
            neighbors |= bug << 1;
        }

        buffer = update_bug(buffer, bug, (grid & neighbors).count_ones());
    }

    buffer
}

fn step_with_recursion(grids: &mut VecDeque<u32>, buffers: &mut VecDeque<u32>) -> Result<()> {
    for (depth_index, (&grid, buffer)) in grids.iter().zip(&mut *buffers).enumerate() {
        let grid_inside = *grids.get(depth_index + 1).unwrap_or(&0);

        let grid_outside = *depth_index
            .checked_sub(1)
            .and_then(|x| grids.get(x))
            .unwrap_or(&0);

        let inside_up_bugs_count = (grid_inside & UP_BUGS).count_ones();
        let inside_down_bugs_count = (grid_inside & DOWN_BUGS).count_ones();
        let inside_left_bugs_count = (grid_inside & LEFT_BUGS).count_ones();
        let inside_right_bugs_count = (grid_inside & RIGHT_BUGS).count_ones();

        let outside_center_up_bug = ((grid_outside & CENTER_UP_BUG) != 0) as u32;
        let outside_center_down_bug = ((grid_outside & CENTER_DOWN_BUG) != 0) as u32;
        let outside_center_left_bug = ((grid_outside & CENTER_LEFT_BUG) != 0) as u32;
        let outside_center_right_bug = ((grid_outside & CENTER_RIGHT_BUG) != 0) as u32;

        let outside_center_up_left_count = outside_center_up_bug + outside_center_left_bug;
        let outside_center_up_right_count = outside_center_up_bug + outside_center_right_bug;
        let outside_center_down_left_count = outside_center_down_bug + outside_center_left_bug;
        let outside_center_down_right_count = outside_center_down_bug + outside_center_right_bug;

        for bug in INSIDE_BUGS_LIST {
            let neighbors = bug >> SIZE | bug >> 1 | bug << 1 | bug << SIZE;
            *buffer = update_bug(*buffer, bug, (grid & neighbors).count_ones());
        }

        for bug in UP_CENTER_BUGS_LIST {
            let neighbors = bug >> 1 | bug << 1 | bug << SIZE;

            *buffer = update_bug(
                *buffer,
                bug,
                (grid & neighbors).count_ones() + outside_center_up_bug,
            );
        }
        for bug in DOWN_CENTER_BUGS_LIST {
            let neighbors = bug >> SIZE | bug >> 1 | bug << 1;

            *buffer = update_bug(
                *buffer,
                bug,
                (grid & neighbors).count_ones() + outside_center_down_bug,
            );
        }
        for bug in LEFT_CENTER_BUGS_LIST {
            let neighbors = bug >> SIZE | bug << 1 | bug << SIZE;

            *buffer = update_bug(
                *buffer,
                bug,
                (grid & neighbors).count_ones() + outside_center_left_bug,
            );
        }
        for bug in RIGHT_CENTER_BUGS_LIST {
            let neighbors = bug >> SIZE | bug >> 1 | bug << SIZE;

            *buffer = update_bug(
                *buffer,
                bug,
                (grid & neighbors).count_ones() + outside_center_right_bug,
            );
        }

        *buffer = update_bug(
            *buffer,
            CENTER_UP_BUG,
            (grid & CENTER_UP_BUG_NEIGHBORS).count_ones() + inside_up_bugs_count,
        );

        *buffer = update_bug(
            *buffer,
            CENTER_DOWN_BUG,
            (grid & CENTER_DOWN_BUG_NEIGHBORS).count_ones() + inside_down_bugs_count,
        );

        *buffer = update_bug(
            *buffer,
            CENTER_LEFT_BUG,
            (grid & CENTER_LEFT_BUG_NEIGHBORS).count_ones() + inside_left_bugs_count,
        );

        *buffer = update_bug(
            *buffer,
            CENTER_RIGHT_BUG,
            (grid & CENTER_RIGHT_BUG_NEIGHBORS).count_ones() + inside_right_bugs_count,
        );

        *buffer = update_bug(
            *buffer,
            UP_LEFT_CORNER_BUG,
            (grid & UP_LEFT_CORNER_BUG_NEIGHBORS).count_ones() + outside_center_up_left_count,
        );

        *buffer = update_bug(
            *buffer,
            UP_RIGHT_CORNER_BUG,
            (grid & UP_RIGHT_CORNER_BUG_NEIGHBORS).count_ones() + outside_center_up_right_count,
        );

        *buffer = update_bug(
            *buffer,
            DOWN_LEFT_CORNER_BUG,
            (grid & DOWN_LEFT_CORNER_BUG_NEIGHBORS).count_ones() + outside_center_down_left_count,
        );

        *buffer = update_bug(
            *buffer,
            DOWN_RIGHT_CORNER_BUG,
            (grid & DOWN_RIGHT_CORNER_BUG_NEIGHBORS).count_ones() + outside_center_down_right_count,
        );
    }

    let most_outside = *grids.front().value()?;
    let most_inside = *grids.back().value()?;

    let mut new_outside = 0;
    let mut new_inside = 0;

    if matches!((most_outside & UP_BUGS).count_ones(), 1 | 2) {
        new_outside |= CENTER_UP_BUG;
    }
    if matches!((most_outside & DOWN_BUGS).count_ones(), 1 | 2) {
        new_outside |= CENTER_DOWN_BUG;
    }
    if matches!((most_outside & LEFT_BUGS).count_ones(), 1 | 2) {
        new_outside |= CENTER_LEFT_BUG;
    }
    if matches!((most_outside & RIGHT_BUGS).count_ones(), 1 | 2) {
        new_outside |= CENTER_RIGHT_BUG;
    }

    if (most_inside & CENTER_UP_BUG) != 0 {
        new_inside |= UP_BUGS;
    }
    if (most_inside & CENTER_DOWN_BUG) != 0 {
        new_inside |= DOWN_BUGS;
    }
    if (most_inside & CENTER_LEFT_BUG) != 0 {
        new_inside |= LEFT_BUGS;
    }
    if (most_inside & CENTER_RIGHT_BUG) != 0 {
        new_inside |= RIGHT_BUGS;
    }

    if new_outside != 0 {
        buffers.push_front(new_outside);
    }
    if new_inside != 0 {
        buffers.push_back(new_inside);
    }

    grids.clear();
    grids.extend(&*buffers);

    Ok(())
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let initial_grid = input
        .lines()
        .flat_map(|line| line.bytes())
        .enumerate()
        .map(|(index, x)| match x {
            b'.' => Ok(0),
            b'#' => Ok(1 << index),
            other => bail!("unknown tile: {other}"),
        })
        .try_sum::<u32>()?;

    let mut grid = initial_grid;
    let mut visited = HashSet::new();

    while visited.insert(grid) {
        grid = step_without_recursion(grid);
    }

    let result1 = grid;

    let mut grids = VecDeque::from([initial_grid]);
    let mut buffers = grids.clone();

    for _ in 0..200 {
        step_with_recursion(&mut grids, &mut buffers)?;
    }

    let result2 = grids.iter().map(|grid| grid.count_ones()).sum::<u32>();

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
