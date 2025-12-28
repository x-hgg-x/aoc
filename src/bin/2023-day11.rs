use aoc::*;

use itertools::Itertools;

fn compute_missing_indexes(
    galaxies: &mut [(i64, i64)],
    elem_fn: impl Fn(&(i64, i64)) -> i64,
) -> Vec<i64> {
    galaxies.sort_unstable_by_key(&elem_fn);

    galaxies
        .iter()
        .map(elem_fn)
        .map(|v| Some((v, v)))
        .dedup()
        .chain([None])
        .coalesce(|v1, v2| {
            let (Some((v1, _)), Some((v2, _))) = (v1, v2) else {
                return Ok(None);
            };
            if v1 + 1 == v2 {
                Ok(Some((v2, v2)))
            } else {
                Err((Some((v1 + 1, v2 - 1)), Some((v2, v2))))
            }
        })
        .flatten()
        .flat_map(|(start, end)| start..=end)
        .collect()
}

fn compute_shortest_paths_sum(
    galaxies: &[(i64, i64)],
    missing_x: &[i64],
    missing_y: &[i64],
    age: i64,
) -> i64 {
    galaxies
        .iter()
        .copied()
        .tuple_combinations()
        .map(|((x1, y1), (x2, y2))| {
            let (x_min, y_min) = (x1.min(x2), y1.min(y2));
            let (x_diff, y_diff) = ((x1 - x2).abs(), (y1 - y2).abs());

            let missing_x_count = missing_x
                .iter()
                .filter(|x| (x_min + 1..x_min + x_diff).contains(x))
                .count();

            let missing_y_count = missing_y
                .iter()
                .filter(|y| (y_min + 1..y_min + y_diff).contains(y))
                .count();

            x_diff + y_diff + age * (missing_x_count as i64 + missing_y_count as i64)
        })
        .sum()
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let mut galaxies = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.bytes()
                .enumerate()
                .filter(|&(_, v)| v == b'#')
                .map(move |(x, _)| (x as i64, y as i64))
        })
        .collect_vec();

    let missing_x = compute_missing_indexes(&mut galaxies, |&(x, _)| x);
    let missing_y = compute_missing_indexes(&mut galaxies, |&(_, y)| y);

    let result1 = compute_shortest_paths_sum(&galaxies, &missing_x, &missing_y, 1);
    let result2 = compute_shortest_paths_sum(&galaxies, &missing_x, &missing_y, 999999);

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
