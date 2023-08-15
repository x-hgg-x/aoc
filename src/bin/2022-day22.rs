use aoc::*;

use eyre::{bail, ensure};
use itertools::Itertools;
use num_complex::Complex;

use std::collections::{HashMap, HashSet};
use std::iter;
use std::ops::ControlFlow;

const LEFT: Complex<i64> = Complex::new(-1, 0);
const RIGHT: Complex<i64> = Complex::new(1, 0);
const DOWN: Complex<i64> = Complex::new(0, -1);
const UP: Complex<i64> = Complex::new(0, 1);

const ROTATION_LEFT: &Mat3x3 = &[[0, 0, 1], [0, 1, 0], [-1, 0, 0]];
const ROTATION_RIGHT: &Mat3x3 = &[[0, 0, -1], [0, 1, 0], [1, 0, 0]];
const ROTATION_DOWN: &Mat3x3 = &[[1, 0, 0], [0, 0, 1], [0, -1, 0]];
const ROTATION_UP: &Mat3x3 = &[[1, 0, 0], [0, 0, -1], [0, 1, 0]];

type Vec3 = [i64; 3];
type Mat3x3 = [Vec3; 3];

trait Vector {
    fn sub(&self, rhs: &Self) -> Self;
    fn dot(&self, rhs: &Self) -> i64;
    fn apply(&self, matrix: &Mat3x3) -> Self;
}

impl Vector for Vec3 {
    fn sub(&self, rhs: &Self) -> Self {
        [self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2]]
    }

    fn dot(&self, rhs: &Self) -> i64 {
        self[0] * rhs[0] + self[1] * rhs[1] + self[2] * rhs[2]
    }

    fn apply(&self, matrix: &Mat3x3) -> Self {
        [self.dot(&matrix[0]), self.dot(&matrix[1]), self.dot(&matrix[2])]
    }
}

trait Matrix {
    fn identity() -> Self;
    fn matmul(&self, rhs: &Self) -> Self;
    fn transpose(&self) -> Self;
    fn z_axis(&self) -> Vec3;
}

impl Matrix for Mat3x3 {
    fn identity() -> Self {
        [[1, 0, 0], [0, 1, 0], [0, 0, 1]]
    }

    fn matmul(&self, rhs: &Mat3x3) -> Mat3x3 {
        [
            [
                self[0][0] * rhs[0][0] + self[0][1] * rhs[1][0] + self[0][2] * rhs[2][0],
                self[0][0] * rhs[0][1] + self[0][1] * rhs[1][1] + self[0][2] * rhs[2][1],
                self[0][0] * rhs[0][2] + self[0][1] * rhs[1][2] + self[0][2] * rhs[2][2],
            ],
            [
                self[1][0] * rhs[0][0] + self[1][1] * rhs[1][0] + self[1][2] * rhs[2][0],
                self[1][0] * rhs[0][1] + self[1][1] * rhs[1][1] + self[1][2] * rhs[2][1],
                self[1][0] * rhs[0][2] + self[1][1] * rhs[1][2] + self[1][2] * rhs[2][2],
            ],
            [
                self[2][0] * rhs[0][0] + self[2][1] * rhs[1][0] + self[2][2] * rhs[2][0],
                self[2][0] * rhs[0][1] + self[2][1] * rhs[1][1] + self[2][2] * rhs[2][1],
                self[2][0] * rhs[0][2] + self[2][1] * rhs[1][2] + self[2][2] * rhs[2][2],
            ],
        ]
    }

    fn transpose(&self) -> Self {
        [[self[0][0], self[1][0], self[2][0]], [self[0][1], self[1][1], self[2][1]], [self[0][2], self[1][2], self[2][2]]]
    }

