use eyre::Result;
use regex::Regex;
use smallvec::SmallVec;

use std::fs;
use std::iter::once;

const WIDTH: usize = 50;
const HEIGHT: usize = 6;

fn turn_on_rect(pixels: &mut [char], width: usize, height: usize) {
    for row in 0..height {
        pixels[row * WIDTH..row * WIDTH + width].fill('#');
    }
}

fn rotate_row(pixels: &mut [char], row: usize, shift: usize) {
    pixels[row * WIDTH..(row + 1) * WIDTH].rotate_right(shift);
}

fn rotate_column(pixels: &mut [char], column: usize, shift: usize) {
    let mut column_pixels = <SmallVec<[char; HEIGHT]>>::from_iter(pixels.iter().copied().skip(column).step_by(WIDTH));

    column_pixels.rotate_right(shift);

    for (row, value) in column_pixels.into_iter().enumerate() {
        pixels[row * WIDTH + column] = value;
    }
}

fn main() -> Result<()> {
    let input = fs::read_to_string("inputs/2016-day08.txt")?;

    let re = Regex::new(r#"(?m)^(rect |rotate row y=|rotate column x=)(\d+)(?:x| by )(\d+)$"#)?;

    let mut pixels = vec![' '; WIDTH * HEIGHT];

    for cap in re.captures_iter(&input) {
        let i: usize = cap[2].parse().unwrap();
        let j: usize = cap[3].parse().unwrap();

        match &cap[1] {
            "rect " => turn_on_rect(&mut pixels, i, j),
            "rotate row y=" => rotate_row(&mut pixels, i, j),
            "rotate column x=" => rotate_column(&mut pixels, i, j),
            _ => {}
        }
    }

    let result1 = pixels.iter().filter(|&&x| x == '#').count();
    let result2 = String::from_iter((0..HEIGHT).flat_map(|row| pixels[row * WIDTH..(row + 1) * WIDTH].iter().chain(once(&'\n'))));

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
