use eyre::Result;
use itertools::Itertools;
use regex::{Match, Regex};

use std::collections::HashMap;
use std::fs;

struct Rule<'a> {
    new_value: u8,
    direction: i64,
    new_state: &'a str,
}

fn parse_rule<'a>(match_new_value: Match<'a>, match_direction: Match<'a>, match_new_state: Match<'a>) -> Rule<'a> {
    Rule {
        new_value: match_new_value.as_str().parse().unwrap(),
        direction: match match_direction.as_str() {
            "left" => -1,
            "right" => 1,
            _ => panic!("unknown offset"),
        },
        new_state: match_new_state.as_str(),
    }
}

fn main() -> Result<()> {
    let input = fs::read_to_string("inputs/2017-day25.txt")?;

    let regex_begin = Regex::new(r#"Begin in state (\w+)\."#)?;
    let regex_steps = Regex::new(r#"Perform a diagnostic checksum after (\d+) steps\."#)?;

    let regex_rules = Regex::new(concat!(
        r#"(?s)In state (\w+):\s+"#,
        r#"If the current value is 0:.+?Write the value ([01])\..+?Move one slot to the (left|right)\..+?Continue with state (\w+)\..+?"#,
        r#"If the current value is 1:.+?Write the value ([01])\..+?Move one slot to the (left|right)\..+?Continue with state (\w+)\."#,
    ))?;

    let rules: HashMap<_, _> = regex_rules
        .captures_iter(&input)
        .map(|cap| {
            let current_state = cap.get(1).unwrap().as_str();

            let state_rules = [
                parse_rule(cap.get(2).unwrap(), cap.get(3).unwrap(), cap.get(4).unwrap()),
                parse_rule(cap.get(5).unwrap(), cap.get(6).unwrap(), cap.get(7).unwrap()),
            ];

            (current_state, state_rules)
        })
        .collect();

    let steps = regex_steps.captures_iter(&input).next().map(|cap| cap[1].parse::<usize>().unwrap()).unwrap();
    let mut state = regex_begin.captures_iter(&input).next().and_then(|cap| cap.get(1)).unwrap().as_str();

    let mut tape = HashMap::new();
    let mut current_position = 0;

    for _ in 0..steps {
        let value = tape.entry(current_position).or_default();
        let rule = &rules[state][*value as usize];

        *value = rule.new_value;
        current_position += rule.direction;
        state = rule.new_state;
    }

    let result = tape.values().copied().map_into::<u64>().sum::<u64>();

    println!("{}", result);
    Ok(())
}
