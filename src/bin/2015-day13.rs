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

fn max_hapiness(nodes: &[&str], edges: &HashMap<(&str, &str), i32>) -> i32 {
    Permutations::<_, 9>::new(nodes)
        .map(|mut x| {
            x.push(x[0]);
            x.windows(2).map(|x| edges.get(&(x[0], x[1])).unwrap() + edges.get(&(x[1], x[0])).unwrap()).sum::<i32>()
        })
        .max()
        .unwrap()
}

fn main() -> Result<()> {
    let input = fs::read_to_string("inputs/2015-day13.txt")?;

    let re = Regex::new(r#"(?m)^(\w+) would (lose|gain) (\d+).*?(\w+).$"#)?;

    let mut nodes =
        re.captures_iter(&input).flat_map(|cap| [cap.get(1).unwrap().as_str(), cap.get(4).unwrap().as_str()]).sorted_unstable().dedup().collect_vec();

    let mut edges = re
        .captures_iter(&input)
        .map(|cap| {
            let happiness: i32 = cap[3].parse::<i32>().unwrap()
                * match &cap[2] {
                    "lose" => -1,
                    "gain" => 1,
                    _ => 0,
                };
            ((cap.get(1).unwrap().as_str(), cap.get(4).unwrap().as_str()), happiness)
        })
        .collect();

    let result1 = max_hapiness(&nodes, &edges);

    for &node in &nodes {
        edges.insert(("Me", node), 0);
        edges.insert((node, "Me"), 0);
    }
    nodes.push("Me");

    let result2 = max_hapiness(&nodes, &edges);

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
