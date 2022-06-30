use aoc::*;

use std::collections::hash_map::{Entry, HashMap};

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let mut tiles = HashMap::new();

    for line in input.lines() {
        let position = line.split_inclusive(|c: char| matches!(c, 'e' | 'w')).fold((0i64, 0i64), |(q, r), direction| match direction {
            "e" => (q + 1, r),
            "w" => (q - 1, r),
            "se" => (q, r + 1),
            "nw" => (q, r - 1),
            "sw" => (q - 1, r + 1),
            "ne" => (q + 1, r - 1),
            _ => (q, r),
        });

        match tiles.entry(position) {
            Entry::Occupied(entry) => {
                entry.remove();
            }
            Entry::Vacant(entry) => {
                entry.insert(());
            }
        }
    }

    let result1 = tiles.len();

    let mut buffer = HashMap::new();
    let mut neighbors_count = HashMap::<_, usize>::new();

    for _ in 0..100 {
        buffer.clear();
        neighbors_count.clear();

        for &(q, r) in tiles.keys() {
            for neighbor in [(q + 1, r), (q - 1, r), (q, r + 1), (q, r - 1), (q - 1, r + 1), (q + 1, r - 1)] {
                *neighbors_count.entry(neighbor).or_default() += 1;
            }
        }

        buffer.extend(neighbors_count.iter().filter_map(|(&position, &count)| match (tiles.contains_key(&position), count) {
            (false, 2) => Some((position, ())),
            (true, 1 | 2) => Some((position, ())),
            _ => None,
        }));

        std::mem::swap(&mut buffer, &mut tiles);
    }

    let result2 = tiles.len();

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
