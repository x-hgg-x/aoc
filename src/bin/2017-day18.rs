use aoc::*;

use eyre::{bail, eyre};
use itertools::{izip, Itertools};
use smallvec::SmallVec;

use std::collections::{HashMap, VecDeque};

#[derive(Copy, Clone)]
enum Input {
    Register(u8),
    Value(i64),
}

impl Input {
    fn get_value(&self, registers: &mut HashMap<u8, i64>) -> i64 {
        match *self {
            Input::Register(r) => *registers.entry(r).or_default(),
            Input::Value(v) => v,
        }
    }
}

enum Instruction1 {
    Sound(Input),
    Set(u8, Input),
    Addition(u8, Input),
    Multiplication(u8, Input),
    Modulo(u8, Input),
    RecoverIfNotZero(Input),
    JumpIfGreaterThanZero(Input, Input),
}

enum Instruction2 {
    Send(Input),
    Set(u8, Input),
    Addition(u8, Input),
    Multiplication(u8, Input),
    Modulo(u8, Input),
    Receive(u8),
    JumpIfGreaterThanZero(Input, Input),
}

fn convert_instruction(instruction: &Instruction1) -> Result<Instruction2> {
    Ok(match *instruction {
        Instruction1::Sound(input) => Instruction2::Send(input),
        Instruction1::Set(r, input) => Instruction2::Set(r, input),
        Instruction1::Addition(r, input) => Instruction2::Addition(r, input),
        Instruction1::Multiplication(r, input) => Instruction2::Multiplication(r, input),
        Instruction1::Modulo(r, input) => Instruction2::Modulo(r, input),
        Instruction1::RecoverIfNotZero(Input::Register(r)) => Instruction2::Receive(r),
        Instruction1::JumpIfGreaterThanZero(input1, input2) => Instruction2::JumpIfGreaterThanZero(input1, input2),
        _ => bail!("unable to convert instruction: Instruction1::RecoverIfNotZero(Input::Value)"),
    })
}

fn get_input(input: &str) -> Input {
    match input.parse() {
        Ok(value) => Input::Value(value),
        Err(_) => Input::Register(input.as_bytes()[0]),
    }
}

fn run1(instructions: &[Instruction1]) -> Result<i64> {
    let mut registers = HashMap::new();
    let mut last_frequency = None;
    let mut ip = 0;
    let range = 0..instructions.len().try_into()?;

    while range.contains(&ip) {
        match instructions[ip as usize] {
            Instruction1::Sound(input) => last_frequency = Some(input.get_value(&mut registers)),
            Instruction1::Set(r, input) => *registers.entry(r).or_default() = input.get_value(&mut registers),
            Instruction1::Addition(r, input) => *registers.entry(r).or_default() += input.get_value(&mut registers),
            Instruction1::Multiplication(r, input) => *registers.entry(r).or_default() *= input.get_value(&mut registers),
            Instruction1::Modulo(r, input) => *registers.entry(r).or_default() %= input.get_value(&mut registers),
            Instruction1::RecoverIfNotZero(input) => {
                if input.get_value(&mut registers) != 0 {
                    return last_frequency.ok_or_else(|| eyre!("no sound was emitted"));
                }
            }
            Instruction1::JumpIfGreaterThanZero(input1, input2) => {
                if input1.get_value(&mut registers) > 0 {
                    ip += input2.get_value(&mut registers);
                    continue;
                }
            }
        };
        ip += 1;
    }

    bail!("no frequency was recovered")
}

fn run2(instructions: &[Instruction2]) -> Result<usize> {
    let mut program_1_send_count = 0;

    let mut all_registers = [HashMap::from([(b'p', 0)]), HashMap::from([(b'p', 1)])];
    let mut ips = [0, 0];

    let program_ids = [0, 1];
    let mut queue_0 = VecDeque::new();
    let mut queue_1 = VecDeque::new();

    let mut locked = [false, false];
    let mut finished = [false, false];

    let range = 0..instructions.len().try_into()?;

    'run: loop {
        for (ip, registers, program_id) in izip!(&mut ips, &mut all_registers, program_ids) {
            let (self_queue, other_queue, other_program_id) = match program_id {
                0 => (&mut queue_0, &mut queue_1, (program_id ^ 1) as usize),
                1 => (&mut queue_1, &mut queue_0, (program_id ^ 1) as usize),
                other => bail!("unknown program id: {other}"),
            };

            loop {
                if !range.contains(ip) {
                    finished[program_id] = true;
                    break;
                }

                match instructions[*ip as usize] {
                    Instruction2::Send(input) => {
                        locked[other_program_id] = false;
                        other_queue.push_back(input.get_value(registers));

                        if program_id == 1 {
                            program_1_send_count += 1;
                        }
                    }
                    Instruction2::Set(r, input) => *registers.entry(r).or_default() = input.get_value(registers),
                    Instruction2::Addition(r, input) => *registers.entry(r).or_default() += input.get_value(registers),
                    Instruction2::Multiplication(r, input) => *registers.entry(r).or_default() *= input.get_value(registers),
                    Instruction2::Modulo(r, input) => *registers.entry(r).or_default() %= input.get_value(registers),
                    Instruction2::Receive(r) => match self_queue.pop_front() {
                        Some(value) => *registers.entry(r).or_default() = value,
                        None => {
                            locked[program_id] = true;
                            break;
                        }
                    },
                    Instruction2::JumpIfGreaterThanZero(input1, input2) => {
                        if input1.get_value(registers) > 0 {
                            *ip += input2.get_value(registers);
                            continue;
                        }
                    }
                };
                *ip += 1;
            }

            if locked.into_iter().zip(finished).all(|(is_locked, is_finished)| is_locked || is_finished) {
                break 'run Ok(program_1_send_count);
            }
        }
    }
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let instructions_1: Vec<_> = input
        .lines()
        .map(|line| {
            let args: SmallVec<[_; 3]> = line.split_ascii_whitespace().collect();

            Ok(match args[0] {
                "snd" => Instruction1::Sound(get_input(args[1])),
                "set" => Instruction1::Set(args[1].as_bytes()[0], get_input(args[2])),
                "add" => Instruction1::Addition(args[1].as_bytes()[0], get_input(args[2])),
                "mul" => Instruction1::Multiplication(args[1].as_bytes()[0], get_input(args[2])),
                "mod" => Instruction1::Modulo(args[1].as_bytes()[0], get_input(args[2])),
                "rcv" => Instruction1::RecoverIfNotZero(get_input(args[1])),
                "jgz" => Instruction1::JumpIfGreaterThanZero(get_input(args[1]), get_input(args[2])),
                other => bail!("unknown instruction: {other}"),
            })
        })
        .try_collect()?;

    let result1 = run1(&instructions_1)?;

    let instructions_2: Vec<_> = instructions_1.iter().map(convert_instruction).try_collect()?;
    let result2 = run2(&instructions_2)?;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
