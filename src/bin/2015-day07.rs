use eyre::Result;
use regex::Regex;

use std::cell::Cell;
use std::collections::HashMap;
use std::fs;

struct GlobalMap<'a>(HashMap<&'a str, Variable<'a>>);

impl<'a> GlobalMap<'a> {
    fn new() -> Self {
        GlobalMap(HashMap::new())
    }

    fn value(&self, variable: &str) -> u16 {
        self.0[variable].value(self)
    }

    fn clear(&self) {
        for variable in self.0.values() {
            variable.clear();
        }
    }
}

enum Operand<'a> {
    Constant(u16),
    Variable(&'a str),
}

impl<'a> Operand<'a> {
    fn parse_new(op: &'a str) -> Self {
        match op.parse::<u16>() {
            Ok(val) => Operand::Constant(val),
            Err(_) => Operand::Variable(op),
        }
    }

    fn value(&self, map: &GlobalMap) -> u16 {
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
    value: Cell<Option<u16>>,
}

impl<'a> Variable<'a> {
    fn new(operation: Operation<'a>) -> Self {
        Variable { operation, value: Cell::new(None) }
    }

    fn value(&self, map: &GlobalMap) -> u16 {
        self.value.get().unwrap_or_else(|| {
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
        })
    }

    fn clear(&self) {
        self.value.set(None);
    }
}

struct ParseRegex {
    regex_identity: Regex,
    regex_and: Regex,
    regex_or: Regex,
    regex_not: Regex,
    regex_lshift: Regex,
    regex_rshift: Regex,
}

impl ParseRegex {
    fn parse<'a>(&self, map: &mut GlobalMap<'a>, line: &'a str) {
        if let Some(cap) = self.regex_identity.captures(line) {
            let name = cap.name("name").unwrap().as_str();
            let op = cap.name("op").unwrap().as_str();
            map.0.insert(name, Variable::new(Operation::Identity(Operand::parse_new(op))));
        } else if let Some(cap) = self.regex_and.captures(line) {
            let name = cap.name("name").unwrap().as_str();
            let op1 = cap.name("op1").unwrap().as_str();
            let op2 = cap.name("op2").unwrap().as_str();
            map.0.insert(name, Variable::new(Operation::And(Operand::parse_new(op1), Operand::parse_new(op2))));
        } else if let Some(cap) = self.regex_or.captures(line) {
            let name = cap.name("name").unwrap().as_str();
            let op1 = cap.name("op1").unwrap().as_str();
            let op2 = cap.name("op2").unwrap().as_str();
            map.0.insert(name, Variable::new(Operation::Or(Operand::parse_new(op1), Operand::parse_new(op2))));
        } else if let Some(cap) = self.regex_not.captures(line) {
            let name = cap.name("name").unwrap().as_str();
            let op = cap.name("op").unwrap().as_str();
            map.0.insert(name, Variable::new(Operation::Not(Operand::parse_new(op))));
        } else if let Some(cap) = self.regex_lshift.captures(line) {
            let name = cap.name("name").unwrap().as_str();
            let op1 = cap.name("op1").unwrap().as_str();
            let op2 = cap.name("op2").unwrap().as_str();
            map.0.insert(name, Variable::new(Operation::LShift(Operand::parse_new(op1), Operand::parse_new(op2))));
        } else if let Some(cap) = self.regex_rshift.captures(line) {
            let name = cap.name("name").unwrap().as_str();
            let op1 = cap.name("op1").unwrap().as_str();
            let op2 = cap.name("op2").unwrap().as_str();
            map.0.insert(name, Variable::new(Operation::RShift(Operand::parse_new(op1), Operand::parse_new(op2))));
        } else {
            panic!("unknown instruction: {}", line);
        }
    }
}

fn main() -> Result<()> {
    let input = fs::read_to_string("inputs/2015-day07.txt")?;

    let parse_regex = ParseRegex {
        regex_identity: Regex::new(r#"^(?P<op>\w+) -> (?P<name>\w+)$"#)?,
        regex_and: Regex::new(r#"^(?P<op1>\w+) AND (?P<op2>\w+) -> (?P<name>\w+)$"#)?,
        regex_or: Regex::new(r#"^(?P<op1>\w+) OR (?P<op2>\w+) -> (?P<name>\w+)$"#)?,
        regex_not: Regex::new(r#"^NOT (?P<op>\w+) -> (?P<name>\w+)$"#)?,
        regex_lshift: Regex::new(r#"^(?P<op1>\w+) LSHIFT (?P<op2>\w+) -> (?P<name>\w+)$"#)?,
        regex_rshift: Regex::new(r#"^(?P<op1>\w+) RSHIFT (?P<op2>\w+) -> (?P<name>\w+)$"#)?,
    };

    let mut map = GlobalMap::new();
    for line in input.lines() {
        parse_regex.parse(&mut map, line);
    }
    let result1 = map.value("a");

    map.clear();
    let new_line = format!("{} -> b", result1);
    parse_regex.parse(&mut map, &new_line);
    let result2 = map.value("a");

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
