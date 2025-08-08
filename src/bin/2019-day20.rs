use aoc::*;

use eyre::{bail, ensure};
use itertools::Itertools;
use smallvec::SmallVec;

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::iter::repeat;

type Point = (usize, usize);

struct Map {
    width: usize,
    tiles: Vec<u8>,
    portals: HashMap<Point, Point>,
}

impl Map {
    fn new(width: usize, height: usize, tiles: Vec<u8>, portals: HashMap<Point, Point>) -> Result<Self> {
        ensure!(width * height == tiles.len(), "unable to construct Map: width * height != tiles.len()");
        Ok(Self { width, tiles, portals })
    }

    fn get_index(&self, (row, column): Point) -> usize {
        row * self.width + column
    }
}

fn parse_map(input: &str) -> Result<(Map, Point, Point)> {
    let lines = input.lines().collect_vec();

    let is_inside = |x| matches!(x, b'#' | b'.');
    let is_inside_at = |line: &str, index| line.as_bytes().get(index).map(|&x| is_inside(x)) == Some(true);

    let mut line_iter = lines[lines.len() / 2].bytes().enumerate();
    let min_x_left = line_iter.find(|&(_, x)| is_inside(x)).map(|(index, _)| index).value()?;
    let max_x_left = line_iter.find(|&(_, x)| !is_inside(x)).map(|(index, _)| index).value()? - 1;
    let min_x_right = line_iter.find(|&(_, x)| is_inside(x)).map(|(index, _)| index).value()?;
    let max_x_right = line_iter.rev().find(|&(_, x)| is_inside(x)).map(|(index, _)| index).value()?;

    let mut lines_iter = lines.iter().enumerate();
    let min_y_top = lines_iter.find(|&(_, &line)| is_inside_at(line, min_x_left)).map(|(index, _)| index).value()?;
    let max_y_top = lines_iter.find(|&(_, &line)| !is_inside_at(line, max_x_left + 1)).map(|(index, _)| index).value()? - 1;
    let min_y_bottom = lines_iter.find(|&(_, &line)| is_inside_at(line, max_x_left + 1)).map(|(index, _)| index).value()?;
    let max_y_bottom = lines_iter.rev().find(|&(_, &line)| is_inside_at(line, min_x_left)).map(|(index, _)| index).value()?;

    let width = lines.iter().map(|&line| line.len()).max().value()?;
    let height = lines.len();
    let tiles = lines.iter().flat_map(|&line| line.bytes().chain(repeat(b' ')).take(width)).collect_vec();

    let mut map = Map::new(width, height, tiles, HashMap::new())?;

    let mut portals_entries = HashMap::<_, SmallVec<[_; 2]>>::new();

    map.tiles.chunks_exact_mut(width).skip(min_y_top - 2).next_tuple().into_iter().for_each(|(row_0, row_1, row_2)| {
        for (i_col, tile) in row_2.iter_mut().enumerate().filter(|(_, tile)| **tile == b'.') {
            *tile = b'-';
            portals_entries.entry([row_0[i_col], row_1[i_col]]).or_default().push((min_y_top, i_col));
        }
    });
    map.tiles.chunks_exact_mut(width).skip(max_y_bottom).next_tuple().into_iter().for_each(|(row_0, row_1, row_2)| {
        for (i_col, tile) in row_0.iter_mut().enumerate().filter(|(_, tile)| **tile == b'.') {
            *tile = b'-';
            portals_entries.entry([row_1[i_col], row_2[i_col]]).or_default().push((max_y_bottom, i_col));
        }
    });
    map.tiles.chunks_exact_mut(width).skip(min_y_bottom - 2).next_tuple().into_iter().for_each(|(row_0, row_1, row_2)| {
        for (i_col, tile) in row_2.iter_mut().enumerate().skip(max_x_left + 1).take(min_x_right - max_x_left - 1).filter(|(_, tile)| **tile == b'.') {
            *tile = b'+';
            portals_entries.entry([row_0[i_col], row_1[i_col]]).or_default().push((min_y_bottom, i_col));
        }
    });
    map.tiles.chunks_exact_mut(width).skip(max_y_top).next_tuple().into_iter().for_each(|(row_0, row_1, row_2)| {
        for (i_col, tile) in row_0.iter_mut().enumerate().skip(max_x_left + 1).take(min_x_right - max_x_left - 1).filter(|(_, tile)| **tile == b'.') {
            *tile = b'+';
            portals_entries.entry([row_1[i_col], row_2[i_col]]).or_default().push((max_y_top, i_col));
        }
    });

    for y in min_y_top..=max_y_bottom {
        let index = map.get_index((y, min_x_left));
        let tile = map.tiles.get_mut(index).value()?;
        if *tile == b'.' {
            *tile = b'-';
            portals_entries.entry([map.tiles[index - 2], map.tiles[index - 1]]).or_default().push((y, min_x_left));
        }
    }
    for y in min_y_top..=max_y_bottom {
        let index = map.get_index((y, max_x_right));
        let tile = map.tiles.get_mut(index).value()?;
        if *tile == b'.' {
            *tile = b'-';
            portals_entries.entry([map.tiles[index + 1], map.tiles[index + 2]]).or_default().push((y, max_x_right));
        }
    }
    for y in max_y_top + 1..=min_y_bottom - 1 {
        let index = map.get_index((y, min_x_right));
        let tile = map.tiles.get_mut(index).value()?;
        if *tile == b'.' {
            *tile = b'+';
            portals_entries.entry([map.tiles[index - 2], map.tiles[index - 1]]).or_default().push((y, min_x_right));
        }
    }
    for y in max_y_top + 1..=min_y_bottom - 1 {
        let index = map.get_index((y, max_x_left));
        let tile = map.tiles.get_mut(index).value()?;
        if *tile == b'.' {
            *tile = b'+';
            portals_entries.entry([map.tiles[index + 1], map.tiles[index + 2]]).or_default().push((y, max_x_left));
        }
    }

    let start_position = portals_entries[b"AA"][0];
    let goal_position = portals_entries[b"ZZ"][0];

    for portal_positions in portals_entries.into_values() {
        if let &[first, second] = portal_positions.as_slice() {
            map.portals.insert(first, second);
            map.portals.insert(second, first);
        }
    }

    Ok((map, start_position, goal_position))
}

