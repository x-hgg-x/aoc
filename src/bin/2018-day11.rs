use aoc::*;

use itertools::iproduct;

const SIZE: usize = 300;
const SIZE_P_1: usize = 1 + SIZE;

fn compute_power_partial_sums(serial_number: i64) -> Vec<i64> {
    let mut power_partial_sums = vec![0; SIZE_P_1 * SIZE_P_1];
    let mut remaining = power_partial_sums.as_mut_slice();

    for index_y in 0.. {
        let (prev_row, row) = remaining.split_at_mut(SIZE_P_1);
        if row.is_empty() {
            break;
        }

        let iter = prev_row.windows(2).enumerate().scan(0, |sum, (index_x, value_row)| {
            let x = index_x as i64;
            let y = index_y as i64;
            let rack_id = x + 10;
            let power = (((rack_id * (rack_id * y + serial_number)) / 100) % 10) - 5;

            *sum += value_row[1] - value_row[0] + power;
            Some(*sum)
        });

        for (partial_sum, value) in row[1..SIZE_P_1].iter_mut().zip(iter) {
            *partial_sum = value;
        }

        remaining = row;
    }

    power_partial_sums
}

fn compute_max_square_sum(power_partial_sums: &[i64], square_size: usize) -> Result<(i64, usize, usize)> {
    iproduct!(square_size..SIZE_P_1, square_size..SIZE_P_1)
        .map(|(row, column)| {
            let prev_row = row - square_size;
            let prev_column = column - square_size;

            let ul = power_partial_sums[prev_row * SIZE_P_1 + prev_column];
            let ur = power_partial_sums[prev_row * SIZE_P_1 + column];
            let dl = power_partial_sums[row * SIZE_P_1 + prev_column];
            let dr = power_partial_sums[row * SIZE_P_1 + column];

            (ul + dr - ur - dl, column - square_size, row - square_size)
        })
        .max_by_key(|&(sum, ..)| sum)
        .value()
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let serial_number = input.trim().parse::<i64>()?;

    let power_partial_sums = compute_power_partial_sums(serial_number);

    let (_, x1, y1) = compute_max_square_sum(&power_partial_sums, 3)?;
    let result1 = format!("{x1},{y1}");

    let (x2, y2, best_square_size) = (1..=300)
        .map(|square_size| {
            let max = compute_max_square_sum(&power_partial_sums, square_size)?;
            Ok((max, square_size))
        })
        .try_process(|iter| iter.max_by_key(|&((sum, ..), _)| sum))?
        .map(|((_, x, y), square_size)| (x, y, square_size))
        .value()?;

    let result2 = format!("{x2},{y2},{best_square_size}");

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
