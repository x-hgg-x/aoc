use aoc::*;

use itertools::Itertools;

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

    let (first_line, second_line) = input.lines().next_tuple().value()?;

    let timestamp = first_line.parse::<i64>()?;

    let bus_list = second_line
        .split(',')
        .enumerate()
        .filter_map(|(index, x)| x.parse().ok().map(|x| (index as i64, x)))
        .collect_vec();

    let (id, minutes) = bus_list
        .iter()
        .map(|&(_, id)| (id, (-timestamp).rem_euclid(id)))
        .min_by_key(|&(_, x)| x)
        .value()?;

    let result1 = id * minutes;

    let modulos_remainders = bus_list
        .iter()
        .map(|&(index, id)| (id, -index))
        .collect_vec();

    let result2 = chinese_remainder_theorem(&modulos_remainders);

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
