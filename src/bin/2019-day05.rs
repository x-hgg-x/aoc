use aoc::*;

use eyre::bail;
use itertools::Itertools;

fn get_input(program: &[i64], instruction: i64, arg_position: u32, arg: i64) -> Result<i64> {
    match instruction / 10i64.pow(1 + arg_position) % 10 {
        0 => Ok(program[usize::try_from(arg)?]),
        1 => Ok(arg),
        other => bail!("unknown parameter mode: {other}"),
    }
}

fn run(mut program: Vec<i64>, input: i64) -> Result<i64> {
    let mut diagnostic_code = None;

    let mut ip = 0;
    loop {
        let instruction = program[ip];
        match instruction % 100 {
            1 => {
                let arg1 = get_input(&program, instruction, 1, program[ip + 1])?;
                let arg2 = get_input(&program, instruction, 2, program[ip + 2])?;
                let arg3 = program[ip + 3];
                program[usize::try_from(arg3)?] = arg1 + arg2;
                ip += 4;
            }
            2 => {
                let arg1 = get_input(&program, instruction, 1, program[ip + 1])?;
                let arg2 = get_input(&program, instruction, 2, program[ip + 2])?;
                let arg3 = program[ip + 3];
                program[usize::try_from(arg3)?] = arg1 * arg2;
                ip += 4;
            }
            3 => {
                let arg1 = program[ip + 1];
                program[usize::try_from(arg1)?] = input;
                ip += 2;
            }
            4 => {
                let arg1 = get_input(&program, instruction, 1, program[ip + 1])?;
                diagnostic_code = Some(arg1);
                ip += 2;
            }
            5 => {
                let arg1 = get_input(&program, instruction, 1, program[ip + 1])?;
                let arg2 = get_input(&program, instruction, 2, program[ip + 2])?;
                if arg1 != 0 {
                    ip = arg2.try_into()?;
                } else {
                    ip += 3;
                }
            }
            6 => {
                let arg1 = get_input(&program, instruction, 1, program[ip + 1])?;
                let arg2 = get_input(&program, instruction, 2, program[ip + 2])?;
                if arg1 == 0 {
                    ip = arg2.try_into()?;
                } else {
                    ip += 3;
                }
            }
            7 => {
                let arg1 = get_input(&program, instruction, 1, program[ip + 1])?;
                let arg2 = get_input(&program, instruction, 2, program[ip + 2])?;
                let arg3 = program[ip + 3];
                program[usize::try_from(arg3)?] = (arg1 < arg2).into();
                ip += 4;
            }
            8 => {
                let arg1 = get_input(&program, instruction, 1, program[ip + 1])?;
                let arg2 = get_input(&program, instruction, 2, program[ip + 2])?;
                let arg3 = program[ip + 3];
                program[usize::try_from(arg3)?] = (arg1 == arg2).into();
                ip += 4;
            }
            99 => break,
            other => bail!("unknown opcode: {other}"),
        }
    }

    diagnostic_code.value()
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);
    let input = input.trim();

    let program: Vec<i64> = input.split(',').map(|x| x.parse()).try_collect()?;

    let result1 = run(program.clone(), 1)?;
    let result2 = run(program, 5)?;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
