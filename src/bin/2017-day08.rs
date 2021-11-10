use eyre::{bail, Result};
use regex::Regex;

use std::collections::HashMap;
use std::fs;

fn main() -> Result<()> {
    let input = fs::read_to_string("inputs/2017-day08.txt")?;

    let re = Regex::new(r#"(?m)^(\w+) (inc|dec) (-?\d+) if (\w+) ([<>!=]+) (-?\d+)$"#)?;

    let mut registers = HashMap::<_, i64>::new();
    let mut max_value = 0;

    for cap in re.captures_iter(&input) {
        let condition_register = *registers.entry(cap.get(4).unwrap().as_str()).or_default();
        let condition_value = cap[6].parse::<i64>()?;

        if match &cap[5] {
            "<" => condition_register < condition_value,
            ">" => condition_register > condition_value,
            "<=" => condition_register <= condition_value,
            ">=" => condition_register >= condition_value,
            "==" => condition_register == condition_value,
            "!=" => condition_register != condition_value,
            other => bail!("unknown instruction: {}", other),
        } {
            let lvalue = registers.entry(cap.get(1).unwrap().as_str()).or_default();
            let increment = cap[3].parse::<i64>()?;
            let multiplier = match &cap[2] {
                "inc" => 1,
                "dec" => -1,
                other => bail!("unknown instruction: {}", other),
            };

            *lvalue += multiplier * increment;

            if *lvalue > max_value {
                max_value = *lvalue;
            }
        }
    }

    let result1 = *registers.values().max().unwrap();
    let result2 = max_value;

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
