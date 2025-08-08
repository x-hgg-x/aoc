use aoc::*;

use itertools::Itertools;

use std::iter;

fn generator(subject_number: u64) -> impl Iterator<Item = u64> {
    iter::successors(Some(1), move |x| Some((x * subject_number) % 20201227))
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let (card_public_key, door_public_key) = input
        .split_ascii_whitespace()
        .map(|x| Ok(x.parse::<u64>()?))
        .try_process(|mut iter| iter.next_tuple())?
        .value()?;

    let door_loop_size = generator(7)
        .enumerate()
        .scan((0, 0), |(card_loop_size, door_loop_size), (index, x)| {
            if *card_loop_size == 0 || *door_loop_size == 0 {
                if x == card_public_key {
                    *card_loop_size = index;
                }
                if x == door_public_key {
                    *door_loop_size = index;
                }
                Some(*door_loop_size)
            } else {
                None
            }
        })
        .last()
        .value()?;

    let result = generator(card_public_key).nth(door_loop_size).value()?;

    println!("{result}");
    Ok(())
}
