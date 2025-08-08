use aoc::*;

use itertools::Itertools;

fn knot_hash_round(
    list: &mut [u8],
    lengths: &[usize],
    current_position: &mut usize,
    skip: &mut usize,
) {
    let size = list.len();

    for &len in lengths {
        if len >= 2 {
            let offset = *current_position % size;
            list.rotate_left(offset);
            list[..len].reverse();
            list.rotate_right(offset);
        }

        *current_position += len + *skip;
        *skip += 1;
    }
}

fn knot_hash(input: &[u8]) -> Result<String> {
    let lengths = input
        .iter()
        .map(|&x| x as usize)
        .chain([17, 31, 73, 47, 23])
        .collect_vec();

    let mut list = (0..=u8::MAX).collect_vec();
    let mut current_position = 0;
    let mut skip = 0;

    for _ in 0..64 {
        knot_hash_round(&mut list, &lengths, &mut current_position, &mut skip);
    }

    list.chunks_exact(16)
        .map(|elem| {
            let out = elem.iter().fold(0, |acc, x| acc ^ x);
            let char1 = char::from_digit((out >> 4) as u32, 16).value()?;
            let char2 = char::from_digit((out & 0x0F) as u32, 16).value()?;
            Ok([char1, char2])
        })
        .try_process(|iter| iter.flatten().collect())
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);
    let input = input.trim();

    let lengths: Vec<usize> = input.split(',').map(|x| x.parse()).try_collect()?;

    let mut list = (0..=u8::MAX).collect_vec();
    knot_hash_round(&mut list, &lengths, &mut 0, &mut 0);

    let result1 = list[..2].iter().copied().map_into::<u64>().product::<u64>();
    let result2 = knot_hash(input.as_bytes())?;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
