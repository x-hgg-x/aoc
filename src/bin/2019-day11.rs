use aoc::*;

use eyre::bail;
use itertools::Itertools;
use num_complex::Complex;
use smallvec::SmallVec;

use std::collections::HashMap;
use std::iter::{once, repeat};

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

    fn run(&mut self) -> Result<Option<SmallVec<[i64; 2]>>> {
        let mut outputs = SmallVec::new();

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
                    let input = self.input;
                    let arg1 = self.get_register(1, instruction)?;
                    *arg1 = input;
                    self.ip += 2;
                }
                4 => {
                    let arg1 = self.get_input(1, instruction)?;
                    self.ip += 2;
                    outputs.push(arg1);
                    if outputs.len() == 2 {
                        return Ok(Some(outputs));
                    }
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
                99 => return Ok(None),
                other => bail!("unknown opcode: {other}"),
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
            other => bail!("unknown turn: {other}"),
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
            other => bail!("unknown color: {other}"),
        };

        image[(width + 1) * y + x] = pixel;
    }

    let result2 = String::from_utf8_lossy(&image);

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
