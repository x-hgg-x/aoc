use eyre::Result;
use itertools::Itertools;
use regex::Regex;

use std::collections::HashMap;
use std::fs;

fn max_hapiness(nodes: &[&str], edges: &HashMap<(&str, &str), i32>) -> i32 {
    nodes
        .iter()
        .permutations(nodes.len())
        .map(|mut x| -> i32 {
            x.push(x[0]);
            x.windows(2).map(|x| edges.get(&(*x[0], *x[1])).unwrap() + edges.get(&(*x[1], *x[0])).unwrap()).sum()
        })
        .max()
        .unwrap()
}

fn main() -> Result<()> {
    let input = fs::read_to_string("inputs/2015-day13.txt")?;

    let re = Regex::new(r#"(?m)^(\w+) would (lose|gain) (\d+).*?(\w+).$"#)?;

    let mut nodes =
        re.captures_iter(&input).flat_map(|cap| [cap.get(1).unwrap().as_str(), cap.get(4).unwrap().as_str()]).sorted_unstable().dedup().collect_vec();

    let mut edges = re
        .captures_iter(&input)
        .map(|cap| {
            let happiness: i32 = cap[3].parse::<i32>().unwrap()
                * match &cap[2] {
                    "lose" => -1,
                    "gain" => 1,
                    _ => 0,
                };
            ((cap.get(1).unwrap().as_str(), cap.get(4).unwrap().as_str()), happiness)
        })
        .collect();

    let result1 = max_hapiness(&nodes, &edges);

    for &node in &nodes {
        edges.insert(("Me", node), 0);
        edges.insert((node, "Me"), 0);
    }
    nodes.push("Me");

    let result2 = max_hapiness(&nodes, &edges);

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
