use aoc::*;

use eyre::bail;
use regex::{Regex, RegexSet};
use smallvec::SmallVec;

use std::collections::{HashMap, VecDeque};

enum Operand<'a> {
    Constant(u64),
    Variable(&'a str),
}

impl<'a> Operand<'a> {
    fn parse_new(op: &'a str) -> Self {
        match op.parse() {
            Ok(val) => Operand::Constant(val),
            Err(_) => Operand::Variable(op),
        }
    }

    fn dependency(&self) -> Option<&'a str> {
        match *self {
            Operand::Variable(x) => Some(x),
            _ => None,
        }
    }

    fn value(&self, values: &HashMap<&str, u64>) -> u64 {
        match *self {
            Operand::Constant(x) => x,
            Operand::Variable(x) => values[x],
        }
    }
}

enum Operation<'a> {
    Identity(Operand<'a>),
    And(Operand<'a>, Operand<'a>),
    Or(Operand<'a>, Operand<'a>),
    Not(Operand<'a>),
    LShift(Operand<'a>, Operand<'a>),
    RShift(Operand<'a>, Operand<'a>),
}

impl<'a> Operation<'a> {
    fn value(&self, values: &HashMap<&str, u64>) -> u64 {
        match self {
            Operation::Identity(op) => op.value(values),
            Operation::And(op1, op2) => op1.value(values) & op2.value(values),
            Operation::Or(op1, op2) => op1.value(values) | op2.value(values),
            Operation::Not(op) => !op.value(values),
            Operation::LShift(op1, op2) => op1.value(values) << op2.value(values),
            Operation::RShift(op1, op2) => op1.value(values) >> op2.value(values),
        }
    }
}

struct ParseRegex {
    set: RegexSet,
    regex_identity: Regex,
    regex_and: Regex,
    regex_or: Regex,
    regex_not: Regex,
    regex_lshift: Regex,
    regex_rshift: Regex,
}

impl ParseRegex {
    const REGEX_IDENTITY: usize = 0;
    const REGEX_AND: usize = 1;
    const REGEX_OR: usize = 2;
    const REGEX_NOT: usize = 3;
    const REGEX_LSHIFT: usize = 4;
    const REGEX_RSHIFT: usize = 5;

    fn new(
        regex_identity: Regex,
        regex_and: Regex,
        regex_or: Regex,
        regex_not: Regex,
        regex_lshift: Regex,
        regex_rshift: Regex,
    ) -> Result<Self> {
        Ok(Self {
            set: RegexSet::new([
                regex_identity.as_str(),
                regex_and.as_str(),
                regex_or.as_str(),
                regex_not.as_str(),
                regex_lshift.as_str(),
                regex_rshift.as_str(),
            ])?,
            regex_identity,
            regex_and,
            regex_or,
            regex_not,
            regex_lshift,
            regex_rshift,
        })
    }

    fn parse<'a>(&self, line: &'a str) -> Result<(&'a str, SmallVec<[&'a str; 2]>, Operation<'a>)> {
        match self.set.matches(line).iter().next() {
            Some(Self::REGEX_IDENTITY) => {
                let cap = self.regex_identity.captures(line).value()?;
                let name = cap.name("name").value()?.as_str();
                let op = Operand::parse_new(cap.name("op").value()?.as_str());
                let dependencies = [op.dependency()].into_iter().flatten().collect();
                Ok((name, dependencies, Operation::Identity(op)))
            }
            Some(Self::REGEX_AND) => {
                let cap = self.regex_and.captures(line).value()?;
                let name = cap.name("name").value()?.as_str();
                let op1 = Operand::parse_new(cap.name("op1").value()?.as_str());
                let op2 = Operand::parse_new(cap.name("op2").value()?.as_str());

                let dependencies = [op1.dependency(), op2.dependency()]
                    .into_iter()
                    .flatten()
                    .collect();

                Ok((name, dependencies, Operation::And(op1, op2)))
            }
            Some(Self::REGEX_OR) => {
                let cap = self.regex_or.captures(line).value()?;
                let name = cap.name("name").value()?.as_str();
                let op1 = Operand::parse_new(cap.name("op1").value()?.as_str());
                let op2 = Operand::parse_new(cap.name("op2").value()?.as_str());

                let dependencies = [op1.dependency(), op2.dependency()]
                    .into_iter()
                    .flatten()
                    .collect();

                Ok((name, dependencies, Operation::Or(op1, op2)))
            }
            Some(Self::REGEX_NOT) => {
                let cap = self.regex_not.captures(line).value()?;
                let name = cap.name("name").value()?.as_str();
                let op = Operand::parse_new(cap.name("op").value()?.as_str());
                let dependencies = op.dependency().into_iter().collect();
                Ok((name, dependencies, Operation::Not(op)))
            }
            Some(Self::REGEX_LSHIFT) => {
                let cap = self.regex_lshift.captures(line).value()?;
                let name = cap.name("name").value()?.as_str();
                let op1 = Operand::parse_new(cap.name("op1").value()?.as_str());
                let op2 = Operand::parse_new(cap.name("op2").value()?.as_str());

                let dependencies = [op1.dependency(), op2.dependency()]
                    .into_iter()
                    .flatten()
                    .collect();

                Ok((name, dependencies, Operation::LShift(op1, op2)))
            }
            Some(Self::REGEX_RSHIFT) => {
                let cap = self.regex_rshift.captures(line).value()?;
                let name = cap.name("name").value()?.as_str();
                let op1 = Operand::parse_new(cap.name("op1").value()?.as_str());
                let op2 = Operand::parse_new(cap.name("op2").value()?.as_str());

                let dependencies = [op1.dependency(), op2.dependency()]
                    .into_iter()
                    .flatten()
                    .collect();

                Ok((name, dependencies, Operation::RShift(op1, op2)))
            }
            _ => bail!("unknown instruction: {line}"),
        }
    }
}

