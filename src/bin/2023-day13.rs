use aoc::*;

use itertools::Itertools;

use std::iter;

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let patterns: Vec<_> = input
        .split("\n\n")
        .map(|group| {
            let mut columns = vec![0u64; group.lines().next().value()?.len()];

            let rows = group
                .lines()
                .enumerate()
                .map(|(row_idx, line)| {
                    line.bytes()
                        .enumerate()
                        .map(|(col_idx, x)| {
                            let value = (x == b'#') as u64;
                            columns[col_idx] += value * (1 << row_idx);
                            value * (1 << col_idx)
                        })
                        .sum::<u64>()
                })
                .collect_vec();

            Result::Ok((rows, columns))
        })
        .try_collect()?;

    let mut summaries = [0u64; 2];

    for (row_pattern, column_pattern) in &patterns {
        for (pattern, multiplier) in [(row_pattern, 100), (column_pattern, 1)] {
            for (idx, x) in iter::zip(1.., pattern.windows(2)) {
                if (x[0] ^ x[1]).count_ones() as usize > 1 {
                    continue;
                }

                let diff = iter::zip(pattern[..idx].iter().rev(), &pattern[idx..])
                    .map(|(x1, x2)| (x1 ^ x2).count_ones() as usize)
                    .sum::<usize>();

                if let Some(summary) = summaries.get_mut(diff) {
                    *summary += idx as u64 * multiplier;
                }
            }
        }
    }

    let [result1, result2] = summaries;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
