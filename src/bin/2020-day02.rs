use aoc::*;

use eyre::ensure;
use regex::Regex;

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let re = Regex::new(r#"(?m)^(\d+)-(\d+) (\w): (.+?)$"#)?;

    let mut count1 = 0usize;
    let mut count2 = 0usize;

    for cap in re.captures_iter(&input) {
        let n1 = cap[1].parse()?;
        let n2 = cap[2].parse()?;
        let letter = cap[3].as_bytes()[0];
        let password = cap[4].as_bytes();

        ensure!(n1 != 0 && n2 != 0, "invalid positions");

        if (n1..=n2).contains(&password.iter().filter(|&&x| x == letter).count()) {
            count1 += 1;
        }

        if (*password.get(n1 - 1).value()? == letter) ^ (*password.get(n2 - 1).value()? == letter) {
            count2 += 1;
        }
    }

    let result1 = count1;
    let result2 = count2;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
