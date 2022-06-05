use aoc::*;

use eyre::eyre;
use itertools::Itertools;
use num_complex::Complex;

use std::collections::{HashMap, HashSet};

fn get_input(program: &mut HashMap<usize, i64>, ip: usize, arg_position: usize, relative_base: i64, instruction: i64) -> Result<i64> {
    let arg = *program.entry(ip + arg_position).or_default();

    match instruction / 10i64.pow(1 + arg_position as u32) % 10 {
        0 => Ok(*program.entry(usize::try_from(arg)?).or_default()),
        1 => Ok(arg),
        2 => Ok(*program.entry(usize::try_from(relative_base + arg)?).or_default()),
        other => Err(eyre!("unknown parameter mode: {other}")),
    }
}

fn get_register(program: &mut HashMap<usize, i64>, ip: usize, arg_position: usize, relative_base: i64, instruction: i64) -> Result<&mut i64> {
    let arg = *program.entry(ip + arg_position).or_default();

    match instruction / 10i64.pow(1 + arg_position as u32) % 10 {
        0 => Ok(program.entry(usize::try_from(arg)?).or_default()),
        2 => Ok(program.entry(usize::try_from(relative_base + arg)?).or_default()),
        other => Err(eyre!("invalid parameter mode: {other}")),
    }
}

enum State {
    NeedInput,
    HasOutput(i64),
    Finished,
}

struct Intcode {
    program: HashMap<usize, i64>,
    ip: usize,
    relative_base: i64,
    input: Option<i64>,
}

impl Intcode {
    fn new(program: HashMap<usize, i64>, input: Option<i64>) -> Self {
        Self { program, ip: 0, relative_base: 0, input }
    }

    fn run(&mut self) -> Result<State> {
        let Intcode { program, ip, relative_base, input } = self;

        loop {
            let instruction = *program.entry(*ip).or_default();
            match instruction % 100 {
                1 => {
                    let arg1 = get_input(program, *ip, 1, *relative_base, instruction)?;
                    let arg2 = get_input(program, *ip, 2, *relative_base, instruction)?;
                    let arg3 = get_register(program, *ip, 3, *relative_base, instruction)?;
                    *arg3 = arg1 + arg2;
                    *ip += 4;
                }
                2 => {
                    let arg1 = get_input(program, *ip, 1, *relative_base, instruction)?;
                    let arg2 = get_input(program, *ip, 2, *relative_base, instruction)?;
                    let arg3 = get_register(program, *ip, 3, *relative_base, instruction)?;
                    *arg3 = arg1 * arg2;
                    *ip += 4;
                }
                3 => match input.take() {
                    None => return Ok(State::NeedInput),
                    Some(input) => {
                        let arg1 = get_register(program, *ip, 1, *relative_base, instruction)?;
                        *arg1 = input;
                        *ip += 2;
                    }
                },
                4 => {
                    let arg1 = get_input(program, *ip, 1, *relative_base, instruction)?;
                    *ip += 2;
                    return Ok(State::HasOutput(arg1));
                }
                5 => {
                    let arg1 = get_input(program, *ip, 1, *relative_base, instruction)?;
                    let arg2 = get_input(program, *ip, 2, *relative_base, instruction)?;
                    if arg1 != 0 {
                        *ip = arg2.try_into()?;
                    } else {
                        *ip += 3;
                    }
                }
                6 => {
                    let arg1 = get_input(program, *ip, 1, *relative_base, instruction)?;
                    let arg2 = get_input(program, *ip, 2, *relative_base, instruction)?;
                    if arg1 == 0 {
                        *ip = arg2.try_into()?;
                    } else {
                        *ip += 3;
                    }
                }
                7 => {
                    let arg1 = get_input(program, *ip, 1, *relative_base, instruction)?;
                    let arg2 = get_input(program, *ip, 2, *relative_base, instruction)?;
                    let arg3 = get_register(program, *ip, 3, *relative_base, instruction)?;
                    *arg3 = (arg1 < arg2).into();
                    *ip += 4;
                }
                8 => {
                    let arg1 = get_input(program, *ip, 1, *relative_base, instruction)?;
                    let arg2 = get_input(program, *ip, 2, *relative_base, instruction)?;
                    let arg3 = get_register(program, *ip, 3, *relative_base, instruction)?;
                    *arg3 = (arg1 == arg2).into();
                    *ip += 4;
                }
                9 => {
                    let arg1 = get_input(program, *ip, 1, *relative_base, instruction)?;
                    *relative_base += arg1;
                    *ip += 2;
                }
                99 => return Ok(State::Finished),
                other => return Err(eyre!("unknown opcode: {other}")),
            }
        }
    }
}

