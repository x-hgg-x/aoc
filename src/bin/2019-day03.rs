use aoc::*;

use eyre::bail;
use itertools::Itertools;
use num_complex::Complex;

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let mut wire_positions = input
        .lines()
        .enumerate()
        .flat_map(|(i_line, line)| {
            line.split(',').scan(
                (Complex::new(0, 0), 0),
                move |(position, step_count), path| {
                    (|| {
                        let mut chars = path.chars();

                        let direction = match chars.next() {
                            Some('L') => Complex::new(-1, 0),
                            Some('R') => Complex::new(1, 0),
                            Some('D') => Complex::new(0, -1),
                            Some('U') => Complex::new(0, 1),
                            other => bail!("unknown direction: {other:?}"),
                        };

                        let length = chars.as_str().parse::<i64>()?;

                        let current_position = *position;
                        let current_step_count = *step_count;

                        *position += length * direction;
                        *step_count += length;

                        Ok(Some((
                            i_line,
                            (1..=length).map(move |i| {
                                (current_position + i * direction, current_step_count + i)
                            }),
                        )))
                    })()
                    .transpose()
                },
            )
        })
        .try_process(|iter| {
            iter.flat_map(|(i_line, step_iter)| {
                step_iter.map(move |(position, step_count)| (position, i_line, step_count))
            })
            .collect_vec()
        })?;

    wire_positions.sort_unstable_by_key(|&(position, i_line, step_count)| {
        (position.re, position.im, i_line, step_count)
    });

    wire_positions.dedup_by_key(|(position, i_line, _)| (*position, *i_line));

    let intersections = wire_positions
        .windows(2)
        .filter(|&x| x[0].0 == x[1].0)
        .map(|x| (x[0].0.l1_norm(), x[0].2 + x[1].2))
        .collect_vec();

    let result1 = intersections.iter().map(|&(norm, _)| norm).min().value()?;

    let result2 = intersections
        .iter()
        .map(|&(_, steps)| steps)
        .min()
        .value()?;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
