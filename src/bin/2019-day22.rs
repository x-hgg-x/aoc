use aoc::*;

use eyre::bail;
use itertools::Itertools;

const BIG_SIZE: i128 = 119315717514047;

trait Modulo: Sized {
    fn inv(self) -> Self;
    fn sub(self, other: Self) -> Self;
    fn add(self, other: Self) -> Self;
    fn mul(self, other: Self) -> Self;
    fn power(self, other: Self) -> Self;
}

impl Modulo for i128 {
    fn inv(self) -> Self {
        let (mut r1, mut u1, mut r2, mut u2) = (self, 1, BIG_SIZE, 0);

        while r2 != 0 {
            let q = r1 / r2;
            (r1, u1, r2, u2) = (r2, u2, r1 - q * r2, u1 - q * u2);
        }

        u1.rem_euclid(BIG_SIZE)
    }

    fn add(self, other: Self) -> Self {
        (self + other).rem_euclid(BIG_SIZE)
    }

    fn sub(self, other: Self) -> Self {
        (self - other).rem_euclid(BIG_SIZE)
    }

    fn mul(self, other: Self) -> Self {
        (self * other).rem_euclid(BIG_SIZE)
    }

    fn power(mut self, mut other: Self) -> Self {
        self = self.rem_euclid(BIG_SIZE);

        let mut res = 1;
        while other > 0 {
            if other & 1 != 0 {
                res = res.mul(self);
            }
            self = self.mul(self);
            other >>= 1;
        }
        res
    }
}

enum Instruction {
    DealNewStack,
    Cut(i128),
    DealIncrement(i128),
}

fn compute_card_position(
    instructions: &[Instruction],
    size: i128,
    mut current_position: i128,
) -> i128 {
    for instruction in instructions {
        match *instruction {
            Instruction::DealNewStack => current_position = size - 1 - current_position,
            Instruction::Cut(n) => current_position = (current_position - n) % size,
            Instruction::DealIncrement(n) => current_position = (current_position * n) % size,
        }
    }
    current_position
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let instructions: Vec<_> = input
        .lines()
        .map(|line| {
            if line == "deal into new stack" {
                Ok(Instruction::DealNewStack)
            } else if line.starts_with("cut") {
                Ok(Instruction::Cut(
                    line.split_ascii_whitespace().last().value()?.parse()?,
                ))
            } else if line.starts_with("deal with increment") {
                Ok(Instruction::DealIncrement(
                    line.split_ascii_whitespace().last().value()?.parse()?,
                ))
            } else {
                bail!("unkwnown instruction: {line}")
            }
        })
        .try_collect()?;

    let result1 = compute_card_position(&instructions, 10007, 2019);

    let reversed_instructions = instructions
        .iter()
        .rev()
        .map(|instruction| match *instruction {
            Instruction::DealNewStack => Instruction::DealNewStack,
            Instruction::Cut(n) => Instruction::Cut(-n),
            Instruction::DealIncrement(n) => Instruction::DealIncrement(n.inv()),
        })
        .collect_vec();

    let position_0 = 2020_i128;
    let position_1 = compute_card_position(&reversed_instructions, BIG_SIZE, position_0);
    let position_2 = compute_card_position(&reversed_instructions, BIG_SIZE, position_1);

    let inv_0_1 = (position_0.sub(position_1)).inv();
    let scale = (position_1 - position_2).mul(inv_0_1);
    let shift = position_1.sub(scale.mul(position_0));
    let scale_pow_n = scale.power(101741582076661);

    let result2 = scale_pow_n
        .mul(position_0)
        .add((scale_pow_n - 1).mul((scale - 1).inv()).mul(shift));

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
