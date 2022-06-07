use aoc::*;

use eyre::bail;
use itertools::{iproduct, Itertools};
use smallvec::SmallVec;

use std::collections::HashMap;

struct Intcode {
    program: HashMap<usize, i64>,
    ip: usize,
    relative_base: i64,
    inputs: SmallVec<[i64; 2]>,
}

impl Intcode {
    fn new(program: HashMap<usize, i64>, inputs: SmallVec<[i64; 2]>) -> Self {
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

    fn run(&mut self) -> Result<i64> {
        let mut output = None;

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
                    let input = self.inputs.pop().value()?;
                    let arg1 = self.get_register(1, instruction)?;
                    *arg1 = input;
                    self.ip += 2;
                }
                4 => {
                    let arg1 = self.get_input(1, instruction)?;
                    output = Some(arg1);
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

        output.value()
    }
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);
    let input = input.trim();

    let program: HashMap<usize, i64> = input.split(',').enumerate().map(|(pos, val)| Result::Ok((pos, val.parse()?))).try_collect()?;

    let run = |x, y| Intcode::new(program.clone(), SmallVec::from_buf([y, x])).run();

    let result1 = iproduct!(0..50, 0..50).map(|(x, y)| run(x, y)).try_process(|iter| iter.sum::<i64>())?;

    let mut x = 0;
    let mut y = 99;

    let result2 = loop {
        if run(x, y)? == 1 {
            if run(x + 99, y - 99)? == 1 {
                break x * 10000 + (y - 99);
            }
            y += 1;
        } else {
            x += 1;
        }
    };

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
