use eyre::Result;
use itertools::Itertools;
use smallvec::SmallVec;

use std::fs;

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
    Transmit(Input),
}

fn parse_register(register: &str) -> Option<usize> {
    match register {
        "a" => Some(0),
        "b" => Some(1),
        "c" => Some(2),
        "d" => Some(3),
        _ => None,
    }
}

fn get_input(input: &str) -> Input {
    match parse_register(input) {
        Some(r) => Input::Register(r),
        None => input.parse().map(Input::Value).unwrap_or_else(|_| panic!("unknown register or value: {}", input)),
    }
}

fn run(instructions: &mut [Instruction], mut registers: [i64; 4]) -> Result<SmallVec<[i64; 10]>> {
    let mut clock = SmallVec::new();
    let mut ip = 0;
    let range = 0..instructions.len().try_into()?;

    while range.contains(&ip) && clock.len() < clock.capacity() {
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
                        Instruction::Transmit(input) => Instruction::Increment(input),
                    };
                }
            }
            Instruction::Transmit(input) => clock.push(input.get_value(&registers)),
            _ => (),
        };
        ip += 1;
    }
    Ok(clock)
}

fn main() -> Result<()> {
    let input = fs::read_to_string("inputs/2016-day25.txt")?;

    let instructions = input
        .lines()
        .map(|line| {
            let args: SmallVec<[_; 3]> = line.split_ascii_whitespace().collect();

            match args[0] {
                "cpy" => Instruction::Copy(get_input(args[1]), get_input(args[2])),
                "inc" => Instruction::Increment(get_input(args[1])),
                "dec" => Instruction::Decrement(get_input(args[1])),
                "jnz" => Instruction::JumpIfNotZero(get_input(args[1]), get_input(args[2])),
                "tgl" => Instruction::Toogle(get_input(args[1])),
                "out" => Instruction::Transmit(get_input(args[1])),
                other => panic!("unknown instruction: {}", other),
            }
        })
        .collect_vec();

    let mut buf = Vec::with_capacity(instructions.len());

    let result = (0..)
        .find(|&a| {
            buf.clone_from(&instructions);
            run(&mut buf, [a, 0, 0, 0]).unwrap().into_iter().zip([0, 1].into_iter().cycle()).all(|(x, y)| x == y)
        })
        .unwrap();

    println!("{}", result);
    Ok(())
}
