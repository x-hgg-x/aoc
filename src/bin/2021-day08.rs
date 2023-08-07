use aoc::*;

use eyre::bail;
use itertools::Itertools;

const DIGITS: [u8; 10] = [0b1110111, 0b0100100, 0b1011101, 0b1101101, 0b0101110, 0b1101011, 0b1111011, 0b0100101, 0b1111111, 0b1101111];

fn compute_segments(uniques: &[(u8, u8); 10]) -> Result<[u8; 7]> {
    let [mut v1, mut v4, mut v7, mut v8] = [0; 4];
    let mut v235 = [0; 3];
    let mut v069 = [0; 3];

    let mut iter_v_235 = v235.iter_mut();
    let mut iter_v_069 = v069.iter_mut();

    for &(x, len) in uniques {
        match len {
            2 => v1 = x,
            3 => v7 = x,
            4 => v4 = x,
            7 => v8 = x,
            5 => *iter_v_235.next().value()? = x,
            6 => *iter_v_069.next().value()? = x,
            _ => bail!("invalid input"),
        }
    }

    let [a, b, c, d, e, f, g] = &mut [0; 7];

    let adg = v235.iter().fold(0x7F, |acc, v| acc & v);
    let abfg = v069.iter().fold(0x7F, |acc, v| acc & v);

    *d = adg & v4;
    *a = v7 & !v1 & 0x7F;
    *e = v8 & !(abfg | v4) & 0x7F;
    *b = v4 & !(v1 | *d) & 0x7F;
    *g = adg & !(*a | *d) & 0x7F;
    *f = abfg & !(adg | *b) & 0x7F;
    *c = v1 & !*f & 0x7F;

    Ok([*a, *b, *c, *d, *e, *f, *g])
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let signals: Vec<_> = input
        .lines()
        .map(|line| {
            let mut uniques = [(0u8, 0u8); 10];
            let mut outputs = [(0u8, 0u8); 4];

            let (unique_part, output_part) = line.split(" | ").next_tuple().value()?;

            let fill = |part: &str, array: &mut [_]| {
                let iter = part.split_ascii_whitespace().map(|s| {
                    let value = s.bytes().map(|x| 1 << (x - b'a')).sum::<u8>();
                    let len = s.len();
                    (value, len as u8)
                });

                for (elem, array_elem) in iter.zip(array) {
                    *array_elem = elem;
                }
            };

            fill(unique_part, &mut uniques);
            fill(output_part, &mut outputs);

            Result::Ok((uniques, outputs))
        })
        .try_collect()?;

    let result1 = signals.iter().flat_map(|(_, outputs)| outputs.iter().map(|(_, len)| len)).filter(|len| matches!(len, 2 | 3 | 4 | 7)).count();

    let result2 = signals
        .iter()
        .map(|(uniques, outputs)| {
            let segments = compute_segments(uniques)?;

            outputs
                .iter()
                .map(|&(v, _)| {
                    let sum = segments.iter().enumerate().map(|(index, segment)| ((v & segment != 0) as u8) << index).sum::<u8>();
                    DIGITS.iter().position(|&x| x == sum).value()
                })
                .try_process(|iter| {
                    let mut sum = 0u64;
                    for x in iter {
                        sum = sum * 10 + x as u64;
                    }
                    sum
                })
        })
        .try_sum::<u64>()?;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
