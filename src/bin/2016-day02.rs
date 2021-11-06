use eyre::Result;
use itertools::Itertools;

use std::fs;
use std::iter::once;

fn get_code(input: &[u8], keypad: &[&[u32]], start_pos: (usize, usize)) -> String {
    let (height, width) = (keypad.len(), keypad[0].len());

    input
        .iter()
        .scan(start_pos, |state, &x| {
            if x == b'U' && state.0 != 0 && keypad[state.0 - 1][state.1] != 0 {
                state.0 -= 1;
            }
            if x == b'D' && state.0 != height - 1 && keypad[state.0 + 1][state.1] != 0 {
                state.0 += 1;
            }
            if x == b'L' && state.1 != 0 && keypad[state.0][state.1 - 1] != 0 {
                state.1 -= 1;
            }
            if x == b'R' && state.1 != width - 1 && keypad[state.0][state.1 + 1] != 0 {
                state.1 += 1;
            }
            Some((x == b'\n').then(|| char::from_digit(keypad[state.0][state.1], 16).unwrap()))
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
