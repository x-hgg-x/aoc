use aoc::*;

use eyre::bail;
use itertools::Itertools;
use regex::Regex;

#[derive(Copy, Clone)]
enum Operand {
    Constant(u64),
    Old,
}

impl Operand {
    fn parse(s: &str) -> Result<Self> {
        match s {
            "old" => Ok(Self::Old),
            _ => Ok(Self::Constant(s.parse()?)),
        }
    }

    fn value(&self, old: u64) -> u64 {
        match *self {
            Operand::Constant(x) => x,
            Operand::Old => old,
        }
    }
}

#[derive(Copy, Clone)]
enum Operation {
    Add(Operand, Operand),
    Mul(Operand, Operand),
}

impl Operation {
    fn parse(op1: &str, op: &str, op2: &str) -> Result<Self> {
        match op {
            "+" => Ok(Self::Add(Operand::parse(op1)?, Operand::parse(op2)?)),
            "*" => Ok(Self::Mul(Operand::parse(op1)?, Operand::parse(op2)?)),
            _ => bail!("unknown operation: {op}"),
        }
    }

    fn value(&self, old: u64) -> u64 {
        match self {
            Operation::Add(x, y) => x.value(old) + y.value(old),
            Operation::Mul(x, y) => x.value(old) * y.value(old),
        }
    }
}

#[derive(Clone)]
struct Monkey {
    inspected: u64,
    items: Vec<u64>,
    operation: Operation,
    divisor: u64,
    true_index: usize,
    false_index: usize,
}

fn run(mut monkeys: Vec<Monkey>, buffer: &mut Vec<u64>, divisor_product: u64, steps: usize, relief_factor: u64) -> u64 {
    for _ in 0..steps {
        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i];
            let Monkey { operation, divisor, true_index, false_index, .. } = *monkey;

            buffer.clear();
            buffer.extend_from_slice(&monkey.items);
            monkey.inspected += monkey.items.len() as u64;
            monkey.items.clear();

            let mut true_items = std::mem::take(&mut monkeys[true_index].items);
            let mut false_items = std::mem::take(&mut monkeys[false_index].items);

            for &item in &*buffer {
                let item = operation.value(item % divisor_product) / relief_factor;

                if item % divisor == 0 {
                    true_items.push(item);
                } else {
                    false_items.push(item);
                }
            }

            monkeys[true_index].items = true_items;
            monkeys[false_index].items = false_items;
        }
    }

    monkeys.iter().map(|x| x.inspected).sorted_unstable().collect_vec().iter().rev().take(2).product()
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let re = Regex::new(concat!(
        r#"(?m)Monkey \d+:\s+"#,
        r#"Starting items: (?P<items>\d+(?:, \d+)*)\s+"#,
        r#"Operation: new = (?P<op1>\d+|old) (?P<op>[+*]) (?P<op2>\d+|old)\s+"#,
        r#"Test: divisible by (?P<divisor>\d+)\s+"#,
        r#"If true: throw to monkey (?P<true>\d+)\s+"#,
        r#"If false: throw to monkey (?P<false>\d+)"#,
    ))?;

    let monkeys: Vec<_> = re
        .captures_iter(&input)
        .map(|cap| {
            Result::Ok(Monkey {
                inspected: 0,
                items: cap["items"].split(", ").map(|x| x.parse()).try_collect()?,
                operation: Operation::parse(&cap["op1"], &cap["op"], &cap["op2"])?,
                divisor: cap["divisor"].parse()?,
                true_index: cap["true"].parse()?,
                false_index: cap["false"].parse()?,
            })
        })
        .try_collect()?;

    let divisor_product = monkeys.iter().map(|x| x.divisor).product::<u64>();

    let mut buffer = Vec::new();

    let result1 = run(monkeys.clone(), &mut buffer, divisor_product, 20, 3);
    let result2 = run(monkeys, &mut buffer, divisor_product, 10000, 1);

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
