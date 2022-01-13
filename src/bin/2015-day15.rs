use aoc::*;

use eyre::ensure;
use itertools::{izip, Itertools};
use regex::Regex;
use smallvec::SmallVec;

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
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let re = Regex::new(r#"capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (-?\d+)"#)?;

    let ingredients: Vec<_> = re
        .captures_iter(&input)
        .map(|cap| {
            let capacity: i64 = cap[1].parse()?;
            let durability: i64 = cap[2].parse()?;
            let flavor: i64 = cap[3].parse()?;
            let texture: i64 = cap[4].parse()?;
            let calories: i64 = cap[5].parse()?;
            Result::Ok([capacity, durability, flavor, texture, calories])
        })
        .try_collect()?;

    let cookies = composition(100, ingredients.len())?
        .map(|amounts| {
            let properties = ingredients.iter().zip(amounts).fold([0; 5], |total, (weight, amount)| {
                let mut sum = [0; 5];
                for (sum, &total, &weight) in izip!(&mut sum, &total, weight) {
                    *sum = total + amount as i64 * weight;
                }
                sum
            });

            let score = properties[..4].iter().map(|&x| x.max(0)).product::<i64>();
            let calories = properties[4];
            (score, calories)
        })
        .collect_vec();

    let result1 = cookies.iter().map(|(score, _)| score).max().value()?;
    let result2 = cookies.iter().filter(|&&(_, calories)| calories == 500).map(|(score, _)| score).max().value()?;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
