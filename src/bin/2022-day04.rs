use aoc::*;

use eyre::ensure;
use itertools::Itertools;

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let pairs: Vec<_> = input
        .lines()
        .map(|line| {
            let (min1, max1, min2, max2) = line.split([',', '-']).map(|x| Ok(x.parse::<u64>()?)).try_process(|mut iter| iter.next_tuple())?.value()?;
            ensure!(min1 <= max1 && min2 <= max2, "invalid input");
            Ok(((min1, max1), (min2, max2)))
        })
        .try_collect()?;

    let result1 = pairs.iter().filter(|((min1, max1), (min2, max2))| min1 <= min2 && max2 <= max1 || min2 <= min1 && max1 <= max2).count();
    let result2 = pairs.iter().filter(|((min1, max1), (min2, max2))| min1.max(min2) <= max1.min(max2)).count();

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
