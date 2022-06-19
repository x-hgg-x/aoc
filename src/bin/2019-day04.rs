use aoc::*;

use itertools::Itertools;

fn check_password_1(data: &[u8; 6]) -> bool {
    let increasing = data.windows(2).all(|x| x[0] <= x[1]);
    let adjacent_same = data.iter().dedup_with_count().any(|(count, _)| count >= 2);
    increasing && adjacent_same
}

fn check_password_2(data: &[u8; 6]) -> bool {
    let increasing = data.windows(2).all(|x| x[0] <= x[1]);
    let adjacent_same = data.iter().dedup_with_count().any(|(count, _)| count == 2);
    increasing && adjacent_same
}

fn next_password(mut data: [u8; 6]) -> [u8; 6] {
    for x in data.iter_mut().rev() {
        if *x < b'9' {
            *x += 1;
            break;
        } else {
            *x = b'0';
        }
    }
    data
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let (start_password, end_password) = input.split('-').map(|x| x.trim()).next_tuple().value()?;

    let start = start_password.as_bytes().try_into()?;
    let range_len = (1 + end_password.parse::<i64>()? - start_password.parse::<i64>()?).try_into()?;

    let result1 = std::iter::successors(Some(start), |&data| Some(next_password(data))).take(range_len).filter(check_password_1).count();
    let result2 = std::iter::successors(Some(start), |&data| Some(next_password(data))).take(range_len).filter(check_password_2).count();

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
