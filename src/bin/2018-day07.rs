use eyre::Result;
use itertools::Itertools;
use regex::bytes::Regex;

use std::cmp::Reverse;
use std::fs;

struct Node {
    index: usize,
    children: u64,
    parents: u64,
}

fn compute_step_list(nodes: &[Node]) -> String {
    let mut step_list = String::new();
    let mut visited_nodes = 0u64;

    let mut queue = nodes.iter().filter(|node| node.parents == 0).collect_vec();
    queue.sort_unstable_by_key(|node| Reverse(node.index));

    while let Some(node) = queue.pop() {
        step_list.push((node.index as u8 + b'A') as char);
        visited_nodes |= 1 << node.index;

        for child_node in nodes {
            if (visited_nodes >> child_node.index) & 1 == 0
                && (node.children >> child_node.index) & 1 != 0
                && child_node.parents & visited_nodes == child_node.parents
            {
                queue.push(child_node);
            }
        }

        queue.sort_unstable_by_key(|node| Reverse(node.index));
    }

    step_list
}

fn compute_total_time(nodes: &[Node]) -> u64 {
    let mut total_time = 0u64;
    let mut visited_nodes = 0u64;
    let mut finished_nodes = 0u64;
    let mut workers = [Option::<(&Node, u64)>::None; 5];

    let mut queue = nodes.iter().filter(|node| node.parents == 0).collect_vec();
    for node in &queue {
        visited_nodes |= 1 << node.index;
    }

    loop {
        queue.sort_unstable_by_key(|node| Reverse(node.index));

        for worker in workers.iter_mut().filter(|x| matches!(x, None | Some((_, 0)))) {
            match queue.pop() {
                None => break,
                Some(node) => {
                    *worker = Some((node, node.index as u64 + 61));
                }
            }
        }

        match workers.iter().flatten().map(|&(_, remaining_time)| remaining_time).filter(|&remaining_time| remaining_time != 0).min() {
            None => break,
            Some(elapsed_time) => {
                total_time += elapsed_time;

                for (worker_node, worker_timer) in workers.iter_mut().flatten().filter(|(_, remaining_time)| *remaining_time != 0) {
                    *worker_timer -= elapsed_time;

                    if *worker_timer == 0 {
                        finished_nodes |= 1 << worker_node.index;

                        for child_node in nodes {
                            if (visited_nodes >> child_node.index) & 1 == 0
                                && (worker_node.children >> child_node.index) & 1 != 0
                                && child_node.parents & finished_nodes == child_node.parents
                            {
                                queue.push(child_node);
                                visited_nodes |= 1 << child_node.index;
                            }
                        }
                    }
                }
            }
        }
    }

    total_time
}

fn main() -> Result<()> {
    let input = fs::read("inputs/2018-day07.txt")?;

    let re = Regex::new(r#"(?m)^Step (\w) must be finished before step (\w) can begin.$"#)?;

    let edges = re.captures_iter(&input).map(|cap| [cap[1][0] - b'A', cap[2][0] - b'A']).collect_vec();

    let size = 1 + edges.iter().copied().flatten().max().unwrap() as usize;

    let mut nodes = (0..size).map(|index| Node { index, children: 0, parents: 0 }).collect_vec();
    for &edge in &edges {
        nodes[edge[0] as usize].children |= 1 << edge[1];
        nodes[edge[1] as usize].parents |= 1 << edge[0];
    }

    let result1 = compute_step_list(&nodes);
    let result2 = compute_total_time(&nodes);

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
