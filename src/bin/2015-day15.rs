use eyre::{ensure, Result};
use itertools::{izip, Itertools};
use regex::Regex;
use smallvec::SmallVec;

use std::fs;

fn composition(sum: usize, len: usize) -> Result<impl Iterator<Item = SmallVec<[usize; 4]>>> {
    ensure!(sum >= len, "invalid parameters: sum < len");

    let mut first = SmallVec::from_elem(1, len);
    first[len - 1] = sum - len + 1;

    Ok(std::iter::successors(Some(first), |vec| {
        let mut v = vec.clone();
        let len = v.len();

        (1..len).rev().find(|&i| v[i] > 1).map(|index| {
            v[index - 1] += 1;
            v[index] -= 1;
            v[len - 1] += v[index..len - 1].iter().sum::<usize>() + index + 1 - len;
            v[index..len - 1].fill(1);
            v
        })
    }))
}

fn main() -> Result<()> {
    let input = fs::read_to_string("inputs/2015-day15.txt")?;

    let re = Regex::new(r#"capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (-?\d+)"#)?;

    let ingredients = re
        .captures_iter(&input)
        .map(|cap| -> [i32; 5] {
            let capacity = cap[1].parse().unwrap();
            let durability = cap[2].parse().unwrap();
            let flavor = cap[3].parse().unwrap();
            let texture = cap[4].parse().unwrap();
            let calories = cap[5].parse().unwrap();
            [capacity, durability, flavor, texture, calories]
        })
        .collect_vec();

    let cookies = composition(100, ingredients.len())?
        .map(|amounts| {
            let properties = ingredients.iter().zip(amounts).fold([0; 5], |total, (weight, amount)| {
                let mut sum = [0; 5];
                for (sum, &total, &weight) in izip!(&mut sum, &total, weight) {
                    *sum = total + amount as i32 * weight;
                }
                sum
            });

            let score: i32 = properties[..4].iter().map(|&x| x.max(0)).product();
            let calories: i32 = properties[4];
            (score, calories)
        })
        .collect_vec();

    let result1 = cookies.iter().map(|(score, _)| score).max().unwrap();
    let result2 = cookies.iter().filter(|&&(_, calories)| calories == 500).map(|(score, _)| score).max().unwrap();

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
