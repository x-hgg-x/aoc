use itertools::Itertools;

use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("inputs/2015-day02.txt")?;

    let edges =
        input.lines().flat_map(|line| line.split('x').next_tuple()).map(|(x, y, z)| [x.parse().unwrap(), y.parse().unwrap(), z.parse().unwrap()]).collect_vec();

    let result1: i32 = edges
        .iter()
        .map(|edge| {
            let surfaces = edge.iter().tuple_combinations().map(|(side1, side2)| side1 * side2).collect_vec();

            let sum: i32 = surfaces.iter().sum();
            let min: i32 = surfaces.into_iter().min().unwrap();

            2 * sum + min
        })
        .sum();

    let result2: i32 = edges
        .iter()
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
