use aoc::*;

use itertools::Itertools;

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let numbers: Vec<u32> = input.trim().split(',').map(|x| x.parse()).try_collect()?;

    let mut spoken_numbers = vec![0; 30_000_001];
    for (index, &number) in (1..).zip(&numbers[..numbers.len() - 1]) {
        spoken_numbers[number as usize] = index;
    }

    let mut iter = (numbers.len() as u32..).scan(*numbers.last().value()?, |last_number, index| {
        let old_index = &mut spoken_numbers[*last_number as usize];
        *last_number = if *old_index > 0 { index - *old_index } else { 0 };
        *old_index = index;
        Some(*last_number)
    });

    let result1 = iter.nth(2019 - numbers.len()).value()?;
    let result2 = iter.nth(29_997_979).value()?;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
