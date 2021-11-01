use itertools::Itertools;
use regex::bytes::Regex;

use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("inputs/2015-day19.txt")?;
    let input = input.trim().as_bytes().to_vec();

    let regex_replacements = Regex::new(r#"(?m)^(\w+) => (\w+)$"#).unwrap();
    let regex_molecule = Regex::new(r#"(\w+)$"#).unwrap();

    let replacements = regex_replacements.captures_iter(&input).map(|cap| (cap.get(1).unwrap().as_bytes(), cap.get(2).unwrap().as_bytes())).collect_vec();
    let molecule = regex_molecule.find(&input).map(|x| x.as_bytes()).unwrap();

    let result1 = replacements
        .iter()
        .flat_map(|&(old, new)| {
            Regex::new(&String::from_utf8_lossy(old))
                .unwrap()
                .find_iter(molecule)
                .map(|x| {
                    let mut molecule = molecule.to_vec();
                    molecule.splice(x.range(), new.iter().copied()).last();
                    molecule
                })
                .collect_vec()
        })
        .sorted_unstable()
        .dedup()
        .count();

    let molecule = String::from_utf8_lossy(molecule);
    let length = molecule.matches(|c: char| c.is_ascii_uppercase()).count();
    let num_y = molecule.matches('Y').count();
    let num_rn_ar = regex::Regex::new("Rn|Ar")?.find_iter(&molecule).count();

    let result2 = length - num_rn_ar - 2 * num_y - 1;

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
