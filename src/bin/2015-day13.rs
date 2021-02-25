use itertools::Itertools;
use regex::Regex;
use smallvec::SmallVec;

use std::collections::HashMap;
use std::fs;

fn max_hapiness(nodes: &[&str], edges: &HashMap<(String, String), i32>) -> i32 {
    nodes
        .iter()
        .permutations(nodes.len())
        .map(|mut x| -> i32 {
            x.push(x[0]);
            x.windows(2)
                .map(|x| {
                    let edge1 = ((*x[0]).into(), (*x[1]).into());
                    let edge2 = ((*x[1]).into(), (*x[0]).into());
                    edges[&edge1] + edges[&edge2]
                })
                .sum()
        })
        .max()
        .unwrap()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("inputs/2015-day13.txt")?;

    let re = Regex::new(r#"(?m)^(\w+) would (lose|gain) (\d+).*?(\w+).$"#).unwrap();

    let mut nodes = re
        .captures_iter(&input)
        .flat_map(|cap| {
            SmallVec::from_buf([cap.get(1).unwrap().as_str(), cap.get(4).unwrap().as_str()])
        })
        .sorted_unstable()
        .dedup()
        .collect_vec();

    let mut edges = re
        .captures_iter(&input)
        .map(|cap| {
            let happiness: i32 = cap[3].parse::<i32>().unwrap()
                * match &cap[2] {
                    "lose" => -1,
                    "gain" => 1,
                    _ => 0,
                };
            ((cap[1].to_owned(), cap[4].to_owned()), happiness)
        })
        .collect();

    let result1 = max_hapiness(&nodes, &edges);

    for &node in &nodes {
        edges.insert(("Me".into(), node.into()), 0);
        edges.insert((node.into(), "Me".into()), 0);
    }
    nodes.push("Me");

    let result2 = max_hapiness(&nodes, &edges);

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
