use aoc::*;

use eyre::bail;
use itertools::Itertools;

use std::collections::{HashMap, HashSet, VecDeque};

enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

enum Monkey<'a> {
    Constant(i64),
    Calculated(Operator, &'a str, &'a str),
}

impl<'a> Monkey<'a> {
    fn value(&self, values: &HashMap<&str, i64>) -> i64 {
        match *self {
            Self::Constant(n) => n,
            Self::Calculated(Operator::Add, op1, op2) => values[op1] + values[op2],
            Self::Calculated(Operator::Sub, op1, op2) => values[op1] - values[op2],
            Self::Calculated(Operator::Mul, op1, op2) => values[op1] * values[op2],
            Self::Calculated(Operator::Div, op1, op2) => values[op1] / values[op2],
        }
    }

    fn is_calculable(&self, values: &HashMap<&str, i64>) -> bool {
        match *self {
            Self::Constant(_) => true,
            Self::Calculated(_, op1, op2) => values.contains_key(op1) && values.contains_key(op2),
        }
    }
}

fn compute_root_value(monkeys: &HashMap<&str, Monkey>, inverted_graph: &HashMap<&str, Vec<&str>>) -> i64 {
    let mut values = HashMap::new();
    let mut queue: VecDeque<_> = monkeys.iter().filter(|&(_, monkey)| matches!(monkey, Monkey::Constant(_))).map(|(&name, _)| name).collect();

    while let Some(name) = queue.pop_front() {
        values.insert(name, monkeys[name].value(&values));
        queue.extend(inverted_graph[name].iter().copied().filter(|&x| monkeys[x].is_calculable(&values)));
    }

    values["root"]
}

fn compute_human_value(monkeys: &HashMap<&str, Monkey>, inverted_graph: &HashMap<&str, Vec<&str>>) -> Result<i64> {
    let mut values = HashMap::new();

    let mut human_dependencies = HashSet::new();
    let mut queue: VecDeque<_> = inverted_graph["humn"].iter().copied().collect();

    while let Some(name) = queue.pop_front() {
        human_dependencies.insert(name);
        queue.extend(inverted_graph[name].iter().copied());
    }

    queue.clear();
    queue.extend(monkeys.iter().filter(|&(&name, monkey)| name != "humn" && matches!(monkey, Monkey::Constant(_))).map(|(&name, _)| name));

    while let Some(name) = queue.pop_front() {
        values.insert(name, monkeys[name].value(&values));
        queue.extend(inverted_graph[name].iter().copied().filter(|&x| monkeys[x].is_calculable(&values) && !human_dependencies.contains(name)));
    }

    let (mut current_unknown, mut current_value) = match monkeys["root"] {
        Monkey::Constant(_) => bail!("should not be a constant"),
        Monkey::Calculated(_, op1, op2) => match (values.get(op1), values.get(op2)) {
            (Some(&v1), None) => (op2, v1),
            (None, Some(&v2)) => (op1, v2),
            _ => bail!("values are incorrect"),
        },
    };

    loop {
        match monkeys[current_unknown] {
            Monkey::Constant(_) => bail!("should not be a constant"),
            Monkey::Calculated(ref op, op1, op2) => match (values.get(op1), values.get(op2)) {
                (Some(&v1), None) => {
                    current_unknown = op2;
                    current_value = match op {
                        Operator::Add => current_value - v1,
                        Operator::Sub => v1 - current_value,
                        Operator::Mul => current_value / v1,
                        Operator::Div => v1 / current_value,
                    };
                }
                (None, Some(&v2)) => {
                    current_unknown = op1;
                    current_value = match op {
                        Operator::Add => current_value - v2,
                        Operator::Sub => current_value + v2,
                        Operator::Mul => current_value / v2,
                        Operator::Div => current_value * v2,
                    };
                }
                _ => bail!("values are incorrect"),
            },
        };

        if current_unknown == "humn" {
            break Ok(current_value);
        }
    }
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let mut inverted_graph = HashMap::<_, Vec<_>>::new();

    let monkeys: HashMap<_, _> = input
        .lines()
        .map(|line| {
            let (name, operation) = line.split(": ").next_tuple().value()?;

            let monkey = match operation.parse::<i64>() {
                Ok(n) => Monkey::Constant(n),
                Err(_) => {
                    let (op1, op, op2) = operation.split_ascii_whitespace().next_tuple().value()?;
                    inverted_graph.entry(name).or_default();
                    inverted_graph.entry(op1).or_default().push(name);
                    inverted_graph.entry(op2).or_default().push(name);

                    match op {
                        "+" => Monkey::Calculated(Operator::Add, op1, op2),
                        "-" => Monkey::Calculated(Operator::Sub, op1, op2),
                        "*" => Monkey::Calculated(Operator::Mul, op1, op2),
                        "/" => Monkey::Calculated(Operator::Div, op1, op2),
                        _ => bail!("unknown operation: {op}"),
                    }
                }
            };

            Ok((name, monkey))
        })
        .try_collect()?;

    let result1 = compute_root_value(&monkeys, &inverted_graph);
    let result2 = compute_human_value(&monkeys, &inverted_graph)?;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
