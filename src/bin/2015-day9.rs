use itertools::Itertools;
use regex::Regex;

use std::collections::HashMap;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("inputs/2015-day9.txt")?;

    let regex_nodes = Regex::new(r#"(?m)^(\w+ to \w+)"#).unwrap();
    let regex_edges = Regex::new(r#"(?m)^(\w+) to (\w+) = (\d+)$"#).unwrap();

    let nodes = regex_nodes
        .find_iter(&input)
        .flat_map(|x| x.as_str().split(" to "))
        .sorted_unstable()
        .dedup()
        .collect_vec();

    let edges: HashMap<(String, String), u32> = regex_edges
        .captures_iter(&input)
        .flat_map(|cap| {
            let distance: u32 = cap[3].parse().unwrap();
            vec![
                ((cap[1].to_owned(), cap[2].to_owned()), distance),
                ((cap[2].to_owned(), cap[1].to_owned()), distance),
            ]
        })
        .collect();

    let iter = nodes.iter().permutations(nodes.len()).map(|x| -> u32 {
        x.windows(2)
            .map(|x| edges[&((*x[0]).into(), (*x[1]).into())])
            .sum()
    });

    let result1 = iter.clone().min().unwrap();
    let result2 = iter.max().unwrap();

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
