use aoc::*;

use regex::Regex;

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let regexes = [
        Regex::new(r#"one|1"#)?,
        Regex::new(r#"two|2"#)?,
        Regex::new(r#"three|3"#)?,
        Regex::new(r#"four|4"#)?,
        Regex::new(r#"five|5"#)?,
        Regex::new(r#"six|6"#)?,
        Regex::new(r#"seven|7"#)?,
        Regex::new(r#"eight|8"#)?,
        Regex::new(r#"nine|9"#)?,
    ];

    let mut result1 = 0u64;
    let mut result2 = 0u64;

    for line in input.lines() {
        let mut first_digit = None;
        let mut last_digit = None;
        let mut first_extended_digit = None;
        let mut last_extended_digit = None;

        for (idx, re) in regexes.iter().enumerate() {
            let digit = idx as u64 + 1;

            for m in re.find_iter(line) {
                if m.as_str().bytes().next() == Some(b'0' + digit as u8) {
                    if first_digit.is_none_or(|(start, _)| m.start() < start) {
                        first_digit = Some((m.start(), digit));
                    }
                    if last_digit.is_none_or(|(start, _)| m.start() > start) {
                        last_digit = Some((m.start(), digit));
                    }
                }
                if first_extended_digit.is_none_or(|(start, _)| m.start() < start) {
                    first_extended_digit = Some((m.start(), digit));
                }
                if last_extended_digit.is_none_or(|(start, _)| m.start() > start) {
                    last_extended_digit = Some((m.start(), digit));
                }
            }
        }

        result1 += first_digit.value()?.1 * 10 + last_digit.value()?.1;
        result2 += first_extended_digit.value()?.1 * 10 + last_extended_digit.value()?.1;
    }

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
