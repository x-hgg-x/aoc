use eyre::Result;

use std::fs;

fn get_min_house(min_presents: usize, presents_by_house: usize, max_houses_by_elf: usize) -> usize {
    let max_house = min_presents / presents_by_house;
    let mut houses = vec![0; max_house];
    for i in 1..max_house {
        for house in houses.iter_mut().step_by(i).skip(1).take(max_houses_by_elf) {
            *house += i * presents_by_house;
        }
    }
    houses.iter().position(|&x| x >= min_presents).unwrap()
}

fn main() -> Result<()> {
    let input = fs::read_to_string("inputs/2015-day20.txt")?;

    let min_presents: usize = input.trim().parse()?;

    let result1 = get_min_house(min_presents, 10, usize::MAX);
    let result2 = get_min_house(min_presents, 11, 50);

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
