use aoc::*;

use eyre::{bail, ensure};
use itertools::{izip, Itertools};
use num_complex::Complex;

use std::collections::{HashMap, VecDeque};
use std::fmt::Write;
use std::iter::repeat;

struct Intcode {
    program: HashMap<usize, i64>,
    ip: usize,
    relative_base: i64,
    inputs: VecDeque<i64>,
}

impl Intcode {
    fn new(program: HashMap<usize, i64>, inputs: VecDeque<i64>) -> Self {
        Self { program, ip: 0, relative_base: 0, inputs }
    }

    fn get_input(&mut self, arg_position: usize, instruction: i64) -> Result<i64> {
        let arg = *self.program.entry(self.ip + arg_position).or_default();

        match instruction / 10i64.pow(1 + arg_position as u32) % 10 {
            0 => Ok(*self.program.entry(usize::try_from(arg)?).or_default()),
            1 => Ok(arg),
            2 => Ok(*self.program.entry(usize::try_from(self.relative_base + arg)?).or_default()),
            other => bail!("unknown parameter mode: {other}"),
        }
    }

    fn get_register(&mut self, arg_position: usize, instruction: i64) -> Result<&mut i64> {
        let arg = *self.program.entry(self.ip + arg_position).or_default();

        match instruction / 10i64.pow(1 + arg_position as u32) % 10 {
            0 => Ok(self.program.entry(usize::try_from(arg)?).or_default()),
            2 => Ok(self.program.entry(usize::try_from(self.relative_base + arg)?).or_default()),
            other => bail!("invalid parameter mode: {other}"),
        }
    }

    fn run(&mut self) -> Result<Vec<i64>> {
        let mut outputs = Vec::new();

        loop {
            let instruction = *self.program.entry(self.ip).or_default();
            match instruction % 100 {
                1 => {
                    let arg1 = self.get_input(1, instruction)?;
                    let arg2 = self.get_input(2, instruction)?;
                    let arg3 = self.get_register(3, instruction)?;
                    *arg3 = arg1 + arg2;
                    self.ip += 4;
                }
                2 => {
                    let arg1 = self.get_input(1, instruction)?;
                    let arg2 = self.get_input(2, instruction)?;
                    let arg3 = self.get_register(3, instruction)?;
                    *arg3 = arg1 * arg2;
                    self.ip += 4;
                }
                3 => {
                    let input = self.inputs.pop_front().value()?;
                    let arg1 = self.get_register(1, instruction)?;
                    *arg1 = input;
                    self.ip += 2;
                }
                4 => {
                    let arg1 = self.get_input(1, instruction)?;
                    outputs.push(arg1);
                    self.ip += 2;
                }
                5 => {
                    let arg1 = self.get_input(1, instruction)?;
                    let arg2 = self.get_input(2, instruction)?;
                    if arg1 != 0 {
                        self.ip = arg2.try_into()?;
                    } else {
                        self.ip += 3;
                    }
                }
                6 => {
                    let arg1 = self.get_input(1, instruction)?;
                    let arg2 = self.get_input(2, instruction)?;
                    if arg1 == 0 {
                        self.ip = arg2.try_into()?;
                    } else {
                        self.ip += 3;
                    }
                }
                7 => {
                    let arg1 = self.get_input(1, instruction)?;
                    let arg2 = self.get_input(2, instruction)?;
                    let arg3 = self.get_register(3, instruction)?;
                    *arg3 = (arg1 < arg2).into();
                    self.ip += 4;
                }
                8 => {
                    let arg1 = self.get_input(1, instruction)?;
                    let arg2 = self.get_input(2, instruction)?;
                    let arg3 = self.get_register(3, instruction)?;
                    *arg3 = (arg1 == arg2).into();
                    self.ip += 4;
                }
                9 => {
                    let arg1 = self.get_input(1, instruction)?;
                    self.relative_base += arg1;
                    self.ip += 2;
                }
                99 => break,
                other => bail!("unknown opcode: {other}"),
            }
        }

        Ok(outputs)
    }
}

const LEFT_TURN: Complex<i64> = Complex::new(0, -1);
const RIGHT_TURN: Complex<i64> = Complex::new(0, 1);

struct Grid {
    width: usize,
    tiles: Vec<bool>,
}

impl Grid {
    fn new(width: usize, height: usize, tiles: Vec<bool>) -> Result<Self> {
        ensure!(width * height == tiles.len(), "unable to construct Grid: width * height != tiles.len()");
        Ok(Self { width, tiles })
    }

    fn get_index(&self, position: Complex<i64>) -> usize {
        position.im as usize * self.width + position.re as usize
    }
}