    fn z_axis(&self) -> Vec3 {
        [self[0][2], self[1][2], self[2][2]]
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Tile {
    Empty,
    Wall,
}

impl Tile {
    fn from_ascii(x: u8) -> Option<Self> {
        match x {
            b'.' => Some(Tile::Empty),
            b'#' => Some(Tile::Wall),
            _ => None,
        }
    }
}

#[derive(Clone)]
struct Grid {
    size: usize,
    orientation: Mat3x3,
    tiles: Vec<Tile>,
}

impl Grid {
    fn contains(&self, coord: Complex<i64>) -> bool {
        0 <= coord.re && coord.re < self.size as i64 && 0 >= coord.im && coord.im > -(self.size as i64)
    }

    fn get_index(&self, coord: Complex<i64>) -> Result<usize> {
        ensure!(self.contains(coord), "coord must be in bounds");
        Ok((-coord.im) as usize * self.size + (coord.re as usize))
    }

    fn tile(&self, coord: Complex<i64>) -> Result<Tile> {
        let tile_index = self.get_index(coord)?;
        Ok(self.tiles[tile_index])
    }
}

struct BlockGrid {
    width: usize,
    height: usize,
    grids: Vec<Option<Grid>>,
}

impl BlockGrid {
    fn get_index(&self, row: usize, column: usize) -> usize {
        row * self.width + column
    }

    fn get_position(&self, index: usize) -> (usize, usize) {
        let row = index / self.width;
        let column = index % self.width;
        (row, column)
    }

    fn compute_orientations(&mut self) -> Result<HashMap<Vec3, usize>> {
        let initial_column = self.grids.iter().position(|block| block.is_some()).value()?;
        let initial_position = (0usize, initial_column);
        let initial_orientation = Mat3x3::identity();

        let mut visited = HashSet::from([initial_position]);
        let mut queue = vec![(initial_position, initial_orientation)];

        while let Some(((row, column), orientation)) = queue.pop() {
            queue.extend(
                [
                    row.checked_sub(1).map(|new_row| ((new_row, column), ROTATION_UP)),
                    column.checked_sub(1).map(|new_column| ((row, new_column), ROTATION_LEFT)),
                    (row < self.height - 1).then_some(((row + 1, column), ROTATION_DOWN)),
                    (column < self.width - 1).then_some(((row, column + 1), ROTATION_RIGHT)),
                ]
                .into_iter()
                .flatten()
                .filter(|&(position, _)| visited.insert(position))
                .flat_map(|((new_row, new_column), rotation)| {
                    let new_index = self.get_index(new_row, new_column);
                    self.grids[new_index].as_mut().map(|grid| {
                        grid.orientation = orientation.matmul(rotation);
                        ((new_row, new_column), grid.orientation)
                    })
                }),
            );
        }

        Ok(self.grids.iter().enumerate().flat_map(|(index, grid)| grid.as_ref().map(|grid| (grid.orientation.z_axis(), index))).collect())
    }
}

enum Instruction {
    TurnLeft,
    TurnRight,
    Forward(u64),
}

struct State<'a> {
    block_index: usize,
    grid: &'a Grid,
    coord: Complex<i64>,
    direction: Complex<i64>,
}

impl<'a> State<'a> {
    fn new(blocks: &'a BlockGrid) -> Result<Self> {
        let block_index = blocks.grids.iter().position(|block| block.is_some()).value()?;
        let grid = blocks.grids[block_index].as_ref().value()?;
        let coord = grid.tiles.iter().position(|&tile| tile == Tile::Empty).map(|x| Complex::new(x as i64, 0)).value()?;
        let direction = Complex::new(1, 0);

        Ok(Self { block_index, grid, coord, direction })
    }

