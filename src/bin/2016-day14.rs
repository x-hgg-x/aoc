use aoc::*;

use itertools::Itertools;
use md5::Digest;
use smallvec::SmallVec;

use std::collections::VecDeque;

const INTERVAL_LENGTH: usize = 1000;

struct HashGenerator {
    data: SmallVec<[u8; 24]>,
    input_len: usize,
    index: usize,
}

impl HashGenerator {
    fn new(input: &[u8]) -> Self {
        let mut data = SmallVec::from_slice(input);
        data.push(b'0');

        Self { data, input_len: input.len(), index: 0 }
    }
}

impl Iterator for HashGenerator {
    type Item = Digest;

    fn next(&mut self) -> Option<Digest> {
        if self.index == 0 {
            self.index += 1;
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

        self.index += 1;
        Some(md5::compute(&self.data))
    }
}

struct HashInfo {
    index: usize,
    triple: u8,
    quintuples: [bool; 16],
}

struct Queue {
    additional_hashs: usize,
    hash_generator: HashGenerator,
    hash_infos: VecDeque<HashInfo>,
    quintuples_count: [u16; 16],
}

impl Queue {
    fn new(input: &[u8], additional_hashs: usize) -> Self {
        Self { additional_hashs, hash_generator: HashGenerator::new(input), hash_infos: VecDeque::new(), quintuples_count: [0; 16] }
    }

    fn is_empty(&self) -> bool {
        self.hash_infos.is_empty()
    }

    fn compute_next_hash(&mut self) -> Result<()> {
        let mut hex: SmallVec<[u8; 32]> = self.hash_generator.next().as_deref().into_iter().flatten().flat_map(|x| [x >> 4, x & 0x0F]).collect();

        for _ in 0..self.additional_hashs {
            for byte in &mut hex {
                *byte = char::from_digit(*byte as u32, 16).value()? as u8;
            }
            hex = md5::compute(hex).iter().flat_map(|x| [x >> 4, x & 0x0F]).collect();
        }

        if let Some(triple) = hex.windows(3).find(|x| x.iter().all_equal()).map(|x| x[0]) {
            let mut quintuples = [false; 16];

            hex.windows(5).filter(|x| x.iter().all_equal()).for_each(|x| {
                quintuples[x[0] as usize] = true;
            });

            for (count, flag) in self.quintuples_count.iter_mut().zip(quintuples) {
                *count += flag as u16;
            }

            self.hash_infos.push_back(HashInfo { index: self.hash_generator.index - 1, triple, quintuples });
        }

        Ok(())
    }

    fn pop_front(&mut self) -> Result<HashInfo> {
        let hash_info = self.hash_infos.pop_front().value()?;
        for (count, flag) in self.quintuples_count.iter_mut().zip(hash_info.quintuples) {
            *count -= flag as u16;
        }
        Ok(hash_info)
    }

    fn compute_64th_key_index(&mut self) -> Result<usize> {
        let mut key_count = 0;

        while self.is_empty() {
            self.compute_next_hash()?;
        }

        loop {
            let hash_info = self.pop_front()?;

            while self.hash_generator.index < hash_info.index + INTERVAL_LENGTH + 1 {
                self.compute_next_hash()?;
            }

            if self.quintuples_count[hash_info.triple as usize] != 0 {
                key_count += 1;
            }

            if key_count == 64 {
                break Ok(hash_info.index);
            }
        }
    }
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);
    let input = input.trim().as_bytes();

    let result1 = Queue::new(input, 0).compute_64th_key_index()?;
    let result2 = Queue::new(input, 2016).compute_64th_key_index()?;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