fn solve(map: &Map, start_position: Point, goal_position: Point, has_depth: bool) -> Result<usize> {
    let mut visited = HashMap::new();
    let mut queue = BinaryHeap::from([Reverse((0usize, 0usize, start_position))]);

    while let Some(Reverse((depth, distance, position))) = queue.pop() {
        if position == goal_position && (depth == 0 || !has_depth) {
            return Ok(distance);
        }

        if let Some(old_distance) = visited.insert((position, depth), distance)
            && old_distance <= distance
        {
            continue;
        }

        match map.tiles[map.get_index(position)] {
            b'+' => {
                if let Some(&new_position) = map.portals.get(&position) {
                    queue.push(Reverse((depth + 1, distance + 1, new_position)));
                }
            }
            b'-' if depth > 0 => {
                if let Some(&new_position) = map.portals.get(&position) {
                    queue.push(Reverse((depth - 1, distance + 1, new_position)));
                }
            }
            _ => (),
        }

        for new_position in [(position.0 - 1, position.1), (position.0 + 1, position.1), (position.0, position.1 - 1), (position.0, position.1 + 1)] {
            if matches!(map.tiles[map.get_index(new_position)], b'.' | b'+' | b'-') {
                queue.push(Reverse((depth, distance + 1, new_position)));
            }
        }
    }

    bail!("unable to solve maze")
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let (map, start_position, goal_position) = parse_map(&input)?;

    let result1 = solve(&map, start_position, goal_position, false)?;
    let result2 = solve(&map, start_position, goal_position, true)?;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
