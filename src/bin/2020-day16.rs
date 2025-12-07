use aoc::*;

use eyre::ensure;
use itertools::Itertools;
use regex::Regex;
use smallvec::SmallVec;

use std::iter;

fn check_rules_mapping(
    mut rule_indices: SmallVec<[Option<u64>; 20]>,
    possibilities: &[u64],
) -> Result<()> {
    rule_indices.sort_unstable();

    ensure!(
        possibilities.iter().all(|&x| x == 0),
        "unable to map rules to fields"
    );

    ensure!(
        rule_indices
            .iter()
            .enumerate()
            .all(|(index, &x)| x == Some(index as u64)),
        "unable to map rules to fields"
    );

    Ok(())
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let regex_rules = Regex::new(r#"(?m)^(.+?): (\d+)-(\d+) or (\d+)-(\d+)$"#)?;

    let (rules_input, self_ticket_input, nearby_tickets_input) =
        input.split("\n\n").next_tuple().value()?;

    let rules: Vec<_> = regex_rules
        .captures_iter(rules_input)
        .map(|cap| {
            let field = cap.get(1).value()?.as_str();
            let range1 = cap[2].parse::<u64>()?..=cap[3].parse::<u64>()?;
            let range2 = cap[4].parse::<u64>()?..=cap[5].parse::<u64>()?;
            Result::Ok((field, [range1, range2]))
        })
        .try_collect()?;

    let self_ticket: SmallVec<[u64; 20]> = self_ticket_input
        .lines()
        .last()
        .value()?
        .split(',')
        .map(|x| x.parse())
        .try_collect()?;

    let mut nearby_tickets: Vec<SmallVec<[u64; 20]>> = nearby_tickets_input
        .lines()
        .skip(1)
        .map(|line| line.split(',').map(|x| x.parse()).try_collect())
        .try_collect()?;

    let min_rule = rules
        .iter()
        .flat_map(|(_, ranges)| ranges)
        .map(|x| *x.start())
        .min()
        .value()?;

    let max_rule = rules
        .iter()
        .flat_map(|(_, ranges)| ranges)
        .map(|x| *x.end())
        .max()
        .value()?;

    let mut invalid_values = (min_rule..=max_rule).map(Some).collect_vec();

    for range in rules.iter().flat_map(|(_, ranges)| ranges) {
        invalid_values[(range.start() - min_rule) as usize..=(range.end() - min_rule) as usize]
            .fill(None);
    }

    let invalid_values = invalid_values.iter().copied().flatten().collect_vec();

    let mut scanning_error_rate = 0;

    nearby_tickets.retain(|ticket| {
        let sum = ticket
            .iter()
            .filter(|&&value| {
                value < min_rule || invalid_values.contains(&value) || value > max_rule
            })
            .sum1::<u64>();

        if let Some(sum) = sum {
            scanning_error_rate += sum;
            false
        } else {
            true
        }
    });

    let result1 = scanning_error_rate;

    let mut possibilities =
        SmallVec::<[_; 20]>::from_elem(!0u64 >> (u64::BITS - rules.len() as u32), rules.len());

    for (index, (_, ranges)) in rules.iter().enumerate() {
        let bit = 1 << index;

        for ticket in iter::chain([&self_ticket], &nearby_tickets) {
            for (value, possibility) in ticket.iter().zip(&mut possibilities) {
                if !ranges.iter().any(|range| range.contains(value)) {
                    *possibility &= !bit;
                }
            }
        }
    }

    let mut rule_indices = SmallVec::<[_; 20]>::from_elem(None, rules.len());

    while let Some((index, &possibility)) =
        (possibilities.iter()).find_position(|possibility| possibility.count_ones() == 1)
    {
        rule_indices[index] = Some(possibility.trailing_zeros() as u64);
        possibilities.iter_mut().for_each(|x| *x &= !possibility);
    }

    check_rules_mapping(rule_indices.clone(), &possibilities)?;

    let result2 = rule_indices
        .iter()
        .flatten()
        .enumerate()
        .filter(|&(_, &rule_index)| rules[rule_index as usize].0.starts_with("departure"))
        .map(|(index, _)| self_ticket[index])
        .product::<u64>();

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
