use aoc::*;

use regex::Regex;

use std::collections::HashMap;
use std::ops::RangeInclusive;

fn get_aunt(
    input: &str,
    gift: &HashMap<&str, RangeInclusive<u32>>,
    regex_compounds: &Regex,
    regex_num: &Regex,
) -> Result<u32> {
    input
        .lines()
        .find_map(|line| {
            (|| -> Result<_> {
                regex_compounds
                    .captures_iter(line)
                    .map(|cap| Ok((cap.get(1).value()?.as_str(), cap[2].parse()?)))
                    .try_process(|mut iter| iter.all(|(item, count)| gift[item].contains(&count)))?
                    .then(|| Ok(regex_num.captures(line).value()?[1].parse()?))
                    .transpose()
            })()
            .transpose()
        })
        .transpose()?
        .value()
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let regex_compounds = Regex::new(
        r#"(children|cats|samoyeds|pomeranians|akitas|vizslas|goldfish|trees|cars|perfumes): (\d+)(?:, )?"#,
    )?;
    let regex_num = Regex::new(r#"^Sue (\d+): "#)?;

    let mut gift = <HashMap<_, _>>::from([
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

    let result1 = get_aunt(&input, &gift, &regex_compounds, &regex_num)?;

    *gift.get_mut("cats").value()? = (gift["cats"].start() + 1)..=u32::MAX;
    *gift.get_mut("trees").value()? = (gift["trees"].start() + 1)..=u32::MAX;
    *gift.get_mut("pomeranians").value()? = 0..=(gift["pomeranians"].end() - 1);
    *gift.get_mut("goldfish").value()? = 0..=(gift["goldfish"].end() - 1);

    let result2 = get_aunt(&input, &gift, &regex_compounds, &regex_num)?;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
