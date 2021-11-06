use eyre::Result;
use itertools::Itertools;
use regex::Regex;
use smallvec::SmallVec;

use std::collections::HashMap;
use std::fs;
use std::iter::once;

struct Permutations<'a, T, const N: usize> {
    data: &'a [T],
    available: SmallVec<[T; N]>,
    buf: SmallVec<[T; N]>,
    factorials: Vec<i64>,
    factorial_index: i64,
}

impl<'a, T, const N: usize> Permutations<'a, T, N> {
    fn new(data: &'a [T]) -> Self {
        Self { data, available: SmallVec::new(), buf: SmallVec::new(), factorials: Self::compute_factorials(data.len() as i64), factorial_index: 0 }
    }

    fn compute_factorials(num: i64) -> Vec<i64> {
        once(1)
            .chain((1..=num).scan(1, |state, x| {
                *state *= x;
                Some(*state)
            }))
            .collect_vec()
    }
}

impl<'a, T: Clone, const N: usize> Iterator for Permutations<'a, T, N> {
    type Item = SmallVec<[T; N]>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.factorial_index >= self.factorials[self.data.len()] {
            return None;
        }

        let mut x = self.factorial_index;

        self.buf.clear();
        self.available = SmallVec::from(self.data);

        self.buf.extend(self.factorials[..self.data.len()].iter().rev().map(|&place_value| {
            let index = x / place_value;
            x -= index * place_value;
            self.available.remove(index.rem_euclid(self.available.len() as i64) as usize)
        }));

        self.factorial_index += 1;

        Some(self.buf.clone())
    }
}

fn main() -> Result<()> {
    let input = fs::read_to_string("inputs/2015-day09.txt")?;

    let re = Regex::new(r#"(?m)^(\w+) to (\w+) = (\d+)$"#)?;

    let nodes = re.captures_iter(&input).flat_map(|cap| [cap.get(1).unwrap().as_str(), cap.get(2).unwrap().as_str()]).sorted_unstable().dedup().collect_vec();

    let edges = HashMap::<_, _>::from_iter(re.captures_iter(&input).flat_map(|cap| {
        let location1 = cap.get(1).unwrap().as_str();
        let location2 = cap.get(2).unwrap().as_str();
        let distance: u32 = cap[3].parse().unwrap();
        [((location1, location2), distance), ((location2, location1), distance)]
    }));

    let distances = Permutations::<_, 8>::new(&nodes).map(|x| x.windows(2).map(|x| edges[&(x[0], x[1])]).sum::<u32>()).collect_vec();

    let result1 = *distances.iter().min().unwrap();
    let result2 = *distances.iter().max().unwrap();

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