fn compute_grid(mut intcode: Intcode) -> Result<(Grid, Complex<i64>, Complex<i64>)> {
    let outputs = intcode.run()?;

    let width = outputs.split(|&x| x == 10).next().value()?.len() + 2;
    let height = outputs.split(|&x| x == 10).filter(|row| !row.is_empty()).count() + 2;

    let mut current_position = None;
    let mut current_direction = None;

    let mut tiles = Vec::with_capacity(width * height);
    tiles.extend(repeat(false).take(width));

    for (i_row, row) in outputs.split(|&x| x == 10).filter(|row| !row.is_empty()).enumerate() {
        let i_row = i_row as i64;

        tiles.push(false);

        for (i_col, &x) in row.iter().enumerate() {
            let i_col = i_col as i64;

            match x as u8 {
                b'#' => tiles.push(true),
                b'^' => {
                    current_position = Some(Complex::new(i_col + 1, i_row + 1));
                    current_direction = Some(Complex::new(0, -1));
                    tiles.push(true);
                }
                b'v' => {
                    current_position = Some(Complex::new(i_col + 1, i_row + 1));
                    current_direction = Some(Complex::new(0, 1));
                    tiles.push(true);
                }
                b'<' => {
                    current_position = Some(Complex::new(i_col + 1, i_row + 1));
                    current_direction = Some(Complex::new(-1, 0));
                    tiles.push(true);
                }
                b'>' => {
                    current_position = Some(Complex::new(i_col + 1, i_row + 1));
                    current_direction = Some(Complex::new(1, 0));
                    tiles.push(true);
                }
                _ => tiles.push(false),
            }
        }

        tiles.push(false);
    }

    tiles.extend(repeat(false).take(width));

    Ok((Grid::new(width, height, tiles)?, current_position.value()?, current_direction.value()?))
}

fn compute_alignment(grid: &Grid) -> usize {
    grid.tiles
        .chunks_exact(grid.width)
        .tuple_windows()
        .enumerate()
        .flat_map(|(i_row, (row_0, row_1, row_2))| {
            izip!(row_0.windows(3), row_1.windows(3), row_2.windows(3))
                .enumerate()
                .filter(|&(_, (x0, x1, x2))| x1[1] && x0[1] as u8 + x1[0] as u8 + x1[2] as u8 + x2[1] as u8 >= 3)
                .map(move |(i_col, _)| i_row * i_col)
        })
        .sum()
}

fn compute_path(grid: &Grid, mut current_position: Complex<i64>, mut current_direction: Complex<i64>) -> Result<String> {
    let mut path = String::new();

    loop {
        let mut forward_steps = 0;
        while grid.tiles[grid.get_index(current_position + current_direction)] {
            current_position += current_direction;
            forward_steps += 1;
        }

        if forward_steps != 0 {
            write!(path, "{forward_steps},")?;
        }

        if grid.tiles[grid.get_index(current_position + current_direction * LEFT_TURN)] {
            current_direction *= LEFT_TURN;
            path.push_str("L,");
        } else if grid.tiles[grid.get_index(current_position + current_direction * RIGHT_TURN)] {
            current_direction *= RIGHT_TURN;
            path.push_str("R,");
        } else {
            break;
        }
    }

    Ok(path)
}

fn check_pattern(pattern: &str) -> Option<&str> {
    matches!(pattern.as_bytes(), &[first, .., b','] if first != b',').then(|| pattern)
}

fn compute_inputs(path: &str) -> Result<VecDeque<i64>> {
    let mut buffer = Vec::new();

    let patterns = (2..=21.min(path.len()))
        .find_map(|first_index| {
            let first_pattern = check_pattern(&path[..first_index])?;

            let sub_path = match path.split(first_pattern).filter(|&s| !s.is_empty()).min_by_key(|s| s.len()) {
                None => return Some([first_pattern, "", ""]),
                Some(x) if x.len() < 2 => return None,
                Some(x) => x,
            };

            (sub_path.len().saturating_sub(21)..=sub_path.len() - 2).rev().find_map(|second_index| {
                let second_pattern = check_pattern(&sub_path[second_index..])?;

                buffer.clear();
                buffer.extend(path.split(first_pattern).flat_map(|s| s.split(second_pattern)).filter(|&s| !s.is_empty()));

                let third_pattern = match buffer.iter().copied().min_by_key(|s| s.len()) {
                    None => return Some([first_pattern, second_pattern, ""]),
                    Some(x) => check_pattern(x)?,
                };

                if buffer.iter().all(|&s| s.len() % third_pattern.len() == 0) && buffer.iter().flat_map(|&s| s.split(third_pattern)).all(|s| s.is_empty()) {
                    Some([first_pattern, second_pattern, third_pattern])
                } else {
                    None
                }
            })
        })
        .value()?;

    let iter_a = path.match_indices(patterns[0]).filter(|(_, s)| !s.is_empty()).map(|(index, _)| (index, *b"A,"));
    let iter_b = path.match_indices(patterns[1]).filter(|(_, s)| !s.is_empty()).map(|(index, _)| (index, *b"B,"));
    let iter_c = path.match_indices(patterns[2]).filter(|(_, s)| !s.is_empty()).map(|(index, _)| (index, *b"C,"));

    let mut sorted_matches = iter_a.chain(iter_b).chain(iter_c).sorted_unstable_by_key(|&(index, _)| index).collect_vec();
    *sorted_matches.last_mut().map(|(_, [_, x])| x).value()? = b'\n';

    let main_routine = sorted_matches.into_iter().flat_map(|(_, s)| s);
    let sub_functions = patterns.into_iter().flat_map(|pattern| pattern.bytes().rev().skip(1).rev().chain([b'\n']));

    Ok(main_routine.chain(sub_functions).chain(*b"n\n").map_into().collect())
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);
    let input = input.trim();

    let mut program: HashMap<usize, i64> = input.split(',').enumerate().map(|(pos, val)| Result::Ok((pos, val.parse()?))).try_collect()?;

    let (grid, current_position, current_direction) = compute_grid(Intcode::new(program.clone(), VecDeque::new()))?;

    let result1 = compute_alignment(&grid);

    let path = compute_path(&grid, current_position, current_direction)?;
    let inputs = compute_inputs(&path)?;

    program.insert(0, 2);
    let result2 = *Intcode::new(program, inputs).run()?.last().value()?;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
