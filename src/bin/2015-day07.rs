use aoc::*;

use eyre::bail;
use regex::{Regex, RegexSet};

use std::cell::Cell;
use std::collections::HashMap;

struct GlobalMap<'a>(HashMap<&'a str, Variable<'a>>);

impl<'a> GlobalMap<'a> {
    fn new() -> Self {
        GlobalMap(HashMap::new())
    }

    fn value(&self, variable: &str) -> u64 {
        self.0[variable].value(self)
    }

    fn clear(&self) {
        for variable in self.0.values() {
            variable.clear();
        }
    }
}

enum Operand<'a> {
    Constant(u64),
    Variable(&'a str),
}

impl<'a> Operand<'a> {
    fn parse_new(op: &'a str) -> Self {
        match op.parse::<u64>() {
            Ok(val) => Operand::Constant(val),
            Err(_) => Operand::Variable(op),
        }
    }

    fn value(&self, map: &GlobalMap) -> u64 {
        match self {
            &Operand::Constant(x) => x,
            Operand::Variable(variable) => map.value(variable),
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

struct Variable<'a> {
    operation: Operation<'a>,
    value: Cell<Option<u64>>,
}

impl<'a> Variable<'a> {
    fn new(operation: Operation<'a>) -> Self {
        Variable { operation, value: Cell::new(None) }
    }

    fn value(&self, map: &GlobalMap) -> u64 {
        match self.value.get() {
            Some(value) => value,
            None => {
                let value = match &self.operation {
                    Operation::Identity(op) => op.value(map),
                    Operation::And(op1, op2) => op1.value(map) & op2.value(map),
                    Operation::Or(op1, op2) => op1.value(map) | op2.value(map),
                    Operation::Not(op) => !op.value(map),
                    Operation::LShift(op1, op2) => op1.value(map) << op2.value(map),
                    Operation::RShift(op1, op2) => op1.value(map) >> op2.value(map),
                };
                self.value.set(Some(value));
                value
            }
        }
    }

    fn clear(&self) {
        self.value.set(None);
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

    fn new(regex_identity: Regex, regex_and: Regex, regex_or: Regex, regex_not: Regex, regex_lshift: Regex, regex_rshift: Regex) -> Result<Self> {
        Ok(Self {
            set: RegexSet::new(&[
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

    fn parse<'a>(&self, map: &mut GlobalMap<'a>, line: &'a str) -> Result<()> {
        match self.set.matches(line).iter().next() {
            Some(Self::REGEX_IDENTITY) => {
                let cap = self.regex_identity.captures(line).value()?;
                let name = cap.name("name").value()?.as_str();
                let op = cap.name("op").value()?.as_str();
                map.0.insert(name, Variable::new(Operation::Identity(Operand::parse_new(op))));
            }
            Some(Self::REGEX_AND) => {
                let cap = self.regex_and.captures(line).value()?;
                let name = cap.name("name").value()?.as_str();
                let op1 = cap.name("op1").value()?.as_str();
                let op2 = cap.name("op2").value()?.as_str();
                map.0.insert(name, Variable::new(Operation::And(Operand::parse_new(op1), Operand::parse_new(op2))));
            }
            Some(Self::REGEX_OR) => {
                let cap = self.regex_or.captures(line).value()?;
                let name = cap.name("name").value()?.as_str();
                let op1 = cap.name("op1").value()?.as_str();
                let op2 = cap.name("op2").value()?.as_str();
                map.0.insert(name, Variable::new(Operation::Or(Operand::parse_new(op1), Operand::parse_new(op2))));
            }
            Some(Self::REGEX_NOT) => {
                let cap = self.regex_not.captures(line).value()?;
                let name = cap.name("name").value()?.as_str();
                let op = cap.name("op").value()?.as_str();
                map.0.insert(name, Variable::new(Operation::Not(Operand::parse_new(op))));
            }
            Some(Self::REGEX_LSHIFT) => {
                let cap = self.regex_lshift.captures(line).value()?;
                let name = cap.name("name").value()?.as_str();
                let op1 = cap.name("op1").value()?.as_str();
                let op2 = cap.name("op2").value()?.as_str();
                map.0.insert(name, Variable::new(Operation::LShift(Operand::parse_new(op1), Operand::parse_new(op2))));
            }
            Some(Self::REGEX_RSHIFT) => {
                let cap = self.regex_rshift.captures(line).value()?;
                let name = cap.name("name").value()?.as_str();
                let op1 = cap.name("op1").value()?.as_str();
                let op2 = cap.name("op2").value()?.as_str();
                map.0.insert(name, Variable::new(Operation::RShift(Operand::parse_new(op1), Operand::parse_new(op2))));
            }
            _ => bail!("unknown instruction: {line}"),
        };

        Ok(())
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

    let mut map = GlobalMap::new();
    for line in input.lines() {
        parse_regex.parse(&mut map, line)?;
    }
    let result1 = map.value("a");

    map.clear();
    let new_line = format!("{result1} -> b");
    parse_regex.parse(&mut map, &new_line)?;
    let result2 = map.value("a");

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
