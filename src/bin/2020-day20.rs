use aoc::*;

use eyre::{bail, ensure};
use itertools::{izip, Itertools};
use smallvec::SmallVec;

use std::collections::{HashMap, HashSet, VecDeque};
use std::iter::once;

const MONSTER_PIXEL_COUNT: usize = 15;
const MONSTER_WIDTH: usize = 20;
const MONSTER: [[bool; MONSTER_WIDTH]; 3] = [
    [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false],
    [true, false, false, false, false, true, true, false, false, false, false, true, true, false, false, false, false, true, true, true],
    [false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, false],
];

#[derive(Copy, Clone, Eq, PartialEq)]
struct Point {
    x: i8,
    y: i8,
}

impl Point {
    const TOP_LEFT: Self = Self::new(-1, -1);
    const TOP_RIGHT: Self = Self::new(1, -1);
    const BOTTOM_LEFT: Self = Self::new(-1, 1);
    const BOTTOM_RIGHT: Self = Self::new(1, 1);

    const TOP: Self = Self::new(0, -1);
    const BOTTOM: Self = Self::new(0, 1);
    const LEFT: Self = Self::new(-1, 0);
    const RIGHT: Self = Self::new(1, 0);

    const fn new(x: i8, y: i8) -> Self {
        Self { x, y }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Border {
    start: Point,
    end: Point,
}

impl Border {
    const TOP: Self = Self::new(Point::TOP_LEFT, Point::TOP_RIGHT);
    const BOTTOM: Self = Self::new(Point::BOTTOM_LEFT, Point::BOTTOM_RIGHT);
    const LEFT: Self = Self::new(Point::TOP_LEFT, Point::BOTTOM_LEFT);
    const RIGHT: Self = Self::new(Point::TOP_RIGHT, Point::BOTTOM_RIGHT);

    const TOP_REVERSE: Self = Self::new(Point::TOP_RIGHT, Point::TOP_LEFT);
    const BOTTOM_REVERSE: Self = Self::new(Point::BOTTOM_RIGHT, Point::BOTTOM_LEFT);
    const LEFT_REVERSE: Self = Self::new(Point::BOTTOM_LEFT, Point::TOP_LEFT);
    const RIGHT_REVERSE: Self = Self::new(Point::BOTTOM_RIGHT, Point::TOP_RIGHT);

    const fn new(start: Point, end: Point) -> Self {
        Self { start, end }
    }

    const fn with_orientation(self, orientation: Orientation) -> Self {
        Self::new(orientation.apply_on(self.start), orientation.apply_on(self.end))
    }

    const fn opposite(self) -> Self {
        Self::new(Orientation::ROT180.apply_on(self.end), Orientation::ROT180.apply_on(self.start))
    }

    const fn middle(self) -> Point {
        Point::new((self.start.x + self.end.x) / 2, (self.start.y + self.end.y) / 2)
    }

    const fn intersection(self, other: Self) -> Point {
        Point::new((self.start.x + self.end.x + other.start.x + other.end.x) / 2, (self.start.y + self.end.y + other.start.y + other.end.y) / 2)
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Orientation {
    m00: i8,
    m10: i8,
    m01: i8,
    m11: i8,
}

impl Orientation {
    const IDENTITY: Self = Self::new(1, 0, 0, 1);
    const FLIP_LR: Self = Self::new(-1, 0, 0, 1);
    const FLIP_UD: Self = Self::new(1, 0, 0, -1);
    const ROT180: Self = Self::new(-1, 0, 0, -1);
    const ROT90: Self = Self::new(0, -1, 1, 0);
    const ROT90_INV: Self = Self::new(0, 1, -1, 0);
    const TRANSPOSE: Self = Self::new(0, 1, 1, 0);
    const OPPOSITE_TRANSPOSE: Self = Self::new(0, -1, -1, 0);

    const fn new(m00: i8, m10: i8, m01: i8, m11: i8) -> Self {
        Self { m00, m10, m01, m11 }
    }

    const fn mul_inv(self, other: Self) -> Orientation {
        let det = other.m00 * other.m11 - other.m10 * other.m01;

        Self::new(
            (self.m00 * other.m11 + self.m01 * -other.m10) / det,
            (self.m10 * other.m11 + self.m11 * -other.m10) / det,
            (self.m00 * -other.m01 + self.m01 * other.m00) / det,
            (self.m10 * -other.m01 + self.m11 * other.m00) / det,
        )
    }

    const fn apply_on(self, rhs: Point) -> Point {
        Point::new(self.m00 * rhs.x + self.m01 * rhs.y, self.m10 * rhs.x + self.m11 * rhs.y)
    }

    const fn from_borders(border1: Border, border2: Border) -> Self {
        let b1 = Self::new(border1.start.x, border1.start.y, border1.end.x, border1.end.y);
        let b2 = Self::new(border2.start.x, border2.start.y, border2.end.x, border2.end.y);
        b2.mul_inv(b1)
    }
}

struct Grid {
    size: usize,
    tiles: Vec<bool>,
}

impl Grid {
    fn new(size: usize, tiles: Vec<bool>) -> Result<Self> {
        ensure!(size * size == tiles.len(), "unable to construct Grid: width * height != tiles.len()");
        Ok(Self { size, tiles })
    }
}

fn parse_grids(input: &str, size: usize) -> Result<HashMap<u64, Grid>> {
    input
        .split("\n\n")
        .map(|group| {
            let mut group_input = group.lines();

            let id = group_input.next().and_then(|x| x.split_ascii_whitespace().next_back()).and_then(|x| x.split(':').next()).value()?.parse::<u64>()?;

            let tiles = group_input
                .flat_map(|line| line.bytes())
                .flat_map(|x| match x {
                    b'.' => Some(false),
                    b'#' => Some(true),
                    _ => None,
                })
                .collect_vec();

            let grid = Grid::new(size, tiles)?;

            Result::Ok((id, grid))
        })
        .try_collect()
}

fn compute_grid_borders(grids: &HashMap<u64, Grid>, size: usize) -> Vec<(u64, Border, u16)> {
    let mut grid_borders = Vec::new();

    for (&id, grid) in grids {
        let borders = [
            (Border::TOP, grid.tiles[..size].iter().enumerate().map(|(index, &x)| (x as u16) << index).sum::<u16>()),
            (Border::TOP_REVERSE, grid.tiles[..size].iter().rev().enumerate().map(|(index, &x)| (x as u16) << index).sum::<u16>()),
            (Border::BOTTOM, grid.tiles[grid.tiles.len() - size..].iter().enumerate().map(|(index, &x)| (x as u16) << index).sum::<u16>()),
            (Border::BOTTOM_REVERSE, grid.tiles[grid.tiles.len() - size..].iter().rev().enumerate().map(|(index, &x)| (x as u16) << index).sum::<u16>()),
            (Border::LEFT, grid.tiles.iter().step_by(size).enumerate().map(|(index, &x)| (x as u16) << index).sum::<u16>()),
            (Border::LEFT_REVERSE, grid.tiles.iter().step_by(size).rev().enumerate().map(|(index, &x)| (x as u16) << index).sum::<u16>()),
            (Border::RIGHT_REVERSE, grid.tiles.iter().rev().step_by(size).enumerate().map(|(index, &x)| (x as u16) << index).sum::<u16>()),
            (Border::RIGHT, grid.tiles.iter().rev().step_by(size).rev().enumerate().map(|(index, &x)| (x as u16) << index).sum::<u16>()),
        ];

        grid_borders.extend(borders.into_iter().map(|(border, value)| (id, border, value)));
    }

    grid_borders.sort_unstable_by_key(|&(.., value)| value);

    grid_borders
}

fn get_corner_tile_position(square_size: usize, border1: Border, border2: Border) -> Option<(usize, usize)> {
    match border1.intersection(border2) {
        Point::TOP_LEFT => Some((square_size - 1, square_size - 1)),
        Point::TOP_RIGHT => Some((square_size - 1, 0)),
        Point::BOTTOM_LEFT => Some((0, square_size - 1)),
        Point::BOTTOM_RIGHT => Some((0, 0)),
        _ => None,
    }
}

fn fill_image_tile(image_grid: &mut Grid, grid: &Grid, orientation: &Orientation, row: usize, column: usize) -> Result<()> {
    let size = grid.size - 2;

    let iter_image = image_grid.tiles.chunks_exact_mut(image_grid.size).skip(row * size).take(size).flat_map(|x| &mut x[column * size..(column + 1) * size]);

    match *orientation {
        Orientation::IDENTITY => {
            let iter_tile = grid.tiles[grid.size..grid.tiles.len() - grid.size].chunks_exact(grid.size).flat_map(|x| &x[1..x.len() - 1]);
            iter_image.zip(iter_tile).for_each(|(pixel, &value)| *pixel = value);
        }
        Orientation::FLIP_LR => {
            let iter_tile = grid.tiles[grid.size..grid.tiles.len() - grid.size].chunks_exact(grid.size).flat_map(|x| x[1..x.len() - 1].iter().rev());
            iter_image.zip(iter_tile).for_each(|(pixel, &value)| *pixel = value);
        }
        Orientation::FLIP_UD => {
            let iter_tile = grid.tiles[grid.size..grid.tiles.len() - grid.size].chunks_exact(grid.size).rev().flat_map(|x| &x[1..x.len() - 1]);
            iter_image.zip(iter_tile).for_each(|(pixel, &value)| *pixel = value);
        }
        Orientation::ROT180 => {
            let iter_tile = grid.tiles[grid.size..grid.tiles.len() - grid.size].chunks_exact(grid.size).flat_map(|x| &x[1..x.len() - 1]).rev();
            iter_image.zip(iter_tile).for_each(|(pixel, &value)| *pixel = value);
        }
        Orientation::ROT90 => {
            let iter_tile = (1..grid.size - 1).rev().flat_map(|column| grid.tiles[grid.size + column..].iter().step_by(grid.size).take(grid.size - 2));
            iter_image.zip(iter_tile).for_each(|(pixel, &value)| *pixel = value);
        }
        Orientation::ROT90_INV => {
            let iter_tile = (1..grid.size - 1).flat_map(|column| grid.tiles[grid.size + column..].iter().step_by(grid.size).take(grid.size - 2).rev());
            iter_image.zip(iter_tile).for_each(|(pixel, &value)| *pixel = value);
        }
        Orientation::TRANSPOSE => {
            let iter_tile = (1..grid.size - 1).flat_map(|column| grid.tiles[grid.size + column..].iter().step_by(grid.size).take(grid.size - 2));
            iter_image.zip(iter_tile).for_each(|(pixel, &value)| *pixel = value);
        }
        Orientation::OPPOSITE_TRANSPOSE => {
            let iter_tile = (1..grid.size - 1).flat_map(|column| grid.tiles[grid.size + column..].iter().step_by(grid.size).take(grid.size - 2)).rev();
            iter_image.zip(iter_tile).for_each(|(pixel, &value)| *pixel = value);
        }
        _ => bail!("invalid orientation"),
    }

    Ok(())
}

fn find_monsters(image_grid: &Grid) -> usize {
    image_grid
        .tiles
        .chunks_exact(image_grid.size)
        .tuple_windows()
        .flat_map(|(row_0, row_1, row_2)| {
            izip!(row_0.windows(MONSTER_WIDTH), row_1.windows(MONSTER_WIDTH), row_2.windows(MONSTER_WIDTH)).filter(|(x0, x1, x2)| {
                let check0 = x0.iter().zip(MONSTER[0]).all(|(&x, m)| x & m == m);
                let check1 = x1.iter().zip(MONSTER[1]).all(|(&x, m)| x & m == m);
                let check2 = x2.iter().zip(MONSTER[2]).all(|(&x, m)| x & m == m);
                check0 && check1 && check2
            })
        })
        .count()
}

fn transpose(image: &mut [bool], size: usize) {
    for i in 0..size {
        let offset_i = size * i;
        for j in i + 1..size {
            let offset_j = size * j;
            image.swap(i + offset_j, j + offset_i);
        }
    }
}

fn flip(image: &mut [bool], size: usize) {
    for row in image.chunks_exact_mut(size) {
        let mut iter = row.iter_mut();
        while let (Some(first), Some(last)) = (iter.next(), iter.next_back()) {
            std::mem::swap(first, last);
        }
    }
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let width = input.lines().nth(1).value()?.len();
    let height = input.split("\n\n").flat_map(|group| group.lines().count().checked_sub(1)).next().value()?;
    let size = width;
    ensure!(width == height, "tiles must have the same width and height");

    let mut grids = parse_grids(&input, size)?;

    let square_size = (grids.len() as f64).sqrt() as usize;
    ensure!(square_size * square_size == grids.len(), "invalid number of tiles");

    let grid_borders = compute_grid_borders(&grids, size);

    let mut graph = HashMap::<_, SmallVec<[_; 4]>>::new();
    for x in grid_borders.windows(2) {
        let [&(id0, border0, value0), &(id1, border1, value1)] = [&x[0], &x[1]];
        if value0 == value1 {
            graph.entry(id0).or_default().push((id1, border0, border1));
            graph.entry(id1).or_default().push((id0, border1, border0));
        }
    }

    for dependencies in graph.values_mut() {
        dependencies.sort_unstable_by_key(|&(id, ..)| (id));
        dependencies.dedup_by_key(|&mut (id, ..)| id);
    }

    let mut corner_ids_product = 1;
    let mut image_tiles = Vec::with_capacity(square_size * square_size);

    let (corner_id, border1, border2) = graph
        .iter()
        .find_map(|(&id, dependencies)| match dependencies.as_slice() {
            &[(_, border1, _), (_, border2, _)] => Some((id, border1, border2)),
            _ => None,
        })
        .value()?;

    let mut queue = VecDeque::from([(corner_id, Orientation::IDENTITY, get_corner_tile_position(square_size, border1, border2).value()?)]);
    let mut visited = HashSet::from([corner_id]);

    while let Some((id, orientation, (row, column))) = queue.pop_front() {
        let dependencies = &graph[&id];
        if dependencies.len() == 2 {
            corner_ids_product *= id;
        }

        for &(new_id, self_border, border_after) in dependencies {
            if visited.insert(new_id) {
                let new_orientation = Orientation::from_borders(border_after, self_border.opposite().with_orientation(orientation));

                let (new_row, new_column) = match self_border.with_orientation(orientation).middle() {
                    Point::TOP => (row - 1, column),
                    Point::BOTTOM => (row + 1, column),
                    Point::LEFT => (row, column - 1),
                    Point::RIGHT => (row, column + 1),
                    _ => bail!("invalid middle point"),
                };

                queue.push_back((new_id, new_orientation, (new_row, new_column)));
            }
        }

        image_tiles.push(((row, column), id, orientation));
    }

    let result1 = corner_ids_product;

    let image_size = square_size * (size - 2);
    let mut image_grid = Grid::new(image_size, vec![false; image_size * image_size])?;

    for ((row, column), id, orientation) in image_tiles {
        let grid = grids.remove(&id).value()?;
        fill_image_tile(&mut image_grid, &grid, &orientation, row, column)?;
    }

    let monster_count = once(find_monsters(&image_grid))
        .chain([transpose, flip, transpose, flip, transpose, flip, transpose].iter().map(|transform| {
            transform(&mut image_grid.tiles, image_size);
            find_monsters(&image_grid)
        }))
        .find(|&x| x != 0)
        .value()?;

    let result2 = image_grid.tiles.iter().filter(|&&x| x).count() - monster_count * MONSTER_PIXEL_COUNT;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
