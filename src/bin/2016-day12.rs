use smallvec::SmallVec;

use std::fs;

enum Input {
    Register(usize),
    Value(i32),
}

enum Instruction {
    Copy(Input, usize),
    Increment(usize),
    Decrement(usize),
    JumpIfNotZero(Input, i32),
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

fn get_register(register: &str) -> usize {
    parse_register(register).unwrap_or_else(|| panic!("unknown register: {}", register))
}

fn get_input(input: &str) -> Input {
    match parse_register(input) {
        Some(r) => Input::Register(r),
        None => input
            .parse()
            .map(Input::Value)
            .unwrap_or_else(|_| panic!("unknown register or value: {}", input)),
    }
}

fn run(instructions: &[Instruction], mut registers: [i32; 4]) -> [i32; 4] {
    let mut ip = 0;
    while (0..instructions.len()).contains(&(ip as usize)) {
        match instructions[ip as usize] {
            Instruction::Copy(Input::Value(v), r) => registers[r] = v,
            Instruction::Copy(Input::Register(r0), r) => registers[r] = registers[r0],
            Instruction::Increment(r) => registers[r] += 1,
            Instruction::Decrement(r) => registers[r] -= 1,
            Instruction::JumpIfNotZero(Input::Value(v), offset) => {
                if v != 0 {
                    ip += offset;
                    continue;
                }
            }
            Instruction::JumpIfNotZero(Input::Register(r), offset) => {
                if registers[r] != 0 {
                    ip += offset;
                    continue;
                }
            }
        };
        ip += 1;
    }
    registers
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("inputs/2016-day12.txt")?;

    let instructions: Vec<_> = input
        .lines()
        .map(|line| {
            let args: SmallVec<[_; 3]> = line
                .split(|c: char| c.is_ascii_whitespace() || c == ',')
                .filter(|s| !s.is_empty())
                .collect();

            match args[0] {
                "cpy" => Instruction::Copy(get_input(args[1]), get_register(args[2])),
                "inc" => Instruction::Increment(get_register(args[1])),
                "dec" => Instruction::Decrement(get_register(args[1])),
                "jnz" => Instruction::JumpIfNotZero(get_input(args[1]), args[2].parse().unwrap()),
                other => panic!("unknown instruction: {}", other),
            }
        })
        .collect();

    let result1 = run(&instructions, [0, 0, 0, 0])[0];
    let result2 = run(&instructions, [0, 0, 1, 0])[0];

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
