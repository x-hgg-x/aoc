use itertools::Itertools;

use std::fs;

struct LookAndSay {
    data: String,
}

impl Iterator for LookAndSay {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.data = self
            .data
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .dedup_with_count()
            .map(|(count, digit)| format!("{}{}", count, digit))
            .collect();

        Some(self.data.clone())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("inputs/2015-day10.txt")?;
    let input = input.trim();

    let mut look_and_say = LookAndSay {
        data: input.to_owned(),
    };

    let result1 = look_and_say.nth(39).unwrap().len();
    let result2 = look_and_say.nth(9).unwrap().len();

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
