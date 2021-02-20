use regex::Regex;

use std::fs;

#[derive(Clone)]
struct Composition {
    vec: Vec<usize>,
    start: bool,
}

impl Composition {
    fn new(sum: usize, len: usize) -> Result<Composition, &'static str> {
        if sum < len {
            Err("unable to construct Composition: sum < len")
        } else {
            let mut vec = vec![1; len];
            vec[len - 1] = sum - len + 1;
            Ok(Composition { vec, start: true })
        }
    }
}

impl Iterator for Composition {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        let v = &mut self.vec;
        let len = v.len();

        if self.start {
            self.start = false;
            return Some(v.clone());
        }

        (1..len).rev().find(|&i| v[i] > 1).map(|index| {
            v[index - 1] += 1;
            v[index] -= 1;
            v[len - 1] += v[index..len - 1].iter().sum::<usize>() + index + 1 - len;
            v[index..len - 1].fill(1);
            v.clone()
        })
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("inputs/2015-day15.txt")?;

    let re = Regex::new(r#"capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (-?\d+)"#).unwrap();

    let ingredients: Vec<_> = re
        .captures_iter(&input)
        .map(|cap| -> [i32; 5] {
            let capacity = cap[1].parse().unwrap();
            let durability = cap[2].parse().unwrap();
            let flavor = cap[3].parse().unwrap();
            let texture = cap[4].parse().unwrap();
            let calories = cap[5].parse().unwrap();
            [capacity, durability, flavor, texture, calories]
        })
        .collect();

    let iter = Composition::new(100, ingredients.len())?.map(|amounts| {
        let properties = ingredients
            .iter()
            .zip(amounts)
            .fold([0; 5], |total, (weight, amount)| {
                let mut sum = [0; 5];
                let zip_iter = sum.iter_mut().zip(total.iter()).zip(weight.iter());
                for ((sum, &total), &weight) in zip_iter {
                    *sum = total + amount as i32 * weight;
                }
                sum
            });

        let score: i32 = properties[..4].iter().map(|&x| x.max(0)).product();
        let calories: i32 = properties[4];
        (score, calories)
    });

    let result1 = iter.clone().map(|(score, _)| score).max().unwrap();

    let result2 = iter
        .filter(|&(_, calories)| calories == 500)
        .map(|(score, _)| score)
        .max()
        .unwrap();

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
