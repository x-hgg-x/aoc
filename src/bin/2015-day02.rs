use aoc::*;

use itertools::Itertools;

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let edges: Vec<_> = input
        .lines()
        .flat_map(|line| line.split('x').next_tuple())
        .map(|(x, y, z)| Result::Ok([x.parse()?, y.parse()?, z.parse()?]))
        .try_collect()?;

    let result1: i64 = edges
        .iter()
        .map(|edge| {
            let surfaces = edge
                .iter()
                .tuple_combinations()
                .map(|(side1, side2)| side1 * side2)
                .collect_vec();

            let sum: i64 = surfaces.iter().sum();
            let min: i64 = surfaces.into_iter().min().value()?;

            Ok(2 * sum + min)
        })
        .try_sum()?;

    let result2: i64 = edges
        .iter()
        .map(|edge| {
            let sum: i64 = edge.iter().sum();
            let product: i64 = edge.iter().product();
            let max = edge.iter().max().value()?;

            Ok(2 * (sum - max) + product)
        })
        .try_sum()?;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
