use aoc::*;

use itertools::Itertools;

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let list: Vec<i64> = input.split_ascii_whitespace().map(|x| x.parse()).try_collect()?;

    let result1 = list.windows(2).filter(|x| x[0] < x[1]).count();
    let result2 = list.windows(3).map(|x| x.iter().sum::<i64>()).tuple_windows().filter(|(x, y)| x < y).count();

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