fn compute_values<'a>(
    values: &mut HashMap<&'a str, u64>,
    graph: &HashMap<&'a str, (Operation<'a>, SmallVec<[&'a str; 2]>)>,
    inverted_graph: &HashMap<&'a str, Vec<&'a str>>,
) {
    let mut queue: VecDeque<_> = graph
        .iter()
        .filter(|&(_, (_, dependencies))| dependencies.is_empty())
        .map(|(&name, _)| name)
        .collect();

    while let Some(name) = queue.pop_front() {
        values.insert(name, graph[name].0.value(values));
        queue.extend(inverted_graph[name].iter().copied().filter(|&x| {
            (graph[x].1.iter()).all(|&dependencies| values.contains_key(dependencies))
        }));
    }
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let parse_regex = ParseRegex::new(
        Regex::new(r#"^(?P<op>\w+) -> (?P<name>\w+)$"#)?,
        Regex::new(r#"^(?P<op1>\w+) AND (?P<op2>\w+) -> (?P<name>\w+)$"#)?,
        Regex::new(r#"^(?P<op1>\w+) OR (?P<op2>\w+) -> (?P<name>\w+)$"#)?,
        Regex::new(r#"^NOT (?P<op>\w+) -> (?P<name>\w+)$"#)?,
        Regex::new(r#"^(?P<op1>\w+) LSHIFT (?P<op2>\w+) -> (?P<name>\w+)$"#)?,
        Regex::new(r#"^(?P<op1>\w+) RSHIFT (?P<op2>\w+) -> (?P<name>\w+)$"#)?,
    )?;

    let mut graph = HashMap::new();
    let mut inverted_graph = HashMap::<_, Vec<_>>::new();

    for line in input.lines() {
        let (name, dependencies, op) = parse_regex.parse(line)?;

        graph.insert(name, (op, dependencies.clone()));
        inverted_graph.entry(name).or_default();

        for dependency in dependencies {
            inverted_graph.entry(dependency).or_default().push(name);
        }
    }

    let mut values = HashMap::new();
    compute_values(&mut values, &graph, &inverted_graph);
    let result1 = values["a"];

    values.clear();
    for &dependency in &graph["b"].1 {
        let parent = inverted_graph.entry(dependency).or_default();
        if let Some(position) = parent.iter().position(|&x| x == "b") {
            parent.remove(position);
        }
    }
    let op = Operation::Identity(Operand::Constant(result1));
    graph.insert("b", (op, SmallVec::new()));

    compute_values(&mut values, &graph, &inverted_graph);
    let result2 = values["a"];

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
