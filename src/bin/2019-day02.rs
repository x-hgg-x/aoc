use aoc::*;

use eyre::bail;
use itertools::{Itertools, iproduct};

fn run(mut program: Vec<usize>, noun: usize, verb: usize) -> Result<usize> {
    program[1] = noun;
    program[2] = verb;

    let mut ip = 0;
    loop {
        match program[ip] {
            1 => {
                let args = (program[ip + 1], program[ip + 2], program[ip + 3]);
                program[args.2] = program[args.0] + program[args.1]
            }
            2 => {
                let args = (program[ip + 1], program[ip + 2], program[ip + 3]);
                program[args.2] = program[args.0] * program[args.1]
            }
            99 => break,
            other => bail!("unknown opcode: {other}"),
        }
        ip += 4;
    }

    Ok(program[0])
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);
    let input = input.trim();

    let program: Vec<usize> = input.split(',').map(|x| x.parse()).try_collect()?;

    let result1 = run(program.clone(), 12, 2)?;

    let result2 = iproduct!(0..100, 0..100)
        .map(|(noun, verb)| Ok((noun, verb, run(program.clone(), noun, verb)?)))
        .try_process(|mut iter| iter.find(|&(.., x)| x == 19690720))?
        .map(|(noun, verb, _)| 100 * noun + verb)
        .value()?;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
