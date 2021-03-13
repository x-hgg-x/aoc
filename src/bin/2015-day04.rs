use md5::Digest;
use smallvec::SmallVec;

use std::fs;

struct HashGenerator {
    n: usize,
    data: SmallVec<[u8; 24]>,
    input_len: usize,
    start: bool,
}

impl HashGenerator {
    fn new(input: &str) -> Self {
        let mut data = SmallVec::<[u8; 24]>::from_slice(input.as_bytes());
        data.push(b'1');

        Self {
            n: 1,
            data,
            input_len: input.len(),
            start: true,
        }
    }
}

impl Iterator for HashGenerator {
    type Item = (usize, Digest);

    fn next(&mut self) -> Option<Self::Item> {
        if self.start {
            self.start = false;
            return Some((self.n, md5::compute(&self.data)));
        }

        self.n += 1;

        let mut carry = 1;
        for (pos, x) in self.data[self.input_len..].iter_mut().enumerate().rev() {
            if *x + carry <= b'9' {
                *x += carry;
                break;
            } else if pos == 0 {
                self.data[self.input_len..].fill(b'0');
                self.data.push(b'0');
                self.data[self.input_len] = b'1';
                break;
            } else {
                *x = b'0';
                carry = 1;
            }
        }

        Some((self.n, md5::compute(&self.data)))
    }
}

fn find_digest<F: Fn(&Digest) -> bool>(input: &str, f: F) -> usize {
    HashGenerator::new(input)
        .find(|(_, digest)| f(digest))
        .map(|(n, _)| n)
        .unwrap()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("inputs/2015-day04.txt")?;
    let input = input.trim();

    let result1 = find_digest(&input, |digest| digest[..2] == [0, 0] && digest[2] <= 0x0F);
    let result2 = find_digest(&input, |digest| digest[..3] == [0, 0, 0]);

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
