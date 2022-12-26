use aoc::*;

use eyre::bail;
use itertools::Itertools;
use smallvec::SmallVec;

use std::iter::once;

struct Permutations<'a, T, const N: usize> {
    data: &'a [T],
    available: SmallVec<[T; N]>,
    buf: SmallVec<[T; N]>,
    factorials: Vec<i64>,
    factorial_index: i64,
}

impl<'a, T, const N: usize> Permutations<'a, T, N> {
    fn new(data: &'a [T]) -> Self {
        Self { data, available: SmallVec::new(), buf: SmallVec::new(), factorials: Self::compute_factorials(data.len() as i64), factorial_index: 0 }
    }

    fn compute_factorials(num: i64) -> Vec<i64> {
        once(1)
            .chain((1..=num).scan(1, |state, x| {
                *state *= x;
                Some(*state)
            }))
            .collect_vec()
    }
}

impl<'a, T: Copy, const N: usize> Iterator for Permutations<'a, T, N> {
    type Item = SmallVec<[T; N]>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.factorial_index >= self.factorials[self.data.len()] {
            return None;
        }

        let mut x = self.factorial_index;

        self.buf.clear();
        self.available = SmallVec::from_slice(self.data);

        self.buf.extend(self.factorials[..self.data.len()].iter().rev().map(|&place_value| {
            let index = x / place_value;
            x -= index * place_value;
            self.available.remove(index.rem_euclid(self.available.len() as i64) as usize)
        }));

        self.factorial_index += 1;

        Some(self.buf.clone())
    }
}

struct Intcode {
    program: Vec<i64>,
    ip: usize,
    inputs: SmallVec<[i64; 2]>,
}

impl Intcode {
    fn new(program: Vec<i64>, inputs: SmallVec<[i64; 2]>) -> Self {
        Self { program, ip: 0, inputs }
    }

    fn get_input(&self, arg_position: usize, instruction: i64) -> Result<i64> {
        let arg = self.program[self.ip + arg_position];

        match instruction / 10i64.pow(1 + arg_position as u32) % 10 {
            0 => Ok(self.program[usize::try_from(arg)?]),
            1 => Ok(arg),
            other => bail!("unknown parameter mode: {other}"),
        }
    }

    fn run(&mut self) -> Result<Option<i64>> {
        loop {
            let instruction = self.program[self.ip];
            match instruction % 100 {
                1 => {
                    let arg1 = self.get_input(1, instruction)?;
                    let arg2 = self.get_input(2, instruction)?;
                    let arg3 = self.program[self.ip + 3];
                    self.program[usize::try_from(arg3)?] = arg1 + arg2;
                    self.ip += 4;
                }
                2 => {
                    let arg1 = self.get_input(1, instruction)?;
                    let arg2 = self.get_input(2, instruction)?;
                    let arg3 = self.program[self.ip + 3];
                    self.program[usize::try_from(arg3)?] = arg1 * arg2;
                    self.ip += 4;
                }
                3 => {
                    let arg1 = self.program[self.ip + 1];
                    self.program[usize::try_from(arg1)?] = self.inputs.remove(0);
                    self.ip += 2;
                }
                4 => {
                    let arg1 = self.get_input(1, instruction)?;
                    self.ip += 2;
                    return Ok(Some(arg1));
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
                    let arg3 = self.program[self.ip + 3];
                    self.program[usize::try_from(arg3)?] = (arg1 < arg2).into();
                    self.ip += 4;
                }
                8 => {
                    let arg1 = self.get_input(1, instruction)?;
                    let arg2 = self.get_input(2, instruction)?;
                    let arg3 = self.program[self.ip + 3];
                    self.program[usize::try_from(arg3)?] = (arg1 == arg2).into();
                    self.ip += 4;
                }
                99 => return Ok(None),
                other => bail!("unknown opcode: {other}"),
            }
        }
    }
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);
    let input = input.trim();

    let program: Vec<i64> = input.split(',').map(|x| x.parse()).try_collect()?;

    let result1 = Permutations::<_, 5>::new(&[0, 1, 2, 3, 4])
        .map(|permutation| {
            let mut in_out = 0;
            for phase_setting in permutation {
                let mut intcode = Intcode::new(program.clone(), SmallVec::from_buf([phase_setting, in_out]));
                in_out = intcode.run()?.value()?;
            }
            Ok(in_out)
        })
        .try_process(|iter| iter.max())?
        .value()?;

    let result2 = Permutations::<_, 5>::new(&[5, 6, 7, 8, 9])
        .map(|permutation| {
            let mut intcodes: SmallVec<[_; 5]> =
                permutation.iter().map(|&phase_setting| Intcode::new(program.clone(), SmallVec::from_slice(&[phase_setting]))).collect();

            let mut halted_programs = [false; 5];

            let mut in_out = 0;
            loop {
                for (intcode, halted) in intcodes.iter_mut().zip(&mut halted_programs) {
                    if !*halted {
                        intcode.inputs.push(in_out);

                        match intcode.run()? {
                            Some(value) => in_out = value,
                            None => *halted = true,
                        }
                    }
                }
                if halted_programs.iter().all(|&x| x) {
                    break Ok(in_out);
                }
            }
        })
        .try_process(|iter| iter.max())?
        .value()?;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
