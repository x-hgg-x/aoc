use aoc::*;

use eyre::bail;
use itertools::Itertools;
use smallvec::SmallVec;

use std::iter::once;

enum Instruction {
    Noop,
    Add(i64),
}

fn run(instructions: &[Instruction]) -> (i64, Vec<u8>) {
    let mut cycle_count = 0;
    let mut strength = 0;
    let mut sprite_position = 1;
    let mut buffer = Vec::with_capacity(240);

    let mut step = |sprite_position: i64, duration: usize| {
        for _ in 0..duration {
            if (sprite_position - cycle_count % 40).abs() <= 1 {
                buffer.push(b'#');
            } else {
                buffer.push(b' ');
            }

            cycle_count += 1;

            if cycle_count % 40 == 20 {
                strength += cycle_count * sprite_position;
            }
        }
    };

    for instruction in instructions {
        match instruction {
            Instruction::Noop => {
                step(sprite_position, 1);
            }
            Instruction::Add(n) => {
                step(sprite_position, 2);
                sprite_position += n;
            }
        };
    }

    let image = buffer
        .chunks_exact(40)
        .flat_map(|x| x.iter().copied().chain(once(b'\n')))
        .collect_vec();

    (strength, image)
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let instructions: Vec<_> = input
        .lines()
        .map(|line| {
            let args = <SmallVec<[_; 2]>>::from_iter(line.split_ascii_whitespace());

            match args[0] {
                "noop" => Ok(Instruction::Noop),
                "addx" => Ok(Instruction::Add(args[1].parse()?)),
                other => bail!("unknown instruction: {other}"),
            }
        })
        .try_collect()?;

    let (strength, image) = run(&instructions);

    let result1 = strength;
    let result2 = String::from_utf8_lossy(&image);

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
