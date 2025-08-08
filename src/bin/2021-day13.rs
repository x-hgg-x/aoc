use aoc::*;

use eyre::bail;
use itertools::Itertools;
use regex::Regex;

use std::iter::{once, repeat_n};

enum Fold {
    X(usize),
    Y(usize),
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let re = Regex::new(r#"^fold along (.+?)=(\d+)$"#)?;

    let mut lines = input.lines();

    let mut dots: Vec<(usize, usize)> = lines
        .by_ref()
        .take_while(|&line| !line.is_empty())
        .map(|line| line.split(',').next_tuple().map(|(x, y)| Result::Ok((x.parse()?, y.parse()?))).value())
        .try_process(|iter| iter.try_collect())??;

    let folds: Vec<_> = lines
        .map(|line| {
            let cap = re.captures(line).value()?;

            match &cap[1] {
                "x" => Ok(Fold::X(cap[2].parse()?)),
                "y" => Ok(Fold::Y(cap[2].parse()?)),
                _ => bail!("unknown fold instruction: {line}"),
            }
        })
        .try_collect()?;

    let mut buf = Vec::with_capacity(dots.len());

    let mut process_fold = |fold| {
        buf.clear();

        let (fx, fy) = match fold {
            Fold::X(x) => (x, usize::MAX),
            Fold::Y(y) => (usize::MAX, y),
        };

        for &(x, y) in &dots {
            let new_x = if x <= fx { x } else { 2 * fx - x };
            let new_y = if y <= fy { y } else { 2 * fy - y };
            buf.push((new_x, new_y));
        }

        buf.sort_unstable();
        buf.dedup();

        std::mem::swap(&mut dots, &mut buf);

        dots.len()
    };

    let mut fold_iter = folds.into_iter();
    let result1 = process_fold(fold_iter.next().value()?);

    for fold in fold_iter {
        process_fold(fold);
    }

    let (max_x, max_y) = dots.iter().fold((0, 0), |(max_x, max_y), &(x, y)| (x.max(max_x), y.max(max_y)));
    let width = max_x + 1;
    let height = max_y + 1;

    let mut image = repeat_n(repeat_n(b' ', width).chain(once(b'\n')), height).flatten().collect_vec();
    for (x, y) in dots {
        image[(width + 1) * y + x] = b'#';
    }

    let result2 = String::from_utf8_lossy(&image);

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
