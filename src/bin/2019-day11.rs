use aoc::*;

use eyre::eyre;
use itertools::Itertools;
use num_complex::Complex;
use smallvec::SmallVec;

use std::collections::HashMap;
use std::iter::{once, repeat};

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

struct Intcode {
    program: HashMap<usize, i64>,
    ip: usize,
    relative_base: i64,
    input: i64,
}

impl Intcode {
    fn new(program: HashMap<usize, i64>, input: i64) -> Self {
        Self { program, ip: 0, relative_base: 0, input }
    }

    fn run(&mut self) -> Result<Option<SmallVec<[i64; 2]>>> {
        let Intcode { program, ip, relative_base, input } = self;

        let mut outputs = SmallVec::new();

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
                3 => {
                    let arg1 = get_register(program, *ip, 1, *relative_base, instruction)?;
                    *arg1 = *input;
                    *ip += 2;
                }
                4 => {
                    let arg1 = get_input(program, *ip, 1, *relative_base, instruction)?;
                    *ip += 2;
                    outputs.push(arg1);
                    if outputs.len() == 2 {
                        return Ok(Some(outputs));
                    }
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
                99 => return Ok(None),
                other => return Err(eyre!("unknown opcode: {other}")),
            }
        }
    }
}

fn draw(mut intcode: Intcode) -> Result<HashMap<Complex<i64>, i64>> {
    let mut current_position = Complex::new(0, 0);
    let mut current_direction = Complex::new(0, 1);
    let mut grid = HashMap::new();

    while let Some(outputs) = intcode.run()?.as_deref() {
        let [new_color, turn] = <[_; 2]>::try_from(outputs)?;
        grid.insert(current_position, new_color);

        match turn {
            0 => current_direction *= Complex::new(0, 1),
            1 => current_direction *= Complex::new(0, -1),
            other => return Err(eyre!("unknown turn: {other}")),
        }

        current_position += current_direction;
        intcode.input = grid.get(&current_position).copied().unwrap_or(0);
    }

    Ok(grid)
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);
    let input = input.trim();

    let program: HashMap<usize, i64> = input.split(',').enumerate().map(|(pos, val)| Result::Ok((pos, val.parse()?))).try_collect()?;

    let result1 = draw(Intcode::new(program.clone(), 0))?.len();

    let grid = draw(Intcode::new(program, 1))?;

    let mut min_re = 0;
    let mut max_re = 0;
    let mut min_im = 0;
    let mut max_im = 0;

    for position in grid.keys() {
        min_re = min_re.min(position.re);
        max_re = max_re.max(position.re);
        min_im = min_im.min(position.im);
        max_im = max_im.max(position.im);
    }

    let width = (max_re - min_re + 1) as usize;
    let height = (max_im - min_im + 1) as usize;

    let mut image = repeat(repeat(b' ').take(width).chain(once(b'\n'))).take(height).flatten().collect_vec();

    for (position, color) in grid {
        let x = (position.re - min_re) as usize;
        let y = (max_im - position.im) as usize;

        let pixel = match color {
            0 => b' ',
            1 => b'#',
            other => return Err(eyre!("unknown color: {other}")),
        };

        image[(width + 1) * y + x] = pixel;
    }

    let result2 = String::from_utf8_lossy(&image);

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
