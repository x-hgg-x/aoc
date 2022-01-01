use aoc::*;

use itertools::Itertools;

use std::collections::HashSet;

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);
    let input = input.lines().collect_vec();

    let v1 = input[7].split_ascii_whitespace().nth(2).value()?.parse::<i64>()?;
    let v2 = input[8].split_ascii_whitespace().nth(1).value()?.parse::<i64>()?;
    let v3 = input[9].split_ascii_whitespace().nth(2).value()?.parse::<i64>()?;
    let v4 = input[11].split_ascii_whitespace().nth(2).value()?.parse::<i64>()?;
    let v5 = input[12].split_ascii_whitespace().nth(2).value()?.parse::<i64>()?;
    let v6 = input[13].split_ascii_whitespace().nth(2).value()?.parse::<i64>()?;
    let v7 = input[14].split_ascii_whitespace().nth(1).value()?.parse::<i64>()?;

    let mut first_value = None;
    let mut last_value = None;
    let mut previous_values = HashSet::new();

    let mut d = 0;

    'run: loop {
        let mut e = d | v1;
        d = v2;

        loop {
            d += e & v3;
            d &= v4;
            d *= v5;
            d &= v6;

            if v7 > e {
                if first_value.is_none() {
                    first_value = Some(d);
                }

                if previous_values.insert(d) {
                    last_value = Some(d);
                    break;
                } else {
                    break 'run;
                }
            }

            e >>= 8;
        }
    }

    let result1 = first_value.value()?;
    let result2 = last_value.value()?;

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
