use aoc::*;

use eyre::ensure;
use itertools::{iproduct, Itertools};
use regex::Regex;

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);
    let input = input.trim();

    let re = Regex::new(r#"^target area: x=(.+?)\.\.(.+?), y=(.+?)\.\.(.+?)$"#)?;

    let cap = re.captures(input).value()?;
    let xmin: i64 = cap[1].parse()?;
    let xmax: i64 = cap[2].parse()?;
    let ymin: i64 = cap[3].parse()?;
    let ymax: i64 = cap[4].parse()?;

    ensure!(0 < xmin && xmin <= xmax, "invalid target x position");
    ensure!(ymin <= ymax && ymax < 0, "invalid target y position");

    let abs_max = xmax.max(-ymin);

    let partial_sums = (0..=abs_max)
        .scan(0, |state, x| {
            *state += x;
            Some(*state)
        })
        .collect_vec();

    let y_steps = (0..-ymin as usize)
        .filter_map(|v0| {
            let sums = &partial_sums[v0..];
            let offset = sums[0];

            sums.iter()
                .enumerate()
                .filter(|(_, &s)| (ymin..=ymax).contains(&(offset - s)))
                .map(|(index, _)| index)
                .minmax()
                .into_option()
                .map(|range| (v0, range))
        })
        .flat_map(|(v0, (min, max))| {
            let extra_steps = 2 * v0 + 1;

            let v0_neg = -(v0 as i64 + 1);
            let v0_pos = v0 as i64;

            let iter_neg = (min..=max).map(move |step| (step, v0_neg));
            let iter_pos = (extra_steps + min..=extra_steps + max).map(move |step| (step, v0_pos));

            iter_neg.chain(iter_pos)
        })
        .sorted_unstable()
        .collect_vec();

    let (max_y_steps, _) = *y_steps.last().value()?;

    let x_steps = (0..=xmax as usize)
        .flat_map(|v0| {
            let sums = &partial_sums[..=v0];
            let offset = sums[v0];

            let mut iter = sums.iter().rev().map(|&s| offset - s);
            let start = iter.position(|s| (xmin..=xmax).contains(&s));
            let count = iter.position(|s| !(xmin..=xmax).contains(&s));

            match (start, count) {
                (None, _) => None,
                (Some(start), None) => Some((start..=max_y_steps, v0)),
                (Some(start), Some(count)) => Some((start..=start + count, v0)),
            }
        })
        .flat_map(|(range, v0)| range.map(move |step| (step, v0 as i64)))
        .sorted_unstable()
        .collect_vec();

    let mut x_step_iter = x_steps.iter().enumerate();
    let mut y_step_iter = y_steps.iter().enumerate();

    let mut possible_v0s = y_steps
        .iter()
        .dedup_by(|(step_y0, _), (step_y1, _)| step_y0 == step_y1)
        .flat_map(|&(unique_y, _)| {
            [&mut x_step_iter, &mut y_step_iter]
                .into_iter()
                .flat_map(|iter| {
                    iter.take_while_ref(|(_, &(x, _))| x <= unique_y).filter(|(_, &(x, _))| x == unique_y).map(|(index, _)| index).minmax().into_option()
                })
                .next_tuple()
                .map(|(index_x, index_y)| (index_x.0..=index_x.1, index_y.0..=index_y.1))
        })
        .flat_map(|(range_x, range_y)| iproduct!(&x_steps[range_x], &y_steps[range_y]))
        .map(|(&(_, v0x), &(_, v0y))| (v0x, v0y))
        .collect_vec();

    possible_v0s.sort_unstable();
    possible_v0s.dedup();

    let result1 = partial_sums[*possible_v0s.iter().map(|(_, v0y)| v0y).max().value()? as usize];
    let result2 = possible_v0s.len();

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
