use md5::Digest;
use smallvec::SmallVec;

use std::fs;

struct HashGenerator {
    data: SmallVec<[u8; 24]>,
    input_len: usize,
    start: bool,
}

impl HashGenerator {
    fn new(input: &str) -> Self {
        let mut data = SmallVec::<[u8; 24]>::from_slice(input.as_bytes());
        data.push(b'0');

        Self {
            data,
            input_len: input.len(),
            start: true,
        }
    }
}

impl Iterator for HashGenerator {
    type Item = Digest;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start {
            self.start = false;
            return Some(md5::compute(&self.data));
        }

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

        Some(md5::compute(&self.data))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("inputs/2016-day05.txt")?;
    let input = input.trim();

    let sub_hashes: Vec<_> = HashGenerator::new(input)
        .filter(|digest| digest[..2] == [0, 0] && digest[2] <= 0x0F)
        .map(|digest| (digest[2] as u32 % 16, digest[3] as u32 >> 4))
        .scan(
            (true, SmallVec::from_buf([false; 8])),
            |state, (fifth, sixth)| {
                if state.1.iter().all(|&x| x) {
                    state.0 = false;
                }

                if fifth < 8 && !state.1[fifth as usize] {
                    state.1[fifth as usize] = true;
                }

                Some((state.0, fifth, sixth))
            },
        )
        .take_while(|&(flag, _, _)| flag)
        .map(|(_, fifth, sixth)| (fifth, sixth))
        .collect();

    let result1: String = sub_hashes
        .iter()
        .map(|&(fifth, _)| std::char::from_digit(fifth, 16).unwrap())
        .take(8)
        .collect();

    let mut password = ['_'; 8];
    for &(fifth, sixth) in sub_hashes.iter().rev() {
        if fifth < 8 {
            password[fifth as usize] = std::char::from_digit(sixth, 16).unwrap();
        }
    }
    let result2: String = password.iter().collect();

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
