use aoc::*;

use std::collections::hash_map::{Entry, HashMap};

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);
    let input = input.trim().as_bytes();

    let mut states = Vec::new();
    let mut distances = HashMap::new();
    let mut current_position = (0i64, 0i64);
    let mut current_distance = 0i64;

    for &c in input {
        match c {
            b'(' => states.push((current_position, current_distance)),
            b')' => {
                (current_position, current_distance) = states.pop().value()?;
            }
            b'|' => {
                (current_position, current_distance) = *states.last().value()?;
            }
            _ => {
                match c {
                    b'N' => {
                        current_position.1 += 1;
                    }
                    b'S' => {
                        current_position.1 -= 1;
                    }
                    b'E' => {
                        current_position.0 += 1;
                    }
                    b'W' => {
                        current_position.0 -= 1;
                    }
                    _ => continue,
                };

                current_distance += 1;

                match distances.entry(current_position) {
                    Entry::Vacant(entry) => {
                        entry.insert(current_distance);
                    }
                    Entry::Occupied(mut entry) => {
                        *entry.get_mut() = current_distance.min(*entry.get());
                    }
                };
            }
        }
    }

    let result1 = *distances.values().max().value()?;

    let result2 = distances
        .values()
        .filter(|&&distance| distance >= 1000)
        .count();

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
