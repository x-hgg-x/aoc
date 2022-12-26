use aoc::*;

use eyre::ensure;
use itertools::Itertools;
use smallvec::SmallVec;

use std::collections::{HashMap, VecDeque};

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let (rules_input, messages_input) = input.split("\n\n").next_tuple().value()?;

    let messages = messages_input.lines().map(|line| line.as_bytes()).collect_vec();

    let mut graph = HashMap::new();
    let mut inverted_graph = HashMap::<_, Vec<_>>::new();
    let mut valid_rule_messages = HashMap::<_, Vec<_>>::new();

    for line in rules_input.lines() {
        let (name, content) = line.split(": ").next_tuple().value()?;

        inverted_graph.entry(name).or_default();

        match *content.as_bytes() {
            [b'"', letter, b'"'] => {
                valid_rule_messages.insert(name, vec![SmallVec::from_slice(&[letter])]);
                graph.insert(name, SmallVec::new());
            }
            _ => {
                let sub_rules: SmallVec<[SmallVec<[_; 2]>; 2]> = content.split('|').map(|x| x.split_ascii_whitespace().collect()).collect();

                for &dependency in sub_rules.iter().flatten() {
                    inverted_graph.entry(dependency).or_default().push(name);
                }

                graph.insert(name, sub_rules);
            }
        }
    }

    for dependencies in inverted_graph.values_mut() {
        dependencies.sort_unstable();
        dependencies.dedup();
    }

    let mut queue: VecDeque<_> = valid_rule_messages
        .keys()
        .flat_map(|&name| &inverted_graph[name])
        .copied()
        .filter(|&name| graph[name].iter().flatten().all(|&dep| valid_rule_messages.get(dep).is_some()))
        .sorted_unstable()
        .dedup()
        .collect();

    while let Some(name) = queue.pop_front() {
        if name == "0" {
            break;
        }

        let mut valid_messages = Vec::new();

        for possibilities in &graph[name] {
            let combinations: SmallVec<[_; 2]> = possibilities.iter().map(|&x| valid_rule_messages[x].as_slice()).collect();
            let mut current_combination: SmallVec<[_; 2]> = SmallVec::from_elem(0, possibilities.len());

            'outer: loop {
                let mut valid_message = SmallVec::<[_; 16]>::new();
                for (&values, &index) in combinations.iter().zip(&current_combination) {
                    valid_message.extend_from_slice(&values[index]);
                }
                valid_messages.push(valid_message);

                for (pos, (&values, index)) in combinations.iter().zip(&mut current_combination).enumerate() {
                    if *index < values.len() - 1 {
                        *index += 1;
                        break;
                    } else if pos == possibilities.len() - 1 {
                        break 'outer;
                    } else {
                        *index = 0;
                    }
                }
            }
        }

        valid_rule_messages.insert(name, valid_messages);
        queue.extend(inverted_graph[name].iter().copied().filter(|&x| graph[x].iter().flatten().all(|&dep| valid_rule_messages.get(dep).is_some())));
    }

    ensure!(graph["0"].as_slice() == [SmallVec::from_buf(["8", "11"])], "invalid input");
    ensure!(graph["8"].as_slice() == [SmallVec::from_buf(["42"])], "invalid input");
    ensure!(graph["11"].as_slice() == [SmallVec::from_buf(["42", "31"])], "invalid input");

    let rule_31_messages = valid_rule_messages.get("31").value()?;
    let rule_42_messages = valid_rule_messages.get("42").value()?;

    let rule_31_len = rule_31_messages.first().value()?.len();
    let rule_42_len = rule_42_messages.first().value()?.len();

    ensure!(rule_31_messages.iter().map(|x| x.len()).all_equal(), "invalid input");
    ensure!(rule_42_messages.iter().map(|x| x.len()).all_equal(), "invalid input");

    let mut count1 = 0usize;
    let mut count2 = 0usize;

    for &message in messages.iter().filter(|x| x.len() >= 2 * rule_42_len + rule_31_len) {
        let check_combination = |total_42_len| {
            let (slices_42, slices_31) = message.split_at(total_42_len);

            let check_42 = slices_42.chunks_exact(rule_42_len).all(|s| rule_42_messages.iter().any(|item| item.as_slice() == s));
            let check_31 = slices_31.chunks_exact(rule_31_len).all(|s| rule_31_messages.iter().any(|item| item.as_slice() == s));

            check_42 && check_31
        };

        let iter = (0..=(message.len() - 2 * rule_42_len - rule_31_len) / (rule_42_len + rule_31_len)).map(|n| message.len() - rule_31_len * (1 + n));

        let check1 = iter.clone().next().filter(|&x| x == 2 * rule_42_len).map(check_combination);
        let check2 = iter.filter(|&x| x % rule_42_len == 0).any(check_combination);

        if check1 == Some(true) {
            count1 += 1;
            count2 += 1;
        } else if check2 {
            count2 += 1;
        }
    }

    let result1 = count1;
    let result2 = count2;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
