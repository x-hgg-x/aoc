use aoc::*;

use itertools::Itertools;
use regex::Regex;
use smallvec::SmallVec;

use std::collections::{HashMap, HashSet, VecDeque};

const STARTING_BAG: &str = "shiny gold";

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let re = Regex::new(r#"(\d+) (.+?) bag"#)?;

    let mut graph = <HashMap<_, SmallVec<[_; 4]>>>::new();
    let mut inverted_graph = <HashMap<_, Vec<_>>>::new();

    for line in input.lines() {
        let (bag, description) = line.split(" bags contain ").next_tuple().value()?;

        let mut content = <SmallVec<[_; 4]>>::new();
        for cap in re.captures_iter(description) {
            let bag_count = cap[1].parse::<u64>()?;
            let content_bag = cap.get(2).value()?.as_str();

            content.push((bag_count, content_bag));
            inverted_graph.entry(content_bag).or_default().push(bag);
        }

        graph.insert(bag, content);
        inverted_graph.entry(bag).or_default();
    }

    let mut visited = HashSet::from([STARTING_BAG]);
    let mut queue = inverted_graph[STARTING_BAG].clone();

    let mut outside_bag_count = 0usize;
    while let Some(bag) = queue.pop() {
        if visited.insert(bag) {
            queue.extend(&inverted_graph[&bag]);
            outside_bag_count += 1;
        }
    }

    let result1 = outside_bag_count;

    let mut inside_bag_counts = HashMap::new();

    let mut queue: VecDeque<_> = graph
        .iter()
        .filter(|&(_, content)| content.is_empty())
        .map(|(&bag, _)| bag)
        .collect();

    while let Some(bag) = queue.pop_front() {
        let count = graph[bag]
            .iter()
            .map(|&(bag_count, content_bag)| bag_count * (1 + inside_bag_counts[content_bag]))
            .sum::<u64>();

        inside_bag_counts.insert(bag, count);

        queue.extend(inverted_graph[bag].iter().filter(|&&x| {
            (graph[x].iter()).all(|&(_, content_bag)| inside_bag_counts.contains_key(content_bag))
        }));
    }

    let result2 = inside_bag_counts[STARTING_BAG];

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
