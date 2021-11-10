use eyre::Result;
use itertools::Itertools;

use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

fn visit(graph: &HashMap<u64, Vec<u64>>, id: u64) -> HashSet<u64> {
    let mut visited = HashSet::new();

    let mut queue = VecDeque::new();
    queue.push_back(id);

    while let Some(id) = queue.pop_front() {
        if visited.insert(id) {
            queue.extend(&graph[&id]);
        }
    }

    visited
}

fn main() -> Result<()> {
    let input = fs::read_to_string("inputs/2017-day12.txt")?;

    let graph: HashMap<_, _> = input
        .lines()
        .map(|line| {
            let mut iter = line.split(|c: char| !c.is_ascii_digit()).filter(|x| !x.is_empty()).map(|x| x.parse::<u64>().unwrap());
            (iter.next().unwrap(), iter.collect_vec())
        })
        .collect();

    let result1 = visit(&graph, 0).len();

    let mut groups = 0usize;
    let mut ids: HashSet<_> = graph.keys().copied().collect();

    while let Some(&id) = ids.iter().next() {
        ids.remove(&id);

        for linked_id in visit(&graph, id) {
            ids.remove(&linked_id);
        }

        groups += 1;
    }

    let result2 = groups;

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
