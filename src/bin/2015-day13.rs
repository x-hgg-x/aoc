use aoc::*;

use itertools::Itertools;
use regex::Regex;
use smallvec::SmallVec;

use std::collections::HashMap;
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

impl<'a, T: Copy, const N: usize> Iterator for Permutations<'a, T, N> {
    type Item = SmallVec<[T; N]>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.factorial_index >= self.factorials[self.data.len()] {
            return None;
        }

        let mut x = self.factorial_index;

        self.buf.clear();
        self.available = SmallVec::from_slice(self.data);

        self.buf.extend(self.factorials[..self.data.len()].iter().rev().map(|&place_value| {
            let index = x / place_value;
            x -= index * place_value;
            self.available.remove(index.rem_euclid(self.available.len() as i64) as usize)
        }));

        self.factorial_index += 1;

        Some(self.buf.clone())
    }
}

fn max_hapiness(nodes: &[&str], edges: &HashMap<(&str, &str), i64>) -> Result<i64> {
    Permutations::<_, 9>::new(nodes)
        .map(|mut x| {
            x.push(x[0]);
            x.windows(2).map(|x| Ok(edges.get(&(x[0], x[1])).value()? + edges.get(&(x[1], x[0])).value()?)).try_sum()
        })
        .try_process(|iter| iter.max())?
        .value()
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let re = Regex::new(r#"(?m)^(\w+) would (lose|gain) (\d+).*?(\w+).$"#)?;

    let mut nodes: Vec<_> = re.captures_iter(&input).flat_map(|cap| [cap.get(1), cap.get(4)]).map(|m| Result::Ok(m.value()?.as_str())).try_collect()?;
    nodes.sort_unstable();
    nodes.dedup();

    let mut edges = re
        .captures_iter(&input)
        .map(|cap| {
            let node1 = cap.get(1).value()?.as_str();
            let node2 = cap.get(4).value()?.as_str();

            let action = match &cap[2] {
                "lose" => -1,
                "gain" => 1,
                _ => 0,
            };
            let amount = cap[3].parse::<i64>()?;
            let happiness = action * amount;

            Result::Ok(((node1, node2), happiness))
        })
        .try_collect()?;

    let result1 = max_hapiness(&nodes, &edges)?;

    edges.extend(nodes.iter().flat_map(|&node| [(("Me", node), 0), ((node, "Me"), 0)]));
    nodes.push("Me");

    let result2 = max_hapiness(&nodes, &edges)?;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
