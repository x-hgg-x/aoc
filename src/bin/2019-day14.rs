use aoc::*;

use itertools::Itertools;
use regex::Regex;
use smallvec::SmallVec;

use std::collections::HashMap;

struct Ingredient<'a> {
    name: &'a str,
    quantity: u64,
}

struct Reaction<'a> {
    inputs: SmallVec<[Ingredient<'a>; 8]>,
    output: Ingredient<'a>,
}

struct Buffer<'a> {
    requirements: Vec<Ingredient<'a>>,
    leftovers: HashMap<&'a str, u64>,
}

fn fuel_cost<'a>(reactions: &HashMap<&'a str, Reaction<'a>>, buffer: &mut Buffer<'a>, fuel_amount: u64) -> u64 {
    buffer.requirements.clear();
    buffer.requirements.push(Ingredient { name: "FUEL", quantity: fuel_amount });
    buffer.leftovers.clear();

    let mut ore_cost = 0;

    while let Some(required) = buffer.requirements.pop() {
        if required.name == "ORE" {
            ore_cost += required.quantity;
            continue;
        }

        let leftover = buffer.leftovers.entry(required.name).or_default();

        if *leftover >= required.quantity {
            *leftover -= required.quantity;
            continue;
        }

        let remaining = required.quantity - *leftover;

        let required_reaction = &reactions[required.name];

        let mut required_reaction_count = remaining / required_reaction.output.quantity;
        if remaining % required_reaction.output.quantity != 0 {
            required_reaction_count += 1;
        }

        *leftover = (required_reaction_count * required_reaction.output.quantity) - remaining;

        buffer.requirements.extend(
            required_reaction.inputs.iter().map(|ingredient| Ingredient { name: ingredient.name, quantity: required_reaction_count * ingredient.quantity }),
        );
    }

    ore_cost
}

fn max_fuel<'a>(reactions: &HashMap<&'a str, Reaction<'a>>, buffer: &mut Buffer<'a>, total_ore: u64, unit_cost: u64) -> u64 {
    let mut fuel_left = 0;
    let mut fuel_right = 2 * total_ore / unit_cost;

    while fuel_left + 2 <= fuel_right {
        let fuel = (fuel_left + fuel_right) / 2;

        if fuel_cost(reactions, buffer, fuel) <= total_ore {
            fuel_left = fuel;
        } else {
            fuel_right = fuel;
        }
    }

    fuel_left
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let regex_reaction = Regex::new(r#"(?m)^(.+?) => (.+?)$"#)?;
    let regex_ingredient = Regex::new(r#"(\d+) (\w+)"#)?;

    let reactions: HashMap<_, _> = regex_reaction
        .captures_iter(&input)
        .map(|cap_reaction| {
            let reaction_inputs = regex_ingredient
                .captures_iter(cap_reaction.get(1).value()?.as_str())
                .map(|cap_input| Result::Ok(Ingredient { name: cap_input.get(2).value()?.as_str(), quantity: cap_input[1].parse()? }))
                .try_collect()?;

            let reaction_output = cap_reaction
                .get(2)
                .and_then(|x| x.as_str().split_ascii_whitespace().next_tuple())
                .map(|(count, name)| Result::Ok(Ingredient { name, quantity: count.parse()? }))
                .transpose()?
                .value()?;

            Result::Ok((reaction_output.name, Reaction { inputs: reaction_inputs, output: reaction_output }))
        })
        .try_collect()?;

    let mut buffer = Buffer { requirements: Vec::new(), leftovers: HashMap::with_capacity(1 + reactions.len()) };

    let unit_cost = fuel_cost(&reactions, &mut buffer, 1);
    let max_fuel = max_fuel(&reactions, &mut buffer, 1_000_000_000_000, unit_cost);

    let result1 = unit_cost;
    let result2 = max_fuel;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