const NORTH: Complex<i64> = Complex::new(0, 1);
const SOUTH: Complex<i64> = Complex::new(0, -1);
const WEST: Complex<i64> = Complex::new(-1, 0);
const EAST: Complex<i64> = Complex::new(1, 0);
const DIRECTIONS: [Complex<i64>; 4] = [NORTH, SOUTH, WEST, EAST];
const REVERSE_INPUTS: [u8; 4] = [2, 1, 4, 3];

#[derive(Copy, Clone, Eq, PartialEq)]
enum Tile {
    Wall,
    Empty,
    Goal,
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);
    let input = input.trim();

    let program: HashMap<usize, i64> = input.split(',').enumerate().map(|(pos, val)| Result::Ok((pos, val.parse()?))).try_collect()?;
    let mut intcode = Intcode::new(program, None);

    let mut current_position = Complex::new(0, 0);
    let mut current_direction = Complex::new(0, 0);
    let mut unknown_tiles = HashMap::from([(NORTH, vec![1]), (SOUTH, vec![2]), (WEST, vec![3]), (EAST, vec![4])]);
    let mut unknown_path = Vec::new();
    let mut remaining_inputs = Vec::new();

    let mut grid = HashMap::from([(current_position, Tile::Empty)]);
    let mut goal_position = None;

    loop {
        match intcode.run()? {
            State::Finished => break,
            State::NeedInput => {
                if let Some(input) = remaining_inputs.pop() {
                    intcode.input = Some(input as i64);
                    current_direction = DIRECTIONS[(input - 1) as usize];
                    continue;
                }

                let current_path = unknown_path;

                for (new_input, direction) in (1..=4).zip(DIRECTIONS) {
                    let position = current_position + direction;
                    if grid.get(&position).is_none() && !unknown_tiles.contains_key(&position) {
                        let mut new_path = current_path.clone();
                        new_path.push(new_input);
                        unknown_tiles.insert(position, new_path);
                    }
                }

                match unknown_tiles.keys().next().copied() {
                    None => break,
                    Some(position) => {
                        unknown_path = unknown_tiles.remove(&position).value()?;

                        let min_len = || unknown_path.len().min(current_path.len());
                        let common_path_size = current_path.iter().zip(&unknown_path).position(|(x, y)| x != y).unwrap_or_else(min_len);

                        let path_iter = unknown_path.iter().copied().rev().take(unknown_path.len() - common_path_size);
                        let current_path_iter = current_path.iter().map(|&x| REVERSE_INPUTS[(x - 1) as usize]).skip(common_path_size);

                        remaining_inputs.extend(path_iter.chain(current_path_iter));
                    }
                };
            }
            State::HasOutput(output) => {
                match output {
                    0 => {
                        unknown_path.pop();
                        grid.insert(current_position + current_direction, Tile::Wall);
                    }
                    1 => {
                        current_position += current_direction;
                        grid.insert(current_position, Tile::Empty);
                    }
                    2 => {
                        current_position += current_direction;
                        goal_position = Some(current_position);
                        grid.insert(current_position, Tile::Goal);
                    }
                    other => return Err(eyre!("unknown tile: {other}")),
                };
            }
        }
    }

    let goal_position = goal_position.value()?;

    let mut distance = None;
    let mut steps = 0;
    let mut visited = HashSet::new();
    let mut queue = vec![goal_position];
    let mut new_queue = Vec::new();

    loop {
        new_queue.clear();

        while let Some(position) = queue.pop() {
            visited.insert(position);

            let new_positions = DIRECTIONS.into_iter().map(|direction| position + direction);
            new_queue.extend(new_positions.filter(|new_position| !visited.contains(new_position) && grid[new_position] != Tile::Wall));

            if position == Complex::new(0, 0) {
                distance = Some(steps);
            }
        }

        if new_queue.is_empty() {
            break;
        }

        steps += 1;
        std::mem::swap(&mut queue, &mut new_queue);
    }

    let result1 = distance.value()?;
    let result2 = steps;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
