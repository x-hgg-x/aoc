use itertools::Itertools;
use regex::Regex;
use smallvec::SmallVec;

use std::collections::HashMap;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("inputs/2015-day09.txt")?;

    let re = Regex::new(r#"(?m)^(\w+) to (\w+) = (\d+)$"#).unwrap();

    let nodes = re
        .captures_iter(&input)
        .flat_map(|cap| {
            SmallVec::from_buf([cap.get(1).unwrap().as_str(), cap.get(2).unwrap().as_str()])
        })
        .sorted_unstable()
        .dedup()
        .collect_vec();

    let edges: HashMap<(String, String), u32> = re
        .captures_iter(&input)
        .flat_map(|cap| {
            let distance: u32 = cap[3].parse().unwrap();
            SmallVec::from_buf([
                ((cap[1].to_owned(), cap[2].to_owned()), distance),
                ((cap[2].to_owned(), cap[1].to_owned()), distance),
            ])
        })
        .collect();

    let distances = nodes
        .iter()
        .permutations(nodes.len())
        .map(|x| -> u32 {
            x.windows(2)
                .map(|x| edges[&((*x[0]).into(), (*x[1]).into())])
                .sum()
        })
        .collect_vec();

    let result1 = *distances.iter().min().unwrap();
    let result2 = *distances.iter().max().unwrap();

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
