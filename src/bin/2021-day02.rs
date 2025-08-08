use aoc::*;

use eyre::bail;
use itertools::Itertools;

enum Instruction {
    Forward(i64),
    Up(i64),
    Down(i64),
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let instructions: Vec<_> = input
        .lines()
        .map(|line| {
            let (command, value) = line.split_ascii_whitespace().next_tuple().value()?;
            let value = value.parse()?;

            match command {
                "forward" => Ok(Instruction::Forward(value)),
                "up" => Ok(Instruction::Up(value)),
                "down" => Ok(Instruction::Down(value)),
                other => bail!("invalid command: {other}"),
            }
        })
        .try_collect()?;

    let (x1, y1) = instructions
        .iter()
        .fold((0, 0), |(x, y), value| match value {
            Instruction::Forward(value) => (x + value, y),
            Instruction::Up(value) => (x, y - value),
            Instruction::Down(value) => (x, y + value),
        });

    let (x2, y2, _) = instructions
        .iter()
        .fold((0, 0, 0), |(x, y, aim), value| match value {
            Instruction::Forward(value) => (x + value, y + aim * value, aim),
            Instruction::Up(value) => (x, y, aim - value),
            Instruction::Down(value) => (x, y, aim + value),
        });

    let result1 = x1 * y1;
    let result2 = x2 * y2;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
