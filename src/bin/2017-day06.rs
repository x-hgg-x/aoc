use eyre::Result;
use itertools::Itertools;
use smallvec::SmallVec;

use std::collections::HashMap;
use std::fs;

fn main() -> Result<()> {
    let input = fs::read_to_string("inputs/2017-day06.txt")?;

    let mut banks: SmallVec<[_; 16]> = input.split_ascii_whitespace().map(|x| x.parse::<usize>()).try_collect()?;
    let size = banks.len() as usize;

    let mut previous_states = HashMap::new();
    let mut count = 0usize;

    let old_count = loop {
        match previous_states.entry(banks.clone()) {
            std::collections::hash_map::Entry::Occupied(entry) => break *entry.get(),
            std::collections::hash_map::Entry::Vacant(entry) => entry.insert(count),
        };

        let (index_max, &max) = banks.iter().enumerate().rev().max_by_key(|(_, &x)| x).unwrap();

        let q = max / size;
        let r = max % size;

        banks[index_max] = 0;
        for (idx, block) in banks.iter_mut().enumerate() {
            let i = (idx + size - (index_max + 1)) % size;
            *block += q + (i < r) as usize
        }
        count += 1;
    };

    let result1 = count;
    let result2 = count - old_count;

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
