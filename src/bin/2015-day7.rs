use regex::Regex;

use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;

struct GlobalMap(HashMap<String, Variable>);

impl GlobalMap {
    fn new() -> Self {
        GlobalMap(HashMap::new())
    }

    fn value(&self, variable: &str) -> u16 {
        self.0[variable].value(&self)
    }

    fn clear(&self) {
        for variable in self.0.values() {
            variable.clear();
        }
    }
}

enum Operand {
    Constant(u16),
    Variable(String),
}

impl Operand {
    fn parse_new(op: &str) -> Self {
        match op.parse::<u16>() {
            Ok(val) => Operand::Constant(val),
            Err(_) => Operand::Variable(op.to_owned()),
        }
    }

    fn value(&self, map: &GlobalMap) -> u16 {
        match self {
            &Operand::Constant(x) => x,
            Operand::Variable(variable) => map.value(variable),
        }
    }
}

enum Operation {
    Identity(Operand),
    And(Operand, Operand),
    Or(Operand, Operand),
    Not(Operand),
    LShift(Operand, Operand),
    RShift(Operand, Operand),
}

struct Variable {
    operation: Operation,
    value: RefCell<Option<u16>>,
}

impl Variable {
    fn new(operation: Operation) -> Self {
        Variable {
            operation,
            value: RefCell::new(None),
        }
    }

    fn value(&self, map: &GlobalMap) -> u16 {
        if self.value.borrow().is_none() {
            *self.value.borrow_mut() = Some(match &self.operation {
                Operation::Identity(op) => op.value(map),
                Operation::And(op1, op2) => op1.value(map) & op2.value(map),
                Operation::Or(op1, op2) => op1.value(map) | op2.value(map),
                Operation::Not(op) => op.value(map) ^ 0xFFFF,
                Operation::LShift(op1, op2) => op1.value(map) << op2.value(map),
                Operation::RShift(op1, op2) => op1.value(map) >> op2.value(map),
            });
        }
        return self.value.borrow().unwrap();
    }

    fn clear(&self) {
        if self.value.borrow().is_some() {
            *self.value.borrow_mut() = None;
        }
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
    fn parse(&self, map: &mut GlobalMap, line: &str) {
        if let Some(cap) = self.regex_identity.captures(line) {
            map.0.insert(
                cap["name"].to_owned(),
                Variable::new(Operation::Identity(Operand::parse_new(&cap["op"]))),
            );
        } else if let Some(cap) = self.regex_and.captures(line) {
            map.0.insert(
                cap["name"].to_owned(),
                Variable::new(Operation::And(
                    Operand::parse_new(&cap["op1"]),
                    Operand::parse_new(&cap["op2"]),
                )),
            );
        } else if let Some(cap) = self.regex_or.captures(line) {
            map.0.insert(
                cap["name"].to_owned(),
                Variable::new(Operation::Or(
                    Operand::parse_new(&cap["op1"]),
                    Operand::parse_new(&cap["op2"]),
                )),
            );
        } else if let Some(cap) = self.regex_not.captures(line) {
            map.0.insert(
                cap["name"].to_owned(),
                Variable::new(Operation::Not(Operand::parse_new(&cap["op"]))),
            );
        } else if let Some(cap) = self.regex_lshift.captures(line) {
            map.0.insert(
                cap["name"].to_owned(),
                Variable::new(Operation::LShift(
                    Operand::parse_new(&cap["op1"]),
                    Operand::parse_new(&cap["op2"]),
                )),
            );
        } else if let Some(cap) = self.regex_rshift.captures(line) {
            map.0.insert(
                cap["name"].to_owned(),
                Variable::new(Operation::RShift(
                    Operand::parse_new(&cap["op1"]),
                    Operand::parse_new(&cap["op2"]),
                )),
            );
        } else {
            panic!("unknown instruction: {}", line);
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("inputs/2015-day7.txt")?;

    let parse_regex = ParseRegex {
        regex_identity: Regex::new(r#"^(?P<op>\w+) -> (?P<name>\w+)$"#).unwrap(),
        regex_and: Regex::new(r#"^(?P<op1>\w+) AND (?P<op2>\w+) -> (?P<name>\w+)$"#).unwrap(),
        regex_or: Regex::new(r#"^(?P<op1>\w+) OR (?P<op2>\w+) -> (?P<name>\w+)$"#).unwrap(),
        regex_not: Regex::new(r#"^NOT (?P<op>\w+) -> (?P<name>\w+)$"#).unwrap(),
        regex_lshift: Regex::new(r#"^(?P<op1>\w+) LSHIFT (?P<op2>\w+) -> (?P<name>\w+)$"#).unwrap(),
        regex_rshift: Regex::new(r#"^(?P<op1>\w+) RSHIFT (?P<op2>\w+) -> (?P<name>\w+)$"#).unwrap(),
    };

    let mut map = GlobalMap::new();
    for line in input.lines() {
        parse_regex.parse(&mut map, line);
    }

    let result1 = map.value("a");

    map.clear();
    parse_regex.parse(&mut map, &format!("{} -> b", result1));
    let result2 = map.value("a");

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
