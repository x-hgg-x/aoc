use eyre::Result;
use md5::Digest;
use smallvec::SmallVec;

use std::fs;
use std::iter::once;

fn hash_generator(input: &str) -> impl Iterator<Item = (usize, Digest)> {
    let input_len = input.len();
    let mut n: usize = 1;

    let mut data = SmallVec::<[u8; 24]>::from_slice(input.as_bytes());
    data.push(b'1');

    once((n, md5::compute(&data))).chain(std::iter::from_fn(move || {
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

        n += 1;
        Some((n, md5::compute(&data)))
    }))
}

fn find_digest(input: &str, f: impl Fn(&Digest) -> bool) -> usize {
    hash_generator(input).find(|(_, digest)| f(digest)).map(|(n, _)| n).unwrap()
}

fn main() -> Result<()> {
    let input = fs::read_to_string("inputs/2015-day04.txt")?;
    let input = input.trim();

    let result1 = find_digest(input, |digest| digest[..2] == [0, 0] && digest[2] <= 0x0F);
    let result2 = find_digest(input, |digest| digest[..3] == [0, 0, 0]);

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
