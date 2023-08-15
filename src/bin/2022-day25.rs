use std::fmt::{self, Display, Write};

use aoc::*;

struct Snafu(Vec<i8>);

impl Snafu {
    fn from_decimal(mut value: i64) -> Self {
        let mut digits = Vec::new();

        while value != 0 {
            digits.push((value + 2).rem_euclid(5) as i8 - 2);
            value = (value + 2) / 5;
        }

        Self(digits)
    }
}

impl Display for Snafu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for &digit in self.0.iter().rev() {
            match digit {
                -2 => f.write_char(b'=' as char)?,
                -1 => f.write_char(b'-' as char)?,
                _ => f.write_char((b'0' + digit as u8) as char)?,
            };
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let sum = input
        .lines()
        .map(|line| {
            let mut value = 0;
            line.bytes()
                .map(|x| match x {
                    b'=' => -2,
                    b'-' => -1,
                    _ => (x - b'0') as i64,
                })
                .for_each(|digit| {
                    value *= 5;
                    value += digit;
                });
            value
        })
        .sum::<i64>();

    let result = Snafu::from_decimal(sum).to_string();

    println!("{result}");
    Ok(())
}
