use aoc::*;

use itertools::{Either, Itertools};

fn get_subset_sum_iter(set: &[u64], goal_sum: u64) -> impl Iterator<Item = u64> + '_ {
    (1u64..(1 << set.len()))
        .scan((0, 0), move |(sum, gray), index| {
            let new_gray = index ^ (index >> 1);
            let bit_changed = *gray ^ new_gray;
            *gray = new_gray;

            let diff = set[bit_changed.trailing_zeros() as usize];
            if new_gray & bit_changed == 0 {
                *sum -= diff;
            } else {
                *sum += diff;
            }

            Some((*sum, new_gray))
        })
        .filter(move |&(sum, _)| sum == goal_sum)
        .map(|(_, bitset)| bitset)
}

fn get_partition<'a>(iter: impl Iterator<Item = u64> + 'a, set: &'a [u64]) -> impl Iterator<Item = (Vec<u64>, Vec<u64>)> + 'a {
    iter.map(move |bitset| -> (Vec<_>, Vec<_>) {
        (0..set.len()).partition_map(|n| if (bitset >> n) & 1 != 0 { Either::Left(set[n as usize]) } else { Either::Right(set[n as usize]) })
    })
}

fn get_optimal_qe(weights: &[u64], goal_weight: u64, func: impl Fn(&[u64]) -> bool) -> Result<u64> {
    let valid_subsets = get_subset_sum_iter(weights, goal_weight).collect_vec();
    let min_length = valid_subsets.iter().map(|bitset| bitset.count_ones()).min().value()?;
    let filtered_set_iter = valid_subsets.into_iter().filter(move |bitset| bitset.count_ones() == min_length);

    get_partition(filtered_set_iter, weights).filter(|(_, remaining)| func(remaining)).map(|(first_group, _)| first_group.iter().product()).min().value()
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let weights: Vec<u64> = input.split_ascii_whitespace().map(|x| x.parse()).try_collect()?;
    let total_weight: u64 = weights.iter().sum();

    let goal_weight1 = total_weight / 3;
    let result1 = get_optimal_qe(&weights, goal_weight1, |remaining| get_subset_sum_iter(remaining, goal_weight1).next().is_some())?;

    let goal_weight2 = total_weight / 4;
    let result2 = get_optimal_qe(&weights, goal_weight2, |second_group| {
        get_partition(get_subset_sum_iter(second_group, goal_weight2), second_group)
            .any(|(_, third_group)| get_subset_sum_iter(&third_group, goal_weight2).next().is_some())
    })?;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
