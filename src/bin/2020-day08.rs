use aoc::*;

use eyre::bail;
use itertools::Itertools;

use std::collections::HashSet;

enum Instruction {
    Nop(i64),
    Accumulate(i64),
    Jump(i64),
}

impl Instruction {
    fn swap(&mut self) {
        *self = match *self {
            Instruction::Nop(value) => Instruction::Jump(value),
            Instruction::Accumulate(value) => Instruction::Accumulate(value),
            Instruction::Jump(value) => Instruction::Nop(value),
        };
    }
}

fn run(instructions: &[Instruction]) -> Result<std::result::Result<i64, i64>> {
    let mut acc = 0;
    let mut previous_ips = HashSet::new();

    let mut ip = 0;
    let range = 0..instructions.len().try_into()?;

    while range.contains(&ip) {
        if !previous_ips.insert(ip) {
            return Ok(Err(acc));
        }

        match instructions[ip as usize] {
            Instruction::Nop(_) => (),
            Instruction::Accumulate(value) => acc += value,
            Instruction::Jump(value) => {
                ip += value;
                continue;
            }
        };
        ip += 1;
    }

    Ok(Ok(acc))
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let mut instructions: Vec<_> = input
        .lines()
        .map(|line| {
            let (name, arg) = line.split_ascii_whitespace().next_tuple().value()?;

            Ok(match name {
                "nop" => Instruction::Nop(arg.parse()?),
                "acc" => Instruction::Accumulate(arg.parse()?),
                "jmp" => Instruction::Jump(arg.parse()?),
                other => bail!("unknown instruction: {other}"),
            })
        })
        .try_collect()?;

    let result1 = run(&instructions)?.err().value()?;

    let result2 = (0..instructions.len())
        .map(|index| {
            instructions[index].swap();

            if let Ok(acc) = run(&instructions)? {
                return Ok(Some(acc));
            }

            instructions[index].swap();
            Ok(None)
        })
        .try_process(|mut iter| iter.find_map(|x| x))?
        .value()?;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
