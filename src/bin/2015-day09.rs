use itertools::Itertools;
use regex::Regex;

use std::collections::HashMap;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("inputs/2015-day09.txt")?;

    let re = Regex::new(r#"(?m)^(\w+) to (\w+) = (\d+)$"#).unwrap();

    let nodes = re
        .captures_iter(&input)
        .flat_map(|cap| {
            cap.iter()
                .skip(1)
                .take(2)
                .filter_map(|x| x)
                .map(|x| &input[x.range()])
                .collect_vec()
        })
        .sorted_unstable()
        .dedup()
        .collect_vec();

    let edges: HashMap<(String, String), u32> = re
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
