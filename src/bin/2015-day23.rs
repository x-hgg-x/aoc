use eyre::Result;
use itertools::Itertools;
use smallvec::SmallVec;

use std::fs;

enum Instruction {
    Half(usize),
    Triple(usize),
    Increment(usize),
    Jump(i64),
    JumpIfEven(usize, i64),
    JumpIfOne(usize, i64),
}

fn get_register(register: &str) -> usize {
    match register {
        "a" => 0,
        "b" => 1,
        other => panic!("unknown register: {}", other),
    }
}

fn run(instructions: &[Instruction], mut registers: [i64; 2]) -> Result<[i64; 2]> {
    let mut ip = 0;
    let range = 0..instructions.len().try_into()?;

    while range.contains(&ip) {
        match instructions[ip as usize] {
            Instruction::Half(r) => registers[r] /= 2,
            Instruction::Triple(r) => registers[r] *= 3,
            Instruction::Increment(r) => registers[r] += 1,
            Instruction::Jump(offset) => {
                ip += offset;
                continue;
            }
            Instruction::JumpIfEven(r, offset) => {
                if registers[r] % 2 == 0 {
                    ip += offset;
                    continue;
                }
            }
            Instruction::JumpIfOne(r, offset) => {
                if registers[r] == 1 {
                    ip += offset;
                    continue;
                }
            }
        };
        ip += 1;
    }
    Ok(registers)
}

fn main() -> Result<()> {
    let input = fs::read_to_string("inputs/2015-day23.txt")?;

    let instructions = input
        .lines()
        .map(|line| {
            let args = <SmallVec<[_; 3]>>::from_iter(line.split(|c: char| c.is_ascii_whitespace() || c == ',').filter(|s| !s.is_empty()));

            match args[0] {
                "hlf" => Instruction::Half(get_register(args[1])),
                "tpl" => Instruction::Triple(get_register(args[1])),
                "inc" => Instruction::Increment(get_register(args[1])),
                "jmp" => Instruction::Jump(args[1].parse().unwrap()),
                "jie" => Instruction::JumpIfEven(get_register(args[1]), args[2].parse().unwrap()),
                "jio" => Instruction::JumpIfOne(get_register(args[1]), args[2].parse().unwrap()),
                other => panic!("unknown instruction: {}", other),
            }
        })
        .collect_vec();

    let result1 = run(&instructions, [0, 0])?[1];
    let result2 = run(&instructions, [1, 0])?[1];

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
