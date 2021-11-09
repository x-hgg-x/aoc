use eyre::{bail, Result};
use itertools::Itertools;
use regex::Regex;
use smallvec::SmallVec;

use std::cell::Cell;
use std::collections::HashMap;
use std::fs;

#[derive(Default)]
struct Node<'a> {
    name: &'a str,
    weight: u64,
    total_weight: Cell<Option<u64>>,
    children_names: SmallVec<[&'a str; 8]>,
    parent: Option<&'a str>,
}

fn compute_total_weight(map: &HashMap<&str, Node>, node_name: &str) -> u64 {
    let node = &map[node_name];
    node.total_weight.get().unwrap_or_else(|| {
        let total_weight = node.children_names.iter().fold(node.weight, |acc, &child_name| acc + compute_total_weight(map, child_name));
        node.total_weight.set(Some(total_weight));
        total_weight
    })
}

fn compute_unbalanced_node_corrected_weight(map: &HashMap<&str, Node>, bottom_node: &Node) -> Result<u64> {
    let mut unbalanced_node_name = bottom_node.name;
    let mut balanced_weight = bottom_node.weight;

    loop {
        let mut children_total_weights = map[unbalanced_node_name]
            .children_names
            .iter()
            .map(|&child_name| (child_name, map[child_name].total_weight.get().unwrap()))
            .collect::<SmallVec<[_; 8]>>();

        children_total_weights.sort_unstable_by_key(|&(_, weight)| weight);

        let mut iter = children_total_weights.iter().dedup_by_with_count(|&(_, weight_1), &(_, weight_2)| weight_1 == weight_2);

        match (iter.next(), iter.next()) {
            (Some((1, &(child_name, _))), Some((_, &(_, weight)))) | (Some((_, &(_, weight))), Some((1, &(child_name, _)))) => {
                unbalanced_node_name = child_name;
                balanced_weight = weight;
            }
            (Some(_), None) => break,
            _ => bail!("multiple unbalanced nodes"),
        };
    }

    let unbalanced_node = &map[unbalanced_node_name];
    Ok(unbalanced_node.weight + balanced_weight - unbalanced_node.total_weight.get().unwrap())
}

fn main() -> Result<()> {
    let input = fs::read_to_string("inputs/2017-day07.txt")?;

    let regex_line = Regex::new(r#"(?m)^(\w+)\s+\((\d+)\)((?:\s+->.*)?)$"#)?;
    let regex_children = Regex::new(r#"\w+"#)?;

    let mut map = HashMap::<_, Node>::new();

    for cap in regex_line.captures_iter(&input) {
        let node_name = cap.get(1).unwrap().as_str();
        let node_weight = cap[2].parse()?;
        let children_names = regex_children.find_iter(cap.get(3).unwrap().as_str()).map(|x| x.as_str()).collect();

        for &child_name in &children_names {
            map.entry(child_name).or_default().parent = Some(node_name);
        }

        let node = map.entry(node_name).or_default();
        node.name = node_name;
        node.weight = node_weight;
        node.children_names = children_names;
    }

    let bottom_node = map.values().find(|&node| node.parent.is_none()).unwrap();
    compute_total_weight(&map, bottom_node.name);

    let result1 = bottom_node.name;
    let result2 = compute_unbalanced_node_corrected_weight(&map, bottom_node)?;

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
