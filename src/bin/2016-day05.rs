use aoc::*;

use itertools::Itertools;
use md5::Digest;
use smallvec::SmallVec;

use std::iter::once;

fn hash_generator(input: &[u8]) -> impl Iterator<Item = Digest> {
    let input_len = input.len();

    let mut data = SmallVec::<[u8; 24]>::from_slice(input);
    data.push(b'0');

    once(md5::compute(&data)).chain(std::iter::from_fn(move || {
        for (pos, x) in data[input_len..].iter_mut().enumerate().rev() {
            if *x < b'9' {
                *x += 1;
                break;
            } else if pos == 0 {
                data[input_len..].fill(b'0');
                data.push(b'0');
                data[input_len] = b'1';
                break;
            } else {
                *x = b'0';
            }
        }

        Some(md5::compute(&data))
    }))
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);
    let input = input.trim().as_bytes();

    let sub_hashes = hash_generator(input)
        .filter(|digest| digest[..2] == [0, 0] && digest[2] <= 0x0F)
        .map(|digest| ((digest[2] & 0x0F) as usize, (digest[3] >> 4) as usize))
        .scan([false; 8], |state, (fifth, sixth)| {
            if !state.iter().all(|&x| x) {
                if fifth < 8 && !state[fifth] {
                    state[fifth] = true;
                }
                Some((fifth, sixth))
            } else {
                None
            }
        })
        .collect_vec();

    let result1: String = sub_hashes.iter().map(|&(fifth, _)| char::from_digit(fifth as u32, 16).value()).take(8).try_collect()?;

    let mut password = ['_'; 8];
    for &(fifth, sixth) in sub_hashes.iter().rev() {
        if fifth < 8 {
            password[fifth as usize] = char::from_digit(sixth as u32, 16).value()?;
        }
    }
    let result2 = String::from_iter(password);

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
