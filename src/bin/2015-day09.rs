use itertools::Itertools;
use regex::Regex;

use std::collections::HashMap;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("inputs/2015-day09.txt")?;

    let re = Regex::new(r#"(?m)^(\w+) to (\w+) = (\d+)$"#).unwrap();

    let nodes = re.captures_iter(&input).flat_map(|cap| [cap.get(1).unwrap().as_str(), cap.get(2).unwrap().as_str()]).sorted_unstable().dedup().collect_vec();

    let edges = HashMap::<_, _>::from_iter(re.captures_iter(&input).flat_map(|cap| {
        let location1 = cap.get(1).unwrap().as_str();
        let location2 = cap.get(2).unwrap().as_str();
        let distance: u32 = cap[3].parse().unwrap();
        [((location1, location2), distance), ((location2, location1), distance)]
    }));

    let distances = nodes.iter().permutations(nodes.len()).map(|x| -> u32 { x.windows(2).map(|x| edges[&(*x[0], *x[1])]).sum() }).collect_vec();

    let result1 = *distances.iter().min().unwrap();
    let result2 = *distances.iter().max().unwrap();

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
