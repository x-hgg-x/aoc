use aoc::*;

use eyre::{bail, ensure};
use itertools::Itertools;
use smallvec::SmallVec;

use std::iter::once;

const MAX_VALUE: u8 = 252;
const BR_LEFT: u8 = 253;
const COMMA: u8 = 254;
const BR_RIGHT: u8 = 255;

#[derive(Clone)]
struct Number(Vec<u8>);

impl Number {
    fn parse(s: &str) -> Result<Self> {
        let number = Number(s.bytes().map(Self::to_number_byte).try_collect()?);
        let max_depth = number.depth_iter().max().unwrap_or_default();
        ensure!(max_depth < 5, "number is not reduced");
        Ok(number)
    }

    fn to_number_byte(x: u8) -> Result<u8> {
        match x {
            b'0'..=b'9' => Ok(x - b'0'),
            b'[' => Ok(BR_LEFT),
            b',' => Ok(COMMA),
            b']' => Ok(BR_RIGHT),
            _ => bail!("invalid byte: {x}"),
        }
    }

    fn depth_iter(&self) -> impl Iterator<Item = usize> + '_ {
        self.0.iter().scan(0, |depth, &x| {
            match x {
                BR_LEFT => *depth += 1,
                BR_RIGHT => *depth -= 1,
                _ => (),
            }
            Some(*depth)
        })
    }

    fn explode(&mut self) -> Result<bool> {
        let Some(start) = self.depth_iter().position(|depth| depth == 5) else {
            return Ok(false);
        };

        let &[BR_LEFT, x1 @ 0..=MAX_VALUE, COMMA, x2 @ 0..=MAX_VALUE, BR_RIGHT] = &self.0[start..start + 5] else { bail!("invalid number") };

        if let Some(v) = self.0[..start].iter_mut().rev().find(|x| **x <= MAX_VALUE) {
            *v += x1;
        }

        if let Some(v) = self.0[start + 5..].iter_mut().find(|x| **x <= MAX_VALUE) {
            *v += x2;
        }

        self.0.splice(start..start + 5, once(0));
        Ok(true)
    }

    fn split(&mut self) -> bool {
        match self.0.iter().position(|&x| (10..=MAX_VALUE).contains(&x)) {
            None => false,
            Some(index) => {
                let value = self.0[index];
                let left = value / 2;
                let right = left + value % 2;

                let len = self.0.len();
                self.0.extend_from_slice(&[0; 4]);
                self.0.copy_within(index + 1..len, index + 5);
                self.0[index..index + 5].copy_from_slice(&[BR_LEFT, left, COMMA, right, BR_RIGHT]);
                true
            }
        }
    }

    fn reduce(&mut self) -> Result<()> {
        loop {
            if self.explode()? {
                continue;
            }
            if self.split() {
                continue;
            }
            break Ok(());
        }
    }

    fn add_n<'a>(&self, iter: impl IntoIterator<Item = &'a Self>) -> Result<Self> {
        let mut lhs = self.clone();
        for rhs in iter {
            lhs.0.reserve(rhs.0.len() + 3);
            lhs.0.insert(0, BR_LEFT);
            lhs.0.push(COMMA);
            lhs.0.extend_from_slice(&rhs.0);
            lhs.0.push(BR_RIGHT);
            lhs.reduce()?;
        }
        Ok(lhs)
    }

    fn magnitude(&self) -> Result<i64> {
        let mut stack = SmallVec::<[SmallVec<[i64; 2]>; 4]>::new();

        for &x in &self.0 {
            match x {
                COMMA => (),
                BR_LEFT => stack.push(SmallVec::new()),
                BR_RIGHT => {
                    let value = match *stack.pop().value()?.as_slice() {
                        [x] => x,
                        [x1, x2] => 3 * x1 + 2 * x2,
                        _ => bail!("invalid pair"),
                    };

                    match stack.last_mut() {
                        None => return Ok(value),
                        Some(last) => last.push(value),
                    }
                }
                value => stack.last_mut().value()?.push(value as i64),
            }
        }

        bail!("invalid number");
    }
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let numbers: Vec<_> = input.lines().map(Number::parse).try_collect()?;

    let mut iter = numbers.iter();
    let result1 = iter.next().value()?.add_n(iter)?.magnitude()?;

    let result2 = numbers
        .iter()
        .tuple_combinations()
        .flat_map(|(x, y)| [(x, y), (y, x)])
        .map(|(x, y)| x.add_n([y])?.magnitude())
        .try_process(|iter| iter.max())?
        .value()?;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
