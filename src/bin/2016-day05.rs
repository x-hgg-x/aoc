use eyre::Result;
use itertools::Itertools;
use md5::Digest;
use smallvec::SmallVec;

use std::fs;
use std::iter::once;

fn hash_generator(input: &[u8]) -> impl Iterator<Item = Digest> {
    let input_len = input.len();

    let mut data = SmallVec::<[u8; 24]>::from_slice(input);
    data.push(b'0');

    once(md5::compute(&data)).chain(std::iter::from_fn(move || {
        let mut carry = 1;
        for (pos, x) in data[input_len..].iter_mut().enumerate().rev() {
            if *x + carry <= b'9' {
                *x += carry;
                break;
            } else if pos == 0 {
                data[input_len..].fill(b'0');
                data.push(b'0');
                data[input_len] = b'1';
                break;
            } else {
                *x = b'0';
                carry = 1;
            }
        }

        Some(md5::compute(&data))
    }))
}

fn main() -> Result<()> {
    let input = fs::read_to_string("inputs/2016-day05.txt")?;
    let input = input.trim().as_bytes();

    let sub_hashes = hash_generator(input)
        .filter(|digest| digest[..2] == [0, 0] && digest[2] <= 0x0F)
        .map(|digest| (digest[2] as u32 % 16, digest[3] as u32 >> 4))
        .scan((true, [false; 8]), |state, (fifth, sixth)| {
            if state.1.iter().all(|&x| x) {
                state.0 = false;
            }

            if fifth < 8 && !state.1[fifth as usize] {
                state.1[fifth as usize] = true;
            }

            Some((state.0, fifth, sixth))
        })
        .take_while(|&(flag, _, _)| flag)
        .map(|(_, fifth, sixth)| (fifth, sixth))
        .collect_vec();

    let result1 = String::from_iter(sub_hashes.iter().map(|&(fifth, _)| char::from_digit(fifth, 16).unwrap()).take(8));

    let mut password = ['_'; 8];
    for &(fifth, sixth) in sub_hashes.iter().rev() {
        if fifth < 8 {
            password[fifth as usize] = char::from_digit(sixth, 16).unwrap();
        }
    }
    let result2 = String::from_iter(password);

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
