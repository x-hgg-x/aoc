use aoc::*;

use itertools::Itertools;
use regex::bytes::Regex;

fn main() -> Result<()> {
    let input = setup(file!())?;

    let regex_replacements = Regex::new(r#"(?m)^(\w+) => (\w+)$"#)?;
    let regex_molecule = Regex::new(r#"(?m)^(\w+)$"#)?;

    let replacements: Vec<_> = regex_replacements
        .captures_iter(&input)
        .map(|cap| {
            let regex_old = Regex::new(&String::from_utf8_lossy(&cap[1]))?;
            let new = cap.get(2).value()?.as_bytes();
            Result::Ok((regex_old, new))
        })
        .try_collect()?;

    let molecule = regex_molecule.find(&input).map(|x| x.as_bytes()).value()?;

    let result1 = replacements
        .iter()
        .flat_map(|(regex_old, new)| {
            regex_old.find_iter(molecule).map(move |m| {
                molecule[..m.start()]
                    .iter()
                    .chain(&**new)
                    .chain(&molecule[m.end()..])
                    .copied()
                    .collect_vec()
            })
        })
        .sorted_unstable()
        .dedup()
        .count();

    let molecule = String::from_utf8_lossy(molecule);
    let length = molecule.matches(|c: char| c.is_ascii_uppercase()).count();
    let num_y = molecule.matches('Y').count();
    let num_rn_ar = regex::Regex::new("Rn|Ar")?.find_iter(&molecule).count();

    let result2 = length - num_rn_ar - 2 * num_y - 1;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
