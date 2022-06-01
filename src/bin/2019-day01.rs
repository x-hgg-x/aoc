use aoc::*;

use itertools::Itertools;

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let list: Vec<i64> = input.split_ascii_whitespace().map(|x| x.parse()).try_collect()?;

    let result1 = list.iter().map(|x| x / 3 - 2).sum::<i64>();

    let result2 = list
        .iter()
        .map(|x| {
            let mut fuel = x / 3 - 2;
            let mut total_fuel = fuel;

            loop {
                let additional_fuel = fuel / 3 - 2;
                if additional_fuel <= 0 {
                    break;
                }
                fuel = additional_fuel;
                total_fuel += additional_fuel;
            }
            total_fuel
        })
        .sum::<i64>();

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
