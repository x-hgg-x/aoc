use regex::Regex;

use std::fs;

fn euclide_inverse(a: i64, b: i64) -> (i64, i64) {
    let (mut r1, mut u1, mut v1, mut r2, mut u2, mut v2) = (a, 1, 0, b, 0, 1);

    while r2 != 0 {
        let q = r1 / r2;
        let (r1_old, u1_old, v1_old) = (r1, u1, v1);

        r1 = r2;
        u1 = u2;
        v1 = v2;

        r2 = r1_old - q * r2;
        u2 = u1_old - q * u2;
        v2 = v1_old - q * v2;
    }

    assert_eq!(r1, 1, "inputs must be coprimes");

    (u1, v1)
}

fn chinese_remainder_theorem(modulos: &[i64], remainders: &[i64]) -> i64 {
    let product: i64 = modulos.iter().product();

    let solution: i64 = modulos
        .iter()
        .zip(remainders)
        .map(|(&modulo, &remainder)| {
            let prod_other = product / modulo;
            let (u, _) = euclide_inverse(prod_other, modulo);

            remainder * u * prod_other
        })
        .sum();

    solution.rem_euclid(product)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("inputs/2016-day15.txt")?;

    let re = Regex::new(r#"(?m)^Disc #(\d+) has (\d+) positions; at time=0, it is at position (\d+).$"#).unwrap();

    let (mut modulos, mut remainders): (Vec<_>, Vec<_>) = re
        .captures_iter(&input)
        .map(|cap| {
            let num_disc: i64 = cap[1].parse().unwrap();
            let modulo: i64 = cap[2].parse().unwrap();
            let initial_position: i64 = cap[3].parse().unwrap();
            let remainder = -num_disc - initial_position;

            (modulo, remainder)
        })
        .unzip();

    let result1 = chinese_remainder_theorem(&modulos, &remainders);

    let new_modulo = 11;
    let new_initial_position = 0;
    let new_num_disc = remainders.len() as i64 + 1;
    modulos.push(new_modulo);
    remainders.push(-new_num_disc - new_initial_position);

    let result2 = chinese_remainder_theorem(&modulos, &remainders);

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
