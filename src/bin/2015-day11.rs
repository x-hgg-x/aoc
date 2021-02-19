use std::fs;

fn increment(input: &mut [u8]) {
    let mut carry = 1_u8;
    for x in input.iter_mut().rev() {
        if *x + carry <= b'z' {
            *x += carry;
            carry = 0;
        } else {
            *x = b'a';
            carry = 1;
        }
    }
}

fn check_password(input: &[u8]) -> bool {
    let mut iter = input.windows(2).enumerate().filter(|(_, x)| x[0] == x[1]);

    let check1 = iter
        .next()
        .and_then(|(pos1, _)| iter.last().map(|(pos2, _)| pos2 - pos1 > 1))
        .filter(|&x| x)
        .is_some();

    let check2 = input
        .windows(3)
        .any(|x| x[0] + 1 == x[1] && x[1] + 1 == x[2]);

    let check3 = !input.iter().any(|&x| x == b'i' || x == b'o' || x == b'l');

    check1 && check2 && check3
}

fn next_password(input: &mut [u8]) -> String {
    loop {
        increment(input);
        if check_password(input) {
            break String::from_utf8_lossy(input).into_owned();
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = fs::read_to_string("inputs/2015-day11.txt")?
        .trim()
        .as_bytes()
        .to_vec();

    let result1 = next_password(&mut input);
    let result2 = next_password(&mut input);

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
