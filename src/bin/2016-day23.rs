use aoc::*;

use eyre::{WrapErr, bail, eyre};
use itertools::Itertools;
use smallvec::SmallVec;

#[derive(Copy, Clone)]
enum Input {
    Register(usize),
    Value(i64),
}

impl Input {
    fn get_value(&self, registers: &[i64; 4]) -> i64 {
        match *self {
            Input::Register(r) => registers[r],
            Input::Value(v) => v,
        }
    }
}

#[derive(Clone)]
enum Instruction {
    Copy(Input, Input),
    Increment(Input),
    Decrement(Input),
    JumpIfNotZero(Input, Input),
    Toogle(Input),
}

fn parse_register(register: &str) -> Option<usize> {
    match register.as_bytes()[0] {
        x @ b'a'..=b'd' => Some((x - b'a').into()),
        _ => None,
    }
}

fn get_input(input: &str) -> Result<Input> {
    match parse_register(input) {
        Some(r) => Ok(Input::Register(r)),
        None => input.parse().map(Input::Value).wrap_err_with(|| eyre!("unknown register or value: {input}")),
    }
}

fn run(mut instructions: Vec<Instruction>, mut registers: [i64; 4]) -> Result<[i64; 4]> {
    let mut ip = 0;
    let range = 0..instructions.len().try_into()?;

    while range.contains(&ip) {
        match instructions[ip as usize] {
            Instruction::Copy(input, Input::Register(r)) => registers[r] = input.get_value(&registers),
            Instruction::Increment(Input::Register(r)) => registers[r] += 1,
            Instruction::Decrement(Input::Register(r)) => registers[r] -= 1,
            Instruction::JumpIfNotZero(input1, input2) => {
                if input1.get_value(&registers) != 0 {
                    ip += input2.get_value(&registers);
                    continue;
                }
            }
            Instruction::Toogle(input) => {
                let idx = ip + input.get_value(&registers);
                if (0..instructions.len().try_into()?).contains(&idx) {
                    let toggled_instruction = &mut instructions[idx as usize];
                    *toggled_instruction = match *toggled_instruction {
                        Instruction::Copy(input1, input2) => Instruction::JumpIfNotZero(input1, input2),
                        Instruction::Increment(input) => Instruction::Decrement(input),
                        Instruction::Decrement(input) => Instruction::Increment(input),
                        Instruction::JumpIfNotZero(input1, input2) => Instruction::Copy(input1, input2),
                        Instruction::Toogle(input) => Instruction::Increment(input),
                    };
                }
            }
            _ => (),
        };
        ip += 1;
    }
    Ok(registers)
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let instructions: Vec<_> = input
        .lines()
        .map(|line| {
            let args: SmallVec<[_; 3]> = line.split_ascii_whitespace().collect();

            Ok(match args[0] {
                "cpy" => Instruction::Copy(get_input(args[1])?, get_input(args[2])?),
                "inc" => Instruction::Increment(get_input(args[1])?),
                "dec" => Instruction::Decrement(get_input(args[1])?),
                "jnz" => Instruction::JumpIfNotZero(get_input(args[1])?, get_input(args[2])?),
                "tgl" => Instruction::Toogle(get_input(args[1])?),
                other => bail!("unknown instruction: {other}"),
            })
        })
        .try_collect()?;

    let result1 = run(instructions.clone(), [7, 0, 0, 0])?[0];
    let result2 = run(instructions, [12, 0, 0, 0])?[0];

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
