use aoc::*;

use itertools::Itertools;
use regex::Regex;

fn inverse_modulo(x: i64, m: i64) -> i64 {
    let (mut r1, mut u1, mut r2, mut u2) = (x, 1, m, 0);

    while r2 != 0 {
        let q = r1 / r2;
        (r1, u1, r2, u2) = (r2, u2, r1 - q * r2, u1 - q * u2);
    }

    u1.rem_euclid(m)
}

fn chinese_remainder_theorem(modulos_remainders: &[(i64, i64)]) -> i64 {
    let product = modulos_remainders
        .iter()
        .map(|&(modulo, _)| modulo)
        .product();

    modulos_remainders
        .iter()
        .map(|&(modulo, remainder)| {
            let prod_other = product / modulo;
            remainder * inverse_modulo(prod_other, modulo) * prod_other
        })
        .sum::<i64>()
        .rem_euclid(product)
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let re = Regex::new(
        r#"(?m)^Disc #(\d+) has (\d+) positions; at time=0, it is at position (\d+).$"#,
    )?;

    let mut modulos_remainders: Vec<_> = re
        .captures_iter(&input)
        .map(|cap| {
            let num_disc: i64 = cap[1].parse()?;
            let modulo: i64 = cap[2].parse()?;
            let initial_position: i64 = cap[3].parse()?;
            let remainder = -num_disc - initial_position;

            Result::Ok((modulo, remainder))
        })
        .try_collect()?;

    let result1 = chinese_remainder_theorem(&modulos_remainders);

    let new_modulo = 11;
    let new_initial_position = 0;
    let new_num_disc = modulos_remainders.len() as i64 + 1;
    modulos_remainders.push((new_modulo, -new_num_disc - new_initial_position));

    let result2 = chinese_remainder_theorem(&modulos_remainders);

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
