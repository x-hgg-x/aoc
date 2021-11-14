use eyre::Result;
use itertools::{Either, Itertools};

use std::fs;

fn get_subset_sum_iter(set: &[u64], goal_sum: u64) -> impl Iterator<Item = u64> + '_ {
    (1..(1u64 << set.len()))
        .scan((0, 0), move |state, index| {
            let new_gray = index ^ (index >> 1);
            let bit_changed = state.1 ^ new_gray;
            let diff = set[bit_changed.trailing_zeros() as usize];
            if new_gray & bit_changed == 0 {
                state.0 -= diff;
            } else {
                state.0 += diff;
            }
            state.1 = new_gray;

            Some((state.0, new_gray))
        })
        .filter(move |&(sum, _)| sum == goal_sum)
        .map(|(_, bitset)| bitset)
}

fn get_partition<'a>(iter: impl Iterator<Item = u64> + 'a, set: &'a [u64]) -> impl Iterator<Item = (Vec<u64>, Vec<u64>)> + 'a {
    iter.map(move |bitset| -> (Vec<_>, Vec<_>) {
        (0..set.len()).partition_map(|n| if (bitset >> n) & 1 != 0 { Either::Left(set[n as usize]) } else { Either::Right(set[n as usize]) })
    })
}

fn get_optimal_qe(weights: &[u64], goal_weight: u64, func: impl Fn(&[u64]) -> bool) -> u64 {
    let valid_subsets = get_subset_sum_iter(weights, goal_weight).collect_vec();
    let min_length = valid_subsets.iter().map(|bitset| bitset.count_ones()).min().unwrap();
    let filtered_set_iter = valid_subsets.into_iter().filter(move |bitset| bitset.count_ones() == min_length);

    get_partition(filtered_set_iter, weights).filter(|(_, remaining)| func(remaining)).map(|(first_group, _)| first_group.iter().product()).min().unwrap()
}

fn main() -> Result<()> {
    let input = fs::read_to_string("inputs/2015-day24.txt")?;

    let weights = input.split_ascii_whitespace().map(|x| x.parse::<u64>().unwrap()).collect_vec();
    let total_weight: u64 = weights.iter().sum();

    let goal_weight1 = total_weight / 3;
    let result1 = get_optimal_qe(&weights, goal_weight1, |remaining| get_subset_sum_iter(remaining, goal_weight1).next().is_some());

    let goal_weight2 = total_weight / 4;
    let result2 = get_optimal_qe(&weights, goal_weight2, |second_group| {
        get_partition(get_subset_sum_iter(second_group, goal_weight2), second_group)
            .any(|(_, third_group)| get_subset_sum_iter(&third_group, goal_weight2).next().is_some())
    });

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
