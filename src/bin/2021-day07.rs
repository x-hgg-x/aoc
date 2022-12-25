use aoc::*;

use itertools::Itertools;

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);
    let input = input.trim();

    let mut list: Vec<i64> = input.split(',').map(|x| Result::Ok(x.parse()?)).try_collect()?;
    list.sort_unstable();

    let min = *list.first().value()?;
    let max = *list.last().value()?;

    let list_with_count = list.into_iter().dedup_with_count().map(|(count, elem)| (count as i64, elem)).collect_vec();

    let (min_cost_1, min_cost_2) = (min..=max)
        .map(|position| {
            let mut sum1 = 0;
            let mut sum2 = 0;

            for &(count, elem) in list_with_count.iter() {
                let diff = (elem - position).abs();
                let p = count * diff;
                sum1 += p;
                sum2 += p * (diff + 1);
            }

            (sum1, sum2 / 2)
        })
        .fold((i64::MAX, i64::MAX), |(min_cost_1, min_cost_2), (cost1, cost2)| (min_cost_1.min(cost1), min_cost_2.min(cost2)));

    let result1 = min_cost_1;
    let result2 = min_cost_2;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
