use aoc::*;

use itertools::Itertools;
use regex::bytes::Regex;

fn compute_new_recipes(scoreboard: &mut Vec<u8>, current_recipes: &mut [usize; 2]) {
    let scores = [scoreboard[current_recipes[0]], scoreboard[current_recipes[1]]];
    let new_score = scores[0] + scores[1];

    if new_score >= 10 {
        scoreboard.extend_from_slice(&[1, new_score - 10]);
    } else {
        scoreboard.push(new_score);
    }

    current_recipes[0] = (current_recipes[0] + scores[0] as usize + 1) % scoreboard.len();
    current_recipes[1] = (current_recipes[1] + scores[1] as usize + 1) % scoreboard.len();
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);
    let input = input.trim();

    let recipes_count = input.parse::<usize>()?;

    let mut current_recipes = [0, 1];
    let mut scoreboard = vec![3u8, 7u8];

    while scoreboard.len() < recipes_count + 10 {
        compute_new_recipes(&mut scoreboard, &mut current_recipes);
    }

    let result1: String = scoreboard[recipes_count..recipes_count + 10].iter().filter_map(|&score| char::from_digit(score.into(), 10)).collect();

    let sub_slice = input.bytes().map(|x| x - b'0').collect_vec();
    let re = Regex::new(&String::from_utf8_lossy(&sub_slice))?;

    let mut search_start_index = 0;
    let result2 = loop {
        match re.find(&scoreboard[search_start_index..]) {
            Some(m) => break search_start_index + m.start(),
            None => {
                search_start_index = scoreboard.len() - sub_slice.len();
                for _ in 0..1000 * sub_slice.len() {
                    compute_new_recipes(&mut scoreboard, &mut current_recipes);
                }
            }
        }
    };

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
