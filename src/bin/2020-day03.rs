use aoc::*;

use itertools::Itertools;

fn compute_tree_count(tiles: &[bool], width: usize, slope: (usize, usize)) -> i64 {
    let (right, down) = slope;

    let mut count = 0;
    let mut current_index = 0;

    for row in tiles.chunks_exact(width).step_by(down) {
        if row[current_index] {
            count += 1;
        }
        current_index = (current_index + right) % width;
    }

    count as i64
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let tiles = input
        .bytes()
        .filter_map(|x| match x {
            b'.' => Some(false),
            b'#' => Some(true),
            _ => None,
        })
        .collect_vec();

    let width = input.lines().next().value()?.len();

    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut tree_counts = [0; 5];

    for (tree_count, &slope) in tree_counts.iter_mut().zip(&slopes) {
        *tree_count = compute_tree_count(&tiles, width, slope);
    }

    let result1 = tree_counts[1];
    let result2 = tree_counts.iter().product::<i64>();

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
