use aoc::*;

use itertools::Itertools;
use regex::Regex;

const SIZE: usize = 1000;

struct Area {
    id: usize,
    x_offset: usize,
    y_offset: usize,
    x_size: usize,
    y_size: usize,
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let re = Regex::new(r#"(?m)^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$"#)?;

    let areas: Vec<_> = re
        .captures_iter(&input)
        .map(|cap| {
            Result::Ok(Area { id: cap[1].parse()?, x_offset: cap[2].parse()?, y_offset: cap[3].parse()?, x_size: cap[4].parse()?, y_size: cap[5].parse()? })
        })
        .try_collect()?;

    let mut grid = vec![0usize; SIZE * SIZE];

    for area in &areas {
        grid.chunks_exact_mut(SIZE).skip(area.y_offset).take(area.y_size).for_each(|line| {
            line.iter_mut().skip(area.x_offset).take(area.x_size).for_each(|x| *x += 1);
        });
    }

    let result1 = grid.iter().filter(|&&x| x >= 2).count();

    let result2 = areas
        .iter()
        .find(|&area| {
            grid.chunks_exact(SIZE).skip(area.y_offset).take(area.y_size).all(|line| line.iter().skip(area.x_offset).take(area.x_size).all(|&x| x == 1))
        })
        .map(|x| x.id)
        .value()?;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
