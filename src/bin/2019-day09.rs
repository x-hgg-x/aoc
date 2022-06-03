use aoc::*;

use eyre::eyre;
use itertools::Itertools;

use std::collections::HashMap;

fn get_input(program: &mut HashMap<usize, i64>, ip: usize, arg_position: usize, relative_base: i64, instruction: i64) -> Result<i64> {
    let arg = *program.entry(ip + arg_position).or_default();

    match instruction / 10i64.pow(1 + arg_position as u32) % 10 {
        0 => Ok(*program.entry(usize::try_from(arg)?).or_default()),
        1 => Ok(arg),
        2 => Ok(*program.entry(usize::try_from(relative_base + arg)?).or_default()),
        other => Err(eyre!("unknown parameter mode: {other}")),
    }
}

fn get_register(program: &mut HashMap<usize, i64>, ip: usize, arg_position: usize, relative_base: i64, instruction: i64) -> Result<&mut i64> {
    let arg = *program.entry(ip + arg_position).or_default();

    match instruction / 10i64.pow(1 + arg_position as u32) % 10 {
        0 => Ok(program.entry(usize::try_from(arg)?).or_default()),
        2 => Ok(program.entry(usize::try_from(relative_base + arg)?).or_default()),
        other => Err(eyre!("invalid parameter mode: {other}")),
    }
}

fn run(mut program: HashMap<usize, i64>, input: i64) -> Result<i64> {
    let mut output = None;

    let mut ip = 0;
    let mut relative_base = 0;
    loop {
        let instruction = *program.entry(ip).or_default();
        match instruction % 100 {
            1 => {
                let arg1 = get_input(&mut program, ip, 1, relative_base, instruction)?;
                let arg2 = get_input(&mut program, ip, 2, relative_base, instruction)?;
                let arg3 = get_register(&mut program, ip, 3, relative_base, instruction)?;
                *arg3 = arg1 + arg2;
                ip += 4;
            }
            2 => {
                let arg1 = get_input(&mut program, ip, 1, relative_base, instruction)?;
                let arg2 = get_input(&mut program, ip, 2, relative_base, instruction)?;
                let arg3 = get_register(&mut program, ip, 3, relative_base, instruction)?;
                *arg3 = arg1 * arg2;
                ip += 4;
            }
            3 => {
                let arg1 = get_register(&mut program, ip, 1, relative_base, instruction)?;
                *arg1 = input;
                ip += 2;
            }
            4 => {
                let arg1 = get_input(&mut program, ip, 1, relative_base, instruction)?;
                output = Some(arg1);
                ip += 2;
            }
            5 => {
                let arg1 = get_input(&mut program, ip, 1, relative_base, instruction)?;
                let arg2 = get_input(&mut program, ip, 2, relative_base, instruction)?;
                if arg1 != 0 {
                    ip = arg2.try_into()?;
                } else {
                    ip += 3;
                }
            }
            6 => {
                let arg1 = get_input(&mut program, ip, 1, relative_base, instruction)?;
                let arg2 = get_input(&mut program, ip, 2, relative_base, instruction)?;
                if arg1 == 0 {
                    ip = arg2.try_into()?;
                } else {
                    ip += 3;
                }
            }
            7 => {
                let arg1 = get_input(&mut program, ip, 1, relative_base, instruction)?;
                let arg2 = get_input(&mut program, ip, 2, relative_base, instruction)?;
                let arg3 = get_register(&mut program, ip, 3, relative_base, instruction)?;
                *arg3 = (arg1 < arg2).into();
                ip += 4;
            }
            8 => {
                let arg1 = get_input(&mut program, ip, 1, relative_base, instruction)?;
                let arg2 = get_input(&mut program, ip, 2, relative_base, instruction)?;
                let arg3 = get_register(&mut program, ip, 3, relative_base, instruction)?;
                *arg3 = (arg1 == arg2).into();
                ip += 4;
            }
            9 => {
                let arg1 = get_input(&mut program, ip, 1, relative_base, instruction)?;
                relative_base += arg1;
                ip += 2;
            }
            99 => break,
            other => return Err(eyre!("unknown opcode: {other}")),
        }
    }

    output.value()
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);
    let input = input.trim();

    let program: HashMap<usize, i64> = input.split(',').enumerate().map(|(pos, val)| Result::Ok((pos, val.parse()?))).try_collect()?;

    let result1 = run(program.clone(), 1)?;
    let result2 = run(program, 2)?;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
