use itertools::Itertools;

use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("inputs/2015-day02.txt")?;

    let edges = input
        .lines()
        .flat_map(|line| line.split('x').next_tuple())
        .map(|(x, y, z)| {
            vec![x.parse(), y.parse(), z.parse()]
                .into_iter()
                .collect::<Result<Vec<i32>, _>>()
                .unwrap()
        });

    let result1: i32 = edges
        .clone()
        .map(|edge| {
            let surfaces = edge
                .iter()
                .combinations(2)
                .map(|x| x.iter().copied().product());

            let sum: i32 = surfaces.clone().sum();
            let min: i32 = surfaces.min().unwrap();

            2 * sum + min
        })
        .sum();

    let result2: i32 = edges
        .map(|edge| {
            let sum: i32 = edge.iter().sum();
            let product: i32 = edge.iter().product();
            let max = edge.iter().max().unwrap();

            2 * (sum - max) + product
        })
        .sum();

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
