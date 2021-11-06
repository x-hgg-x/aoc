use eyre::Result;
use itertools::Itertools;

use std::fs;

struct LookAndSay {
    data: Vec<u8>,
}

impl LookAndSay {
    fn new(number: &[u8]) -> Self {
        Self { data: number.iter().map(|&x| x - b'0').collect() }
    }

    fn next(&mut self, n: u32) -> &[u8] {
        for _ in 0..n {
            self.data = self.data.iter().dedup_with_count().flat_map(|(count, digit)| [count as u8, *digit]).collect();
        }

        &self.data
    }
}

fn main() -> Result<()> {
    let input = fs::read_to_string("inputs/2015-day10.txt")?;
    let input = input.trim().as_bytes();

    let mut look_and_say = LookAndSay::new(input);

    let result1 = look_and_say.next(40).len();
    let result2 = look_and_say.next(10).len();

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