    fn step(&mut self, instruction: &Instruction, with_3d: bool, blocks: &'a BlockGrid, faces: &HashMap<Vec3, usize>) -> Result<()> {
        match *instruction {
            Instruction::TurnLeft => self.direction *= Complex::new(0, 1),
            Instruction::TurnRight => self.direction *= Complex::new(0, -1),
            Instruction::Forward(n) => {
                for _ in 0..n {
                    let new_coord = self.coord + self.direction;
                    if self.grid.contains(new_coord) {
                        if self.grid.tile(new_coord)? == Tile::Wall {
                            break;
                        }
                        self.coord = new_coord;
                    } else {
                        let grid_size = self.grid.size as i64;
                        let wrapped_coord = Complex::new(new_coord.re.rem_euclid(grid_size), -(-new_coord.im).rem_euclid(grid_size));

                        let control_flow = if with_3d { self.wrap_3d(blocks, faces, wrapped_coord)? } else { self.wrap_2d(blocks, wrapped_coord)? };
                        if control_flow.is_break() {
                            break;
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn wrap_2d(&mut self, blocks: &'a BlockGrid, wrapped_coord: Complex<i64>) -> Result<ControlFlow<()>> {
        let (block_row, block_column) = blocks.get_position(self.block_index);

        let new_block_index = match self.direction {
            RIGHT => ((0..blocks.width).cycle().skip(block_column + 1))
                .map(|column| blocks.get_index(block_row, column))
                .find(|&index| blocks.grids[index].is_some())
                .value()?,
            LEFT => ((0..blocks.width).rev().cycle().skip(blocks.width - block_column))
                .map(|column| blocks.get_index(block_row, column))
                .find(|&index| blocks.grids[index].is_some())
                .value()?,
            UP => ((0..blocks.height).rev().cycle().skip(blocks.height - block_row))
                .map(|row| blocks.get_index(row, block_column))
                .find(|&index| blocks.grids[index].is_some())
                .value()?,
            DOWN => ((0..blocks.height).cycle().skip(block_row + 1))
                .map(|row| blocks.get_index(row, block_column))
                .find(|&index| blocks.grids[index].is_some())
                .value()?,
            direction => bail!("incorrect direction: {direction}"),
        };

        let new_grid = blocks.grids[new_block_index].as_ref().value()?;
        if new_grid.tile(wrapped_coord)? == Tile::Wall {
            return Ok(ControlFlow::Break(()));
        }

        *self = Self { block_index: new_block_index, grid: new_grid, coord: wrapped_coord, direction: self.direction };
        Ok(ControlFlow::Continue(()))
    }

    fn wrap_3d(&mut self, blocks: &'a BlockGrid, faces: &HashMap<Vec3, usize>, wrapped_coord: Complex<i64>) -> Result<ControlFlow<()>> {
        let rotation = match self.direction {
            LEFT => ROTATION_LEFT,
            RIGHT => ROTATION_RIGHT,
            DOWN => ROTATION_DOWN,
            UP => ROTATION_UP,
            direction => bail!("incorrect direction: {direction}"),
        };

        let rotated = self.grid.orientation.matmul(rotation);
        let new_block_index = faces[&rotated.z_axis()];
        let new_grid = blocks.grids[new_block_index].as_ref().value()?;

        let coord_rotation = new_grid.orientation.transpose().matmul(&rotated);
        ensure!(coord_rotation[2] == [0, 0, 1] && coord_rotation.z_axis() == [0, 0, 1], "should be a 2D rotation");

        let new_coord_vec = [wrapped_coord.re, wrapped_coord.im, 0].apply(&coord_rotation);
        let new_origin_corner_vec = [-1, 1, 0].apply(&coord_rotation).sub(&[-1, 1, 0]);

        let side = self.grid.size as i64 - 1;
        let new_coord = Complex::new(new_origin_corner_vec[0].signum() * side + new_coord_vec[0], new_origin_corner_vec[1].signum() * side + new_coord_vec[1]);

        if new_grid.tile(new_coord)? == Tile::Wall {
            return Ok(ControlFlow::Break(()));
        }

        let new_direction_vec = [self.direction.re, self.direction.im, 0].apply(&coord_rotation);
        let new_direction = Complex::new(new_direction_vec[0], new_direction_vec[1]);

        *self = Self { block_index: new_block_index, grid: new_grid, coord: new_coord, direction: new_direction };
        Ok(ControlFlow::Continue(()))
    }
}

fn compute_password(blocks: &BlockGrid, faces: &HashMap<Vec3, usize>, instructions: &[Instruction], with_3d: bool) -> Result<i64> {
    let mut state = State::new(blocks)?;

    for instruction in instructions {
        state.step(instruction, with_3d, blocks, faces)?;
    }

    let (block_row, block_column) = blocks.get_position(state.block_index);

    let row = (block_row * state.grid.size) as i64 - state.coord.im + 1;
    let column = (block_column * state.grid.size) as i64 + state.coord.re + 1;
    let direction_score = direction_score(&state.direction)?;

    Ok(1000 * row + 4 * column + direction_score)
}

fn direction_score(direction: &Complex<i64>) -> Result<i64> {
    match *direction {
        RIGHT => Ok(0),
        DOWN => Ok(1),
        LEFT => Ok(2),
        UP => Ok(3),
        _ => bail!("incorrect direction: {direction}"),
    }
}

fn parse_input(input: &str) -> Result<(BlockGrid, HashMap<Vec3, usize>, Vec<Instruction>)> {
    let mut input_iter = input.split("\n\n");

    let map_lines = input_iter.next().value()?.lines().collect_vec();
    let tile_count = map_lines.iter().flat_map(|line| line.bytes()).filter(|x| matches!(x, b'.' | b'#')).count();
    let block_size = ((tile_count / 6) as f64).sqrt() as usize;

    let map_width = map_lines.iter().map(|line| line.len()).max().value()?;
    let map_height = map_lines.len();

    let block_grid_width = map_width / block_size;
    let block_grid_height = map_height / block_size;

    let mut grids = vec![None; block_grid_width * block_grid_height];

    for (grid_chunk, line_chunk) in iter::zip(grids.chunks_exact_mut(block_grid_width), map_lines.chunks_exact(block_size)) {
        for line in line_chunk {
            for (grid, byte_chunk) in iter::zip(&mut *grid_chunk, line.as_bytes().chunks_exact(block_size)) {
                if byte_chunk.first() != Some(&b' ') {
                    let grid = grid.get_or_insert_with(|| Grid {
                        size: block_size,
                        orientation: Mat3x3::identity(),
                        tiles: Vec::with_capacity(block_size * block_size),
                    });
                    grid.tiles.extend(byte_chunk.iter().copied().flat_map(Tile::from_ascii));
                }
            }
        }
    }

    let mut blocks = BlockGrid { width: block_grid_width, height: block_grid_height, grids };
    let faces = blocks.compute_orientations()?;

    let instructions = input_iter
        .flat_map(|x| x.lines())
        .next()
        .value()?
        .bytes()
        .map(|x| match x {
            b'L' => Instruction::TurnLeft,
            b'R' => Instruction::TurnRight,
            _ => Instruction::Forward((x - b'0').into()),
        })
        .coalesce(|x1, x2| match (&x1, &x2) {
            (Instruction::Forward(n1), Instruction::Forward(n2)) => Ok(Instruction::Forward(n1 * 10 + n2)),
            _ => Err((x1, x2)),
        })
        .collect();

    Ok((blocks, faces, instructions))
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let (blocks, faces, instructions) = parse_input(&input)?;

    let result1 = compute_password(&blocks, &faces, &instructions, false)?;
    let result2 = compute_password(&blocks, &faces, &instructions, true)?;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
