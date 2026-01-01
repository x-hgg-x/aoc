use aoc::*;

use eyre::bail;
use itertools::Itertools;
use regex::Regex;

use std::collections::HashMap;

#[derive(Copy, Clone)]
enum Rule<'a> {
    Accepted,
    Rejected,
    Workflow(&'a str),
}

impl<'a> From<&'a str> for Rule<'a> {
    fn from(s: &'a str) -> Self {
        if s == "A" {
            Self::Accepted
        } else if s == "R" {
            Self::Rejected
        } else {
            Self::Workflow(s)
        }
    }
}

#[derive(Copy, Clone)]
enum Condition {
    Less(u8, i64),
    Greater(u8, i64),
}

#[derive(Copy, Clone)]
struct RuleWithCondition<'a> {
    condition: Option<Condition>,
    rule: Rule<'a>,
}

impl<'a> RuleWithCondition<'a> {
    fn parse(s: &'a str) -> Result<Self> {
        if let Some((condition, rule)) = s.split(':').next_tuple() {
            let rule = rule.into();

            let condition = if let Some((name, value)) = condition.split('<').next_tuple() {
                Some(Condition::Less(rating_index(name)?, value.parse()?))
            } else if let Some((name, value)) = condition.split('>').next_tuple() {
                Some(Condition::Greater(rating_index(name)?, value.parse()?))
            } else {
                bail!("invalid condition")
            };

            Ok(Self { condition, rule })
        } else {
            Ok(Self {
                condition: None,
                rule: s.into(),
            })
        }
    }

    fn evaluate(&self, rating: &[i64; 4]) -> Option<Rule<'a>> {
        match self.condition {
            None => Some(self.rule),
            Some(Condition::Less(index, value)) => {
                (rating[index as usize] < value).then_some(self.rule)
            }
            Some(Condition::Greater(index, value)) => {
                (rating[index as usize] > value).then_some(self.rule)
            }
        }
    }
}

fn rating_index(name: &str) -> Result<u8> {
    match name {
        "x" => Ok(0),
        "m" => Ok(1),
        "a" => Ok(2),
        "s" => Ok(3),
        _ => bail!("invalid name"),
    }
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let regex_workflows = Regex::new(r#"(?m)^(\w+)\{(.+)\}$"#)?;
    let regex_ratings = Regex::new(r#"(?m)^\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}$"#)?;

    let (workflows_input, ratings_input) = input.split("\n\n").next_tuple().value()?;

    let workflows: HashMap<&str, Vec<_>> = regex_workflows
        .captures_iter(workflows_input)
        .map(|cap| {
            let name = cap.get(1).value()?.as_str();

            let rules = (cap.get(2).value()?.as_str())
                .split(',')
                .map(RuleWithCondition::parse)
                .try_collect()?;

            Result::Ok((name, rules))
        })
        .try_collect()?;

    let ratings: Vec<[i64; 4]> = regex_ratings
        .captures_iter(ratings_input)
        .map(|cap| {
            let x = cap[1].parse()?;
            let m = cap[2].parse()?;
            let a = cap[3].parse()?;
            let s = cap[4].parse()?;
            Result::Ok([x, m, a, s])
        })
        .try_collect()?;

    let result1 = ratings
        .iter()
        .map(|rating| {
            let mut workflow_name = "in";

            loop {
                match (workflows[workflow_name].iter()).find_map(|rule| rule.evaluate(rating)) {
                    Some(Rule::Accepted) => break Ok(rating.iter().sum::<i64>()),
                    Some(Rule::Rejected) => break Ok(0),
                    Some(Rule::Workflow(name)) => workflow_name = name,
                    None => bail!("invalid workflow"),
                };
            }
        })
        .try_sum::<i64>()?;

    let mut possible_paths = Vec::new();
    let mut current_states = vec![("in", Vec::new())];

    while let Some((workflow_name, mut path)) = current_states.pop() {
        for rule in &workflows[workflow_name] {
            match rule.rule {
                Rule::Rejected => (),
                Rule::Accepted => {
                    let new_path = path.iter().copied().chain(rule.condition).collect_vec();
                    possible_paths.push(new_path);
                }
                Rule::Workflow(name) => {
                    let new_path = path.iter().copied().chain(rule.condition).collect_vec();
                    current_states.push((name, new_path))
                }
            }

            path.extend(rule.condition.map(|condition| match condition {
                Condition::Less(index, value) => Condition::Greater(index, value - 1),
                Condition::Greater(index, value) => Condition::Less(index, value + 1),
            }));
        }
    }

    let result2 = possible_paths
        .iter()
        .map(|path| {
            let mut rating_ranges = [(1, 4000); 4];

            for &condition in path {
                match condition {
                    Condition::Less(index, value) => {
                        let max = &mut rating_ranges[index as usize].1;
                        *max = (value - 1).min(*max);
                    }
                    Condition::Greater(index, value) => {
                        let min = &mut rating_ranges[index as usize].0;
                        *min = (value + 1).max(*min);
                    }
                }
            }

            (rating_ranges.iter().map(|(min, max)| max - min + 1)).product::<i64>()
        })
        .sum::<i64>();

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
