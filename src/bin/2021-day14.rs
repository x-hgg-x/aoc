use aoc::*;

use itertools::Itertools;
use regex::bytes::Regex;

use std::collections::HashMap;

fn step(
    rules: &HashMap<[u8; 2], u8>,
    pair_count: &mut HashMap<[u8; 2], u64>,
    new_pairs: &mut Vec<([u8; 2], u64)>,
) {
    new_pairs.clear();

    for (&[left, right], count) in pair_count.iter_mut() {
        if *count > 0
            && let Some(&middle) = rules.get(&[left, right])
        {
            new_pairs.push(([left, middle], *count));
            new_pairs.push(([middle, right], *count));
            *count = 0;
        }
    }

    for (pair, count) in new_pairs {
        *pair_count.entry(*pair).or_default() += *count;
    }
}

fn process_counts(
    elements: &[u8],
    pair_count: &HashMap<[u8; 2], u64>,
    first_elem: u8,
    last_elem: u8,
) -> Result<u64> {
    let (min, max) = elements
        .iter()
        .map(|&elem| {
            let initial_count = (elem == first_elem || elem == last_elem) as u64;

            let pair_sum = pair_count
                .iter()
                .map(|(&pair, &count)| count * pair.iter().filter(|&&x| x == elem).count() as u64)
                .sum::<u64>();

            initial_count + pair_sum / 2
        })
        .minmax()
        .into_option()
        .value()?;

    Ok(max - min)
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let re = Regex::new(r#"(?m)^(\w\w) -> (\w)$"#)?;

    let rules: HashMap<_, _> = re
        .captures_iter(input.as_bytes())
        .map(|cap| ([cap[1][0], cap[1][1]], cap[2][0]))
        .collect();

    let initial_polymer = input.lines().next().value()?.as_bytes();
    let first_elem = *initial_polymer.first().value()?;
    let last_elem = *initial_polymer.last().value()?;

    let mut elements = rules
        .iter()
        .flat_map(|(&[k0, k1], &v)| [k0, k1, v])
        .collect_vec();

    elements.sort_unstable();
    elements.dedup();

    let mut pair_count = HashMap::<_, u64>::new();
    for x in initial_polymer.windows(2) {
        *pair_count.entry([x[0], x[1]]).or_default() += 1;
    }

    let mut new_pairs = Vec::new();

    for _ in 0..10 {
        step(&rules, &mut pair_count, &mut new_pairs);
    }
    let result1 = process_counts(&elements, &pair_count, first_elem, last_elem)?;

    for _ in 10..40 {
        step(&rules, &mut pair_count, &mut new_pairs);
    }
    let result2 = process_counts(&elements, &pair_count, first_elem, last_elem)?;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
