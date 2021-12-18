use aoc::*;

use itertools::Itertools;

use std::collections::HashMap;

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let graph: HashMap<_, Vec<_>> = input
        .lines()
        .map(|line| {
            let mut iter = line.split(|c: char| !c.is_ascii_digit()).filter(|x| !x.is_empty()).map(|x| x.parse::<usize>());
            Result::Ok((iter.next().transpose()?.value()?, iter.try_collect()?))
        })
        .try_collect()?;

    let mut group_0_size = 0usize;
    let mut visited = vec![false; graph.len()];
    let mut queue = vec![0];

    while let Some(id) = queue.pop() {
        if !visited[id] {
            visited[id] = true;
            queue.extend(&graph[&id]);
            group_0_size += 1;
        }
    }

    let result1 = group_0_size;

    let mut groups_count = 0usize;
    visited.fill(false);

    for index in 0..visited.len() {
        if !visited[index] {
            queue.clear();
            queue.push(index);

            while let Some(id) = queue.pop() {
                if !visited[id] {
                    visited[id] = true;
                    queue.extend(&graph[&id]);
                }
            }
            groups_count += 1;
        }
    }

    let result2 = groups_count;

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
