use eyre::Result;
use itertools::Itertools;

use std::fs;
use std::iter::once;

fn get_code(input: &[u8], keypad: &[&[u32]], start_pos: (usize, usize)) -> String {
    let (height, width) = (keypad.len(), keypad[0].len());

    input
        .iter()
        .scan(start_pos, |(x, y), &direction| {
            if direction == b'U' && *x != 0 && keypad[*x - 1][*y] != 0 {
                *x -= 1;
            }
            if direction == b'D' && *x != height - 1 && keypad[*x + 1][*y] != 0 {
                *x += 1;
            }
            if direction == b'L' && *y != 0 && keypad[*x][*y - 1] != 0 {
                *y -= 1;
            }
            if direction == b'R' && *y != width - 1 && keypad[*x][*y + 1] != 0 {
                *y += 1;
            }
            Some((direction == b'\n').then(|| char::from_digit(keypad[*x][*y], 16).unwrap()))
        })
        .filter_map(|x| x.map(|c| c.to_ascii_uppercase()))
        .collect()
}

fn main() -> Result<()> {
    let input = fs::read_to_string("inputs/2016-day02.txt")?.lines().chain(once("")).join("\n");
    let input = input.as_bytes();

    let keypad1 = [&[1, 2, 3][..], &[4, 5, 6][..], &[7, 8, 9][..]];
    let keypad2 = [&[0, 0, 1, 0, 0][..], &[0, 2, 3, 4, 0][..], &[5, 6, 7, 8, 9][..], &[0, 10, 11, 12, 0][..], &[0, 0, 13, 0, 0][..]];

    let result1 = get_code(input, &keypad1, (1, 1));
    let result2 = get_code(input, &keypad2, (2, 0));

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
