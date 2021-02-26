use std::fs;

fn get_code(input: &[u8], keypad: &[&[u32]], start_pos: (usize, usize)) -> String {
    let (height, width) = (keypad.len(), keypad[0].len());

    input
        .iter()
        .scan(start_pos, |state, c| match c {
            b'U' => {
                if state.0 != 0 && keypad[state.0 - 1][state.1] != 0 {
                    state.0 -= 1;
                }
                Some(None)
            }
            b'D' => {
                if state.0 != height - 1 && keypad[state.0 + 1][state.1] != 0 {
                    state.0 += 1;
                }
                Some(None)
            }
            b'L' => {
                if state.1 != 0 && keypad[state.0][state.1 - 1] != 0 {
                    state.1 -= 1;
                }
                Some(None)
            }
            b'R' => {
                if state.1 != width - 1 && keypad[state.0][state.1 + 1] != 0 {
                    state.1 += 1;
                }
                Some(None)
            }
            _ => Some(std::char::from_digit(keypad[state.0][state.1], 16)),
        })
        .filter_map(|x| x.map(|c| c.to_ascii_uppercase()))
        .collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("inputs/2016-day02.txt")?;
    let mut input = input.trim().as_bytes().to_vec();
    input.push(b'\n');

    let keypad1 = [&[1, 2, 3][..], &[4, 5, 6][..], &[7, 8, 9][..]];

    let keypad2 = [
        &[0, 0, 1, 0, 0][..],
        &[0, 2, 3, 4, 0][..],
        &[5, 6, 7, 8, 9][..],
        &[0, 10, 11, 12, 0][..],
        &[0, 0, 13, 0, 0][..],
    ];

    let result1 = get_code(&input, &keypad1, (1, 1));
    let result2 = get_code(&input, &keypad2, (2, 0));

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
