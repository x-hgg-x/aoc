use aoc::*;

use itertools::Itertools;

use std::collections::{HashMap, HashSet, VecDeque};

fn compute_farthest_node<'a>(
    graph: &HashMap<&str, Vec<&'a str>>,
    node: &'a str,
    current_states: &mut Vec<(&'a str, u64)>,
    visited: &mut HashSet<&'a str>,
) -> &'a str {
    visited.clear();

    current_states.clear();
    current_states.push((node, 0));

    let mut max_steps = 0u64;
    let mut farthest_node = node;

    while let Some((current_node, steps)) = current_states.pop() {
        if steps > max_steps {
            max_steps = steps;
            farthest_node = current_node;
        }

        current_states.extend(
            graph[current_node]
                .iter()
                .filter(|&&link| visited.insert(link))
                .map(|&link| (link, steps + 1)),
        );
    }

    farthest_node
}

fn compute_group_size<'a>(
    graph: &HashMap<&str, Vec<&'a str>>,
    start_node: &'a str,
    end_node: &str,
    visited: &mut HashSet<&'a str>,
) -> usize {
    let mut visited_edges = HashSet::new();
    let mut current_states = VecDeque::new();

    for _ in 0..4 {
        visited.clear();

        current_states.clear();
        current_states.push_back((start_node, Vec::new()));

        while let Some((node, path)) = current_states.pop_front() {
            if !visited.insert(node) {
                continue;
            }

            if node == end_node {
                visited_edges.extend(path.windows(2).flat_map(|x| [(x[0], x[1]), (x[1], x[0])]));
                break;
            }

            for &link in &graph[node] {
                if !visited_edges.contains(&(node, link)) {
                    let new_path = path.iter().copied().chain([link]).collect_vec();
                    current_states.push_back((link, new_path));
                }
            }
        }
    }

    visited.len()
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let mut graph = HashMap::<_, Vec<_>>::new();

    for line in input.lines() {
        let (node, links) = line.split(": ").next_tuple().value()?;
        let links = links.split_ascii_whitespace().collect_vec();
        for link in links {
            graph.entry(node).or_default().push(link);
            graph.entry(link).or_default().push(node);
        }
    }

    let mut current_states = Vec::new();
    let mut visited = HashSet::new();

    let first_node = input.split(':').next().value()?;
    let start_node = compute_farthest_node(&graph, first_node, &mut current_states, &mut visited);
    let end_node = compute_farthest_node(&graph, start_node, &mut current_states, &mut visited);

    let group_size = compute_group_size(&graph, start_node, end_node, &mut visited);
    let result = group_size * (graph.len() - group_size);

    println!("{result}");
    Ok(())
}
