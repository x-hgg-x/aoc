use itertools::Itertools;
use smallvec::SmallVec;

use std::fs;

struct LookAndSay {
    data: Vec<u8>,
}

impl LookAndSay {
    fn new(number: &str) -> Self {
        Self {
            data: number
                .chars()
                .map(|x| x.to_digit(10).unwrap() as u8)
                .collect(),
        }
    }

    fn next(&mut self, n: u32) -> &[u8] {
        for _ in 0..n {
            self.data = self
                .data
                .iter()
                .dedup_with_count()
                .flat_map(|(count, digit)| SmallVec::from_buf([count as u8, *digit]))
                .collect();
        }

        &self.data
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("inputs/2015-day10.txt")?;
    let input = input.trim();

    let mut look_and_say = LookAndSay::new(&input);

    let result1 = look_and_say.next(40).len();
    let result2 = look_and_say.next(10).len();

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
