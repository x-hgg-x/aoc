use regex::Regex;

use std::collections::HashMap;
use std::fs;
use std::ops::RangeInclusive;

fn get_aunt(input: &str, gift: &HashMap<&str, RangeInclusive<u32>>, regex_compounds: &Regex, regex_num: &Regex) -> u32 {
    input
        .lines()
        .filter(|&line| regex_compounds.captures_iter(line).all(|cap| gift[&cap[1]].contains(&cap[2].parse().unwrap())))
        .map(|line| regex_num.captures(line).and_then(|cap| cap[1].parse().ok()).unwrap())
        .next()
        .unwrap()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("inputs/2015-day16.txt")?;

    let regex_compounds = Regex::new(r#"(children|cats|samoyeds|pomeranians|akitas|vizslas|goldfish|trees|cars|perfumes): (\d+)(?:, )?"#).unwrap();
    let regex_num = Regex::new(r#"^Sue (\d+): "#).unwrap();

    let mut gift = <HashMap<_, _>>::from_iter([
        ("children", 3..=3),
        ("cats", 7..=7),
        ("samoyeds", 2..=2),
        ("pomeranians", 3..=3),
        ("akitas", 0..=0),
        ("vizslas", 0..=0),
        ("goldfish", 5..=5),
        ("trees", 3..=3),
        ("cars", 2..=2),
        ("perfumes", 1..=1),
    ]);

    let result1 = get_aunt(&input, &gift, &regex_compounds, &regex_num);

    *gift.get_mut("cats").unwrap() = (gift["cats"].start() + 1)..=u32::MAX;
    *gift.get_mut("trees").unwrap() = (gift["trees"].start() + 1)..=u32::MAX;
    *gift.get_mut("pomeranians").unwrap() = 0..=(gift["pomeranians"].end() - 1);
    *gift.get_mut("goldfish").unwrap() = 0..=(gift["goldfish"].end() - 1);

    let result2 = get_aunt(&input, &gift, &regex_compounds, &regex_num);

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
