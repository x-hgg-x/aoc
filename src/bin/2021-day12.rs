use aoc::*;

use itertools::Itertools;
use smallvec::SmallVec;

use std::collections::HashMap;

struct State<'a> {
    path: SmallVec<[u8; 24]>,
    neighbors: &'a [u8],
    twice: bool,
}

fn count_paths<'a>(graph: &'a HashMap<u8, Vec<u8>>, nodes: &[&str], idx_start: u8, idx_end: u8, queue: &mut Vec<State<'a>>, twice: bool) -> usize {
    let mut count = 0usize;

    queue.clear();
    queue.push(State { path: SmallVec::from_slice(&[idx_start]), neighbors: &graph[&idx_start][..], twice });

    while let Some(state) = queue.pop() {
        for &idx_node in state.neighbors {
            let mut twice = state.twice;
            let node = nodes[idx_node as usize];

            if node.as_bytes()[0].is_ascii_lowercase() && state.path.contains(&idx_node) {
                if twice && idx_node != idx_start && idx_node != idx_end {
                    twice = false;
                } else {
                    continue;
                }
            }

            if idx_node == idx_end {
                count += 1;
                continue;
            }

            let mut path = state.path.clone();
            path.push(idx_node);
            queue.push(State { path, neighbors: &graph[&idx_node], twice });
        }
    }

    count
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let edges: Vec<(_, _)> = input.lines().map(|line| line.split('-').next_tuple().value()).try_collect()?;

    let mut nodes = edges.iter().flat_map(|&(x, y)| [x, y]).collect_vec();
    nodes.sort_unstable();
    nodes.dedup();

    let mapping: HashMap<_, _> = nodes.iter().enumerate().map(|(index, &node)| (node, index as u8)).collect();

    let mut graph = HashMap::<_, Vec<_>>::new();
    for &(x, y) in &edges {
        let mx = mapping[x];
        let my = mapping[y];
        graph.entry(mx).or_default().push(my);
        graph.entry(my).or_default().push(mx);
    }

    let idx_start = mapping["start"];
    let idx_end = mapping["end"];

    let mut queue = Vec::new();

    let result1 = count_paths(&graph, &nodes, idx_start, idx_end, &mut queue, false);
    let result2 = count_paths(&graph, &nodes, idx_start, idx_end, &mut queue, true);

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
