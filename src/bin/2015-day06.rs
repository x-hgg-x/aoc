use aoc::*;

use eyre::bail;
use itertools::Itertools;
use regex::Regex;

use std::ops::RangeInclusive;

#[derive(Copy, Clone)]
enum Command {
    TurnOn,
    TurnOff,
    Toogle,
}

struct Instruction {
    command: Command,
    line_range: RangeInclusive<usize>,
    column_range: RangeInclusive<usize>,
}

fn compute_brightness<F, Func>(grid: &mut [[u8; 1000]], instructions: &[Instruction], f: F) -> u64
where
    F: Fn(Command) -> Func,
    Func: Fn(u8) -> u8,
{
    for instruction in instructions {
        let func = f(instruction.command);

        for grid_line in &mut grid[instruction.line_range.clone()] {
            for elem in &mut grid_line[instruction.column_range.clone()] {
                *elem = func(*elem);
            }
        }
    }

    grid.iter().flatten().copied().map_into::<u64>().sum()
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let re = Regex::new(r#"(?m)^(.*?) (\d+),(\d+) through (\d+),(\d+)$"#)?;

    let instructions: Vec<_> = re
        .captures_iter(&input)
        .map(|x| {
            let command = match &x[1] {
                "turn on" => Command::TurnOn,
                "turn off" => Command::TurnOff,
                "toggle" => Command::Toogle,
                other => bail!("unknown instruction: {other}"),
            };

            let line_range = x[2].parse()?..=x[4].parse()?;
            let column_range = x[3].parse()?..=x[5].parse()?;

            Ok(Instruction {
                command,
                line_range,
                column_range,
            })
        })
        .try_collect()?;

    let f1 = |instruction| match instruction {
        Command::TurnOn => |_| 1,
        Command::TurnOff => |_| 0,
        Command::Toogle => |x| x ^ 1,
    };

    let f2 = |instruction| match instruction {
        Command::TurnOn => |x| x + 1,
        Command::TurnOff => |x: u8| x.saturating_sub(1),
        Command::Toogle => |x| x + 2,
    };

    let mut grid = vec![[0; 1000]; 1000];
    let result1 = compute_brightness(&mut grid, &instructions, f1);

    grid.fill([0; 1000]);
    let result2 = compute_brightness(&mut grid, &instructions, f2);

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
