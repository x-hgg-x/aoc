use eyre::Result;
use itertools::Itertools;
use smallvec::SmallVec;

use std::fs;

#[derive(Copy, Clone, Debug)]
enum Input {
    Register(usize),
    Value(i64),
}

impl Input {
    fn get_value(&self, registers: &[i64; 8]) -> i64 {
        match *self {
            Input::Register(r) => registers[r],
            Input::Value(v) => v,
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Set(usize, Input),
    Substraction(usize, Input),
    Multiplication(usize, Input),
    JumpIfNotZero(Input, Input),
}

fn parse_register(register: &str) -> Option<usize> {
    match register.bytes().next() {
        Some(x @ b'a'..=b'h') => Some((x - b'a').into()),
        _ => None,
    }
}

fn get_register(register: &str) -> usize {
    parse_register(register).unwrap_or_else(|| panic!("unknown register: {}", register))
}

fn get_input(input: &str) -> Input {
    match parse_register(input) {
        Some(r) => Input::Register(r),
        None => input.parse().map(Input::Value).unwrap_or_else(|_| panic!("unknown register or value: {}", input)),
    }
}

fn run(instructions: &[Instruction], mut registers: [i64; 8]) -> Result<([i64; 8], usize)> {
    let mut mul_count = 0;
    let mut ip = 0;
    let range = 0..instructions.len().try_into()?;

    while range.contains(&ip) {
        match instructions[ip as usize] {
            Instruction::Set(r, input) => registers[r] = input.get_value(&registers),
            Instruction::Substraction(r, input) => registers[r] -= input.get_value(&registers),
            Instruction::Multiplication(r, input) => {
                registers[r] *= input.get_value(&registers);
                mul_count += 1;
            }
            Instruction::JumpIfNotZero(input1, input2) => {
                if input1.get_value(&registers) != 0 {
                    ip += input2.get_value(&registers);
                    continue;
                }
            }
        };
        ip += 1;
    }
    Ok((registers, mul_count))
}

fn main() -> Result<()> {
    let input = fs::read_to_string("inputs/2017-day23.txt")?;

    let instructions = input
        .lines()
        .map(|line| {
            let args: SmallVec<[_; 3]> = line.split_ascii_whitespace().collect();

            match args[0] {
                "set" => Instruction::Set(get_register(args[1]), get_input(args[2])),
                "sub" => Instruction::Substraction(get_register(args[1]), get_input(args[2])),
                "mul" => Instruction::Multiplication(get_register(args[1]), get_input(args[2])),
                "jnz" => Instruction::JumpIfNotZero(get_input(args[1]), get_input(args[2])),
                other => panic!("unknown instruction: {}", other),
            }
        })
        .collect_vec();

    let (_, mul_count) = run(&instructions, [0, 0, 0, 0, 0, 0, 0, 0])?;

    let (registers, _) = run(&instructions[..8], [1, 0, 0, 0, 0, 0, 0, 0])?;
    let [_, min, max, ..] = registers;

    let sqrt = (max as f64).sqrt() as i64;
    let composite_number_count = (min..=max).step_by(17).filter(|&x| (2..=sqrt).any(|i| x % i == 0)).count();

    let result1 = mul_count;
    let result2 = composite_number_count;

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
