use aoc::*;

use itertools::Itertools;
use regex::Regex;
use smallvec::SmallVec;

use std::iter;

const WIDTH: usize = 50;
const HEIGHT: usize = 6;

fn turn_on_rect(pixels: &mut [u8], width: usize, height: usize) {
    for row in 0..height {
        pixels[row * WIDTH..row * WIDTH + width].fill(b'#');
    }
}

fn rotate_row(pixels: &mut [u8], row: usize, shift: usize) {
    pixels[row * WIDTH..(row + 1) * WIDTH].rotate_right(shift);
}

fn rotate_column(pixels: &mut [u8], column: usize, shift: usize) {
    let mut column_pixels: SmallVec<[u8; HEIGHT]> =
        pixels.iter().copied().skip(column).step_by(WIDTH).collect();

    column_pixels.rotate_right(shift);

    for (row, value) in column_pixels.into_iter().enumerate() {
        pixels[row * WIDTH + column] = value;
    }
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let re = Regex::new(r#"(?m)^(rect |rotate row y=|rotate column x=)(\d+)(?:x| by )(\d+)$"#)?;

    let mut pixels = vec![b' '; WIDTH * HEIGHT];

    for cap in re.captures_iter(&input) {
        let i: usize = cap[2].parse()?;
        let j: usize = cap[3].parse()?;

        match &cap[1] {
            "rect " => turn_on_rect(&mut pixels, i, j),
            "rotate row y=" => rotate_row(&mut pixels, i, j),
            "rotate column x=" => rotate_column(&mut pixels, i, j),
            _ => (),
        }
    }

    let count = pixels.iter().filter(|&&x| x == b'#').count();

    let message = pixels
        .chunks_exact(WIDTH)
        .flat_map(|row| iter::chain(row, b"\n").copied())
        .collect_vec();

    let result1 = count;
    let result2 = String::from_utf8_lossy(&message);

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
