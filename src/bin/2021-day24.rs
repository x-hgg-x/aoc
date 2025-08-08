use aoc::*;

use itertools::izip;

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let mut lines = input.lines();

    let mut var0 = [0i64; 14];
    let mut var1 = [0i64; 14];
    let mut var2 = [0i64; 14];

    for (v0, v1, v2) in izip!(&mut var0, &mut var1, &mut var2) {
        *v0 = lines
            .nth(4)
            .and_then(|x| x.split_ascii_whitespace().last())
            .value()?
            .parse()?;

        *v1 = lines
            .next()
            .and_then(|x| x.split_ascii_whitespace().last())
            .value()?
            .parse()?;

        *v2 = lines
            .nth(9)
            .and_then(|x| x.split_ascii_whitespace().last())
            .value()?
            .parse()?;

        lines.nth(1);
    }

    let mut valid_numbers = Vec::new();
    let mut current_states = vec![(0, 0, 0)];

    while let Some((number, index, z)) = current_states.pop() {
        if number >= 10_000_000_000_000 {
            if z == 0 {
                valid_numbers.push(number);
            }
            continue;
        }

        current_states.extend((1..=9).flat_map(|w| {
            let new_z = if z % 26 + var1[index] == w {
                Some(z / var0[index])
            } else if var0[index] == 26 {
                None
            } else {
                Some(z / var0[index] * 26 + w + var2[index])
            };

            new_z.map(|new_z| (number * 10 + w, index + 1, new_z))
        }));
    }

    valid_numbers.sort_unstable();

    let result1 = *valid_numbers.last().value()?;
    let result2 = *valid_numbers.first().value()?;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
