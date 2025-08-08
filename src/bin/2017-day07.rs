use aoc::*;

use eyre::bail;
use itertools::Itertools;
use regex::Regex;
use smallvec::SmallVec;

use std::collections::{HashMap, VecDeque};

struct Node<'a> {
    name: &'a str,
    weight: u64,
    children_names: SmallVec<[&'a str; 8]>,
}

fn compute_total_weights<'a>(nodes: &HashMap<&'a str, Node>, parents: &HashMap<&'a str, Option<&'a str>>) -> HashMap<&'a str, u64> {
    let mut total_weights = HashMap::new();
    let mut queue: VecDeque<_> = nodes.iter().filter(|&(_, node)| node.children_names.is_empty()).map(|(&name, _)| name).collect();

    while let Some(name) = queue.pop_front() {
        let node = &nodes[name];
        total_weights.insert(name, node.children_names.iter().fold(node.weight, |acc, &child_name| acc + total_weights[child_name]));

        if let Some(parent) = parents[name]
            && nodes[parent].children_names.iter().all(|&x| total_weights.contains_key(x))
        {
            queue.push_back(parent);
        }
    }

    total_weights
}

fn compute_unbalanced_node_corrected_weight(nodes: &HashMap<&str, Node>, bottom_node: &Node, total_weights: &HashMap<&str, u64>) -> Result<u64> {
    let mut unbalanced_node_name = bottom_node.name;
    let mut balanced_weight = bottom_node.weight;

    loop {
        let mut children_total_weights: SmallVec<[_; 8]> =
            nodes[unbalanced_node_name].children_names.iter().map(|&child_name| (child_name, total_weights[child_name])).collect();

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

    let unbalanced_node = &nodes[unbalanced_node_name];
    Ok(unbalanced_node.weight + balanced_weight - total_weights[unbalanced_node.name])
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let regex_line = Regex::new(r#"(?m)^(\w+)\s+\((\d+)\)((?:\s+->.*)?)$"#)?;
    let regex_children = Regex::new(r#"\w+"#)?;

    let mut nodes = HashMap::new();
    let mut parents = HashMap::new();

    for cap in regex_line.captures_iter(&input) {
        let node_name = cap.get(1).value()?.as_str();
        let node_weight = cap[2].parse()?;
        let children_names = regex_children.find_iter(cap.get(3).value()?.as_str()).map(|x| x.as_str()).collect();

        for &child_name in &children_names {
            parents.insert(child_name, Some(node_name));
        }
        parents.entry(node_name).or_default();

        nodes.insert(node_name, Node { name: node_name, weight: node_weight, children_names });
    }

    let bottom_node_name = parents.iter().find(|&(_, &v)| v.is_none()).map(|(&k, _)| k).value()?;
    let total_weights = compute_total_weights(&nodes, &parents);

    let result1 = bottom_node_name;
    let result2 = compute_unbalanced_node_corrected_weight(&nodes, &nodes[bottom_node_name], &total_weights)?;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
