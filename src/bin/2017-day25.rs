use aoc::*;

use eyre::bail;
use itertools::Itertools;
use regex::{Match, Regex};

use std::collections::HashMap;

struct Rule<'a> {
    new_value: u8,
    direction: i64,
    new_state: &'a str,
}

fn parse_rule<'a>(match_new_value: Match<'a>, match_direction: Match<'a>, match_new_state: Match<'a>) -> Result<Rule<'a>> {
    Ok(Rule {
        new_value: match_new_value.as_str().parse()?,
        direction: match match_direction.as_str() {
            "left" => -1,
            "right" => 1,
            _ => bail!("unknown offset"),
        },
        new_state: match_new_state.as_str(),
    })
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

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
            let current_state = cap.get(1).value()?.as_str();

            let state_rules = [
                parse_rule(cap.get(2).value()?, cap.get(3).value()?, cap.get(4).value()?)?,
                parse_rule(cap.get(5).value()?, cap.get(6).value()?, cap.get(7).value()?)?,
            ];

            Result::Ok((current_state, state_rules))
        })
        .try_collect()?;

    let steps = regex_steps.captures_iter(&input).next().value()?[1].parse::<usize>()?;
    let mut state = regex_begin.captures_iter(&input).next().value()?.get(1).value()?.as_str();

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
