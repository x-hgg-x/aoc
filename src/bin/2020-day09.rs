use aoc::*;

use eyre::bail;
use itertools::Itertools;

use std::cmp::Ordering;

const WINDOW_SIZE: usize = 25;

fn find_invalid_number(list: &[i64]) -> Option<i64> {
    let mut sums = list[..WINDOW_SIZE].iter().enumerate().flat_map(|(index_x, x)| list[index_x + 1..WINDOW_SIZE].iter().map(move |y| x + y)).collect_vec();
    let mut buffer = Vec::with_capacity(sums.len());

    let number = list[WINDOW_SIZE];
    if !sums.contains(&number) {
        return Some(number);
    }

    for slice in list.windows(WINDOW_SIZE + 1).skip(1) {
        buffer.clear();

        let mut iter = sums.iter().skip(WINDOW_SIZE - 1);
        for (n, elem) in (0..WINDOW_SIZE - 1).rev().zip(&slice[..WINDOW_SIZE - 1]) {
            buffer.extend(iter.by_ref().take(n));
            buffer.push(elem + slice[WINDOW_SIZE - 1]);
        }

        std::mem::swap(&mut sums, &mut buffer);

        let number = slice[WINDOW_SIZE];
        if !sums.contains(&number) {
            return Some(number);
        }
    }

    None
}

fn find_encryption_weakness(list: &[i64], invalid_number: i64) -> Result<i64> {
    let mut start_index = 0;
    let mut end_index = 0;
    let mut sum = list[start_index];

    loop {
        if !(start_index <= end_index && end_index < list.len()) {
            bail!("unable to find encryption weakness");
        }

        match sum.cmp(&invalid_number) {
            Ordering::Equal => {
                let (min, max) = list[start_index..=end_index].iter().minmax().into_option().value()?;
                return Ok(min + max);
            }
            Ordering::Less => {
                end_index += 1;
                sum += list[end_index];
            }
            Ordering::Greater => {
                sum -= list[start_index];
                start_index += 1;
            }
        }
    }
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let list: Vec<i64> = input.split_ascii_whitespace().map(|x| x.parse()).try_collect()?;

    let invalid_number = find_invalid_number(&list).value()?;
    let encryption_weakness = find_encryption_weakness(&list, invalid_number)?;

    let result1 = invalid_number;
    let result2 = encryption_weakness;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
