use aoc::*;

use eyre::bail;
use itertools::Itertools;
use num_complex::Complex;

const NORTH: Complex<i64> = Complex::new(0, 1);
const SOUTH: Complex<i64> = Complex::new(0, -1);
const WEST: Complex<i64> = Complex::new(-1, 0);
const EAST: Complex<i64> = Complex::new(1, 0);
const LEFT_TURN: Complex<i64> = Complex::new(0, 1);
const RIGHT_TURN: Complex<i64> = Complex::new(0, -1);
const REVERSE_TURN: Complex<i64> = Complex::new(-1, 0);

enum Instruction {
    North(i64),
    South(i64),
    East(i64),
    West(i64),
    Forward(i64),
    Left,
    Right,
    Reverse,
}

fn compute_first_destination(instructions: &[Instruction]) -> i64 {
    let mut current_position = Complex::new(0, 0);
    let mut current_direction = EAST;

    for instruction in instructions {
        match instruction {
            Instruction::North(value) => current_position += value * NORTH,
            Instruction::South(value) => current_position += value * SOUTH,
            Instruction::East(value) => current_position += value * EAST,
            Instruction::West(value) => current_position += value * WEST,
            Instruction::Forward(value) => current_position += value * current_direction,
            Instruction::Left => current_direction *= LEFT_TURN,
            Instruction::Right => current_direction *= RIGHT_TURN,
            Instruction::Reverse => current_direction *= REVERSE_TURN,
        }
    }

    current_position.l1_norm()
}

fn compute_second_destination(instructions: &[Instruction]) -> i64 {
    let mut current_position = Complex::new(0, 0);
    let mut current_waypoint_position = Complex::new(10, 1);

    for instruction in instructions {
        match instruction {
            Instruction::North(value) => current_waypoint_position += value * NORTH,
            Instruction::South(value) => current_waypoint_position += value * SOUTH,
            Instruction::East(value) => current_waypoint_position += value * EAST,
            Instruction::West(value) => current_waypoint_position += value * WEST,
            Instruction::Forward(value) => current_position += value * current_waypoint_position,
            Instruction::Left => current_waypoint_position *= LEFT_TURN,
            Instruction::Right => current_waypoint_position *= RIGHT_TURN,
            Instruction::Reverse => current_waypoint_position *= REVERSE_TURN,
        }
    }

    current_position.l1_norm()
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let instructions: Vec<_> = input
        .lines()
        .map(|line| {
            let mut chars = line.chars();
            let action = chars.next();
            let value = chars.as_str().parse()?;

            match (action, value) {
                (Some('N'), _) => Ok(Instruction::North(value)),
                (Some('S'), _) => Ok(Instruction::South(value)),
                (Some('E'), _) => Ok(Instruction::East(value)),
                (Some('W'), _) => Ok(Instruction::West(value)),
                (Some('F'), _) => Ok(Instruction::Forward(value)),
                (Some('L'), 90) => Ok(Instruction::Left),
                (Some('L'), 180) => Ok(Instruction::Reverse),
                (Some('L'), 270) => Ok(Instruction::Right),
                (Some('R'), 90) => Ok(Instruction::Right),
                (Some('R'), 180) => Ok(Instruction::Reverse),
                (Some('R'), 270) => Ok(Instruction::Left),
                other => bail!("invalid action: {other:?}"),
            }
        })
        .try_collect()?;

    let result1 = compute_first_destination(&instructions);
    let result2 = compute_second_destination(&instructions);

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
