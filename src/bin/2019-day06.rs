use aoc::*;

use itertools::Itertools;
use smallvec::SmallVec;

use std::collections::HashMap;

fn path_from_object_to_com<'a>(inverted_graph: &HashMap<&'a str, &'a str>, object: &'a str) -> Vec<&'a str> {
    let mut current_object = inverted_graph[object];
    let mut to_com = vec![current_object];
    while current_object != "COM" {
        current_object = inverted_graph[current_object];
        to_com.push(current_object);
    }
    to_com
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let mut graph = <HashMap<_, SmallVec<[_; 2]>>>::new();
    let mut inverted_graph = HashMap::new();

    for line in input.lines() {
        let (center, object) = line.split(')').next_tuple().value()?;
        graph.entry(center).or_default().push(object);
        graph.entry(object).or_default();
        inverted_graph.insert(object, center);
    }

    let mut orbit_count = 0usize;
    let mut queue = vec![(0, "COM")];

    while let Some((depth, id)) = queue.pop() {
        orbit_count += depth;
        let new_depth = depth + 1;
        for &new_id in &graph[&id] {
            queue.push((new_depth, new_id))
        }
    }

    let result1 = orbit_count;

    let you_to_com = path_from_object_to_com(&inverted_graph, "YOU");
    let san_to_com = path_from_object_to_com(&inverted_graph, "SAN");

    let result2 = match you_to_com.iter().rev().zip(san_to_com.iter().rev()).position(|(&you_obj, &san_obj)| you_obj != san_obj) {
        None => 0,
        Some(position) => you_to_com.len() + san_to_com.len() - 2 * position,
    };

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
