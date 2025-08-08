use aoc::*;

use itertools::Itertools;
use smallvec::SmallVec;

use std::collections::hash_map::{Entry, HashMap};

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let food_list: Vec<(Vec<_>, SmallVec<[_; 3]>)> = input
        .lines()
        .map(|line| {
            let (ingredients_input, allergens_input) =
                line.split("(contains").next_tuple().value()?;

            let ingredients = ingredients_input
                .split(|c: char| !c.is_ascii_lowercase())
                .filter(|x| !x.is_empty())
                .sorted_unstable()
                .collect();

            let allergens = allergens_input
                .split(|c: char| !c.is_ascii_lowercase())
                .filter(|x| !x.is_empty())
                .collect();

            Result::Ok((ingredients, allergens))
        })
        .try_collect()?;

    let mut possible_allergenic_ingredients = HashMap::<_, Vec<_>>::new();

    for (ingredients, allergens) in &food_list {
        for &allergen in allergens {
            match possible_allergenic_ingredients.entry(allergen) {
                Entry::Vacant(entry) => {
                    entry.insert(ingredients.clone());
                }
                Entry::Occupied(mut entry) => {
                    let list = entry.get_mut();

                    if list.len() > 1 {
                        let mut ingredients_iter = ingredients.iter();

                        list.retain(|&x| {
                            ingredients_iter
                                .take_while_ref(|&&item| item <= x)
                                .any(|&item| item == x)
                        });
                    }
                }
            }
        }
    }

    let mut allergenic_ingredients = Vec::new();

    while !possible_allergenic_ingredients.is_empty() {
        let (&allergen, ingredients) = possible_allergenic_ingredients
            .iter_mut()
            .find(|(_, v)| v.len() == 1)
            .value()?;

        let ingredient = *ingredients.first().value()?;

        possible_allergenic_ingredients.remove(allergen);

        possible_allergenic_ingredients
            .values_mut()
            .for_each(|ingredients| ingredients.retain(|&x| x != ingredient));

        allergenic_ingredients.push((ingredient, allergen));
    }

    allergenic_ingredients.sort_unstable_by_key(|&(ingredient, _)| ingredient);

    let mut unique_safe_ingredients = food_list
        .iter()
        .flat_map(|(ingredients, _)| ingredients)
        .copied()
        .sorted_unstable()
        .dedup_with_count()
        .collect_vec();

    let mut allergenic_ingredients_iter = allergenic_ingredients
        .iter()
        .map(|&(ingredient, _)| ingredient);

    unique_safe_ingredients.retain(|&(_, x)| {
        !allergenic_ingredients_iter
            .take_while_ref(|&item| item <= x)
            .any(|item| item == x)
    });

    let result1 = unique_safe_ingredients
        .iter()
        .map(|&(count, _)| count)
        .sum::<usize>();

    allergenic_ingredients.sort_unstable_by_key(|&(_, allergen)| allergen);

    let result2 = allergenic_ingredients
        .iter()
        .map(|&(ingredient, _)| ingredient)
        .join(",");

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
