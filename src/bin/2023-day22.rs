use aoc::*;

use itertools::{Itertools, iproduct};

use std::cmp::Ordering;
use std::collections::HashMap;
use std::ops::RangeInclusive;

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let mut bricks: Vec<[RangeInclusive<i64>; 3]> = input
        .lines()
        .map(|line| {
            let [[x1, y1, z1], [x2, y2, z2]] = line
                .split('~')
                .map(|group| {
                    group
                        .split(',')
                        .map(|x| Ok(x.parse::<i64>()?))
                        .try_process(|iter| iter.collect_array().value())?
                })
                .try_process(|iter| iter.collect_array().value())??;

            Result::Ok([x1..=x2, y1..=y2, z1..=z2])
        })
        .try_process(|iter| {
            iter.sorted_unstable_by_key(|brick| *brick[2].start())
                .collect()
        })?;

    let mut z_map = HashMap::<_, Vec<_>>::new();
    let mut z_map_indexes = vec![HashMap::new(); bricks.len()];

    for (idx, brick) in bricks.iter().enumerate() {
        for (x, y) in iproduct!(brick[0].clone(), brick[1].clone()) {
            (z_map.entry([x, y]).or_default()).push(idx);
        }
    }

    for (&xy, list) in &z_map {
        for (z_map_idx, &brick_idx) in list.iter().enumerate() {
            z_map_indexes[brick_idx].insert(xy, z_map_idx);
        }
    }

    let support_bricks = (0..bricks.len())
        .map(|brick_idx| {
            let brick = (
                bricks[brick_idx][0].clone(),
                bricks[brick_idx][1].clone(),
                bricks[brick_idx][2].clone(),
            );

            let mut below_bricks_max_z = 0;
            let mut support_bricks_buffer = Vec::new();

            for (x, y) in iproduct!(brick.0, brick.1) {
                if let Some(z_map_idx) = z_map_indexes[brick_idx][&[x, y]].checked_sub(1) {
                    let below_brick_idx = z_map[&[x, y]][z_map_idx];
                    let max_z = *bricks[below_brick_idx][2].end();

                    match max_z.cmp(&below_bricks_max_z) {
                        Ordering::Less => (),
                        Ordering::Equal => support_bricks_buffer.push(below_brick_idx),
                        Ordering::Greater => {
                            below_bricks_max_z = max_z;
                            support_bricks_buffer.clear();
                            support_bricks_buffer.push(below_brick_idx);
                        }
                    }
                }
            }

            bricks[brick_idx][2] =
                below_bricks_max_z + 1..=below_bricks_max_z + brick.2.count() as i64;

            support_bricks_buffer.sort_unstable();
            support_bricks_buffer.dedup();
            support_bricks_buffer
        })
        .collect_vec();

    let result1 = bricks.len()
        - support_bricks
            .iter()
            .filter_map(|v| v.iter().exactly_one().ok())
            .sorted_unstable()
            .dedup()
            .count();

    let support_bricks_ref = support_bricks.iter().map(|v| &v[..]).collect_vec();

    let mut removed = Vec::new();

    let result2 = (0..bricks.len())
        .map(|brick_idx| {
            let mut support_bricks_ref = support_bricks_ref.clone();

            removed.clear();
            removed.push(brick_idx);

            let mut current_removed_index = 0;
            let mut next_removed_index = removed.len();

            loop {
                for (idx, slice) in support_bricks_ref.iter_mut().enumerate() {
                    let mut removed_iter = removed[current_removed_index..].iter();

                    let count = slice
                        .iter()
                        .take_while(|&&x| {
                            removed_iter.take_while_ref(|&&item| item <= x).last() == Some(&x)
                        })
                        .count();

                    if count > 0 {
                        *slice = &slice[count..];
                        if slice.is_empty() {
                            removed.push(idx);
                        }
                    }
                }

                if next_removed_index == removed.len() {
                    break;
                }

                current_removed_index = next_removed_index;
                next_removed_index = removed.len();
            }

            removed.len() - 1
        })
        .sum::<usize>();

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
