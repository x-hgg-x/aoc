use eyre::Result;
use itertools::Itertools;

use std::fs;
use std::ops::Deref;
use std::str;

fn main() -> Result<()> {
    let mut input = fs::read("inputs/2018-day02.txt")?;

    let ids = input.split_mut(|x| !x.is_ascii_alphabetic()).filter(|x| !x.is_empty()).collect_vec();

    let result2 = ids
        .iter()
        .tuple_combinations()
        .find_map(|(id1, id2)| {
            let mut iter = id1.iter().zip(id2.deref()).enumerate().filter(|(_, (&x, &y))| x != y);
            match (iter.next(), iter.next()) {
                (Some((index, _)), None) => {
                    let mut s = String::new();
                    s += str::from_utf8(&id1[..index]).unwrap();
                    s += str::from_utf8(&id1[index + 1..]).unwrap();
                    Some(s)
                }
                _ => None,
            }
        })
        .unwrap();

    let mut double_count = 0usize;
    let mut triple_count = 0usize;

    for id in ids {
        id.sort_unstable();

        let mut double = false;
        let mut triple = false;

        for (count, _) in id.iter().dedup_with_count() {
            match count {
                2 => double = true,
                3 => triple = true,
                _ => (),
            }
        }

        double_count += double as usize;
        triple_count += triple as usize;
    }

    let result1 = double_count * triple_count;

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
