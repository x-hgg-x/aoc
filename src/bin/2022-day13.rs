use aoc::*;

use eyre::{bail, ensure};
use itertools::Itertools;

use std::cmp::Ordering;
use std::iter;

#[derive(Clone)]
enum Item {
    Number(u64),
    List(Vec<Item>),
}

impl Eq for Item {}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Number(x), Self::Number(y)) => x.cmp(y),
            (&Self::Number(x), Self::List(l)) => cmp_slice(&[Self::Number(x)], l),
            (Self::List(l), &Self::Number(x)) => cmp_slice(l, &[Self::Number(x)]),
            (Self::List(left), Self::List(right)) => cmp_slice(left, right),
        }
    }
}

fn cmp_slice(left: &[Item], right: &[Item]) -> Ordering {
    iter::zip(left, right)
        .map(|(l, r)| l.cmp(r))
        .find(|&ord| ord != Ordering::Equal)
        .unwrap_or_else(|| left.len().cmp(&right.len()))
}

impl Item {
    fn parse(remaining: &mut &[u8]) -> Result<Self> {
        match *remaining {
            [b'[', tail @ ..] => {
                *remaining = tail;

                let mut items = Vec::new();

                if remaining.first() != Some(&b']') {
                    items.push(Item::parse(remaining)?);

                    while let [b',', tail @ ..] = *remaining {
                        *remaining = tail;
                        items.push(Item::parse(remaining)?);
                    }
                }

                match *remaining {
                    [b']', tail @ ..] => *remaining = tail,
                    _ => bail!("invalid input"),
                }

                Ok(Self::List(items))
            }
            _ => {
                let mut number = 0;
                let mut iter = remaining.iter();

                (iter.take_while_ref(|x| x.is_ascii_digit())).for_each(|digit| {
                    number *= 10;
                    number += (digit - b'0') as u64;
                });

                let after = iter.as_slice();
                ensure!(remaining.len() != after.len(), "invalid input");
                *remaining = after;

                Ok(Self::Number(number))
            }
        }
    }
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let mut packets: Vec<_> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| Item::parse(&mut line.as_bytes()))
        .try_collect()?;

    let result1 = packets
        .chunks_exact(2)
        .enumerate()
        .filter(|(_, x)| x[0] < x[1])
        .map(|(index, _)| index + 1)
        .sum::<usize>();

    let d2 = Item::List(vec![Item::List(vec![Item::Number(2)])]);
    let d6 = Item::List(vec![Item::List(vec![Item::Number(6)])]);

    let dividers = [d2, d6];
    packets.extend_from_slice(&dividers);

    let result2 = dividers
        .iter()
        .map(|d| packets.iter().filter(|&x| x <= d).count())
        .product::<usize>();

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
