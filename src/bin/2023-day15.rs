use aoc::*;

use itertools::Itertools;

use std::collections::VecDeque;
use std::iter;

fn hash(s: &str) -> u64 {
    (s.bytes()).fold(0u64, |acc, x| ((acc + x as u64) * 17) & 0xFF)
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);
    let input = input.trim();

    let instructions: Vec<_> = input
        .split(',')
        .map(|s| {
            if let Some(label) = s.strip_suffix('-') {
                Result::Ok((s, label, None))
            } else {
                let (label, value) = s.split('=').next_tuple().value()?;
                let value = value.parse::<u64>()?;
                Result::Ok((s, label, Some(value)))
            }
        })
        .try_collect()?;

    let result1 = instructions.iter().map(|(s, ..)| hash(s)).sum::<u64>();

    let mut boxes = vec![VecDeque::new(); 256];

    for &(_, label, value) in &instructions {
        let label_box = &mut boxes[hash(label) as usize];
        let index = (label_box.iter()).position(|&(current_label, _)| current_label == label);

        match (value, index) {
            (Some(value), Some(index)) => label_box[index].1 = value,
            (Some(value), None) => label_box.push_back((label, value)),
            (None, Some(index)) => {
                label_box.remove(index);
            }
            (None, None) => (),
        }
    }

    let result2 = iter::zip(1.., &boxes)
        .map(|(box_idx, label_box)| {
            let box_sum = iter::zip(1.., label_box.iter())
                .map(|(slot_idx, (_, value))| slot_idx * value)
                .sum::<u64>();

            box_idx * box_sum
        })
        .sum::<u64>();

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
