use itertools::Itertools;
use smallvec::SmallVec;

use std::collections::VecDeque;
use std::fs;

const INTERVAL_LENGTH: usize = 1000;

struct HashInfo {
    index: usize,
    triple: u8,
    quintuples: [bool; 16],
}

struct Queue {
    additional_hashs: usize,
    current_index: usize,
    hash_infos: VecDeque<HashInfo>,
    quintuples_count: [u16; 16],
}

impl Queue {
    fn new(additional_hashs: usize) -> Self {
        Self { additional_hashs, current_index: 0, hash_infos: VecDeque::new(), quintuples_count: [0; 16] }
    }

    fn is_empty(&self) -> bool {
        self.hash_infos.is_empty()
    }

    fn compute_next_hash(&mut self, input: &str) {
        let index = self.current_index;
        self.current_index += 1;

        let mut hex: SmallVec<[u8; 32]> = md5::compute(format!("{}{}", input, index)).iter().flat_map(|x| [(x & 0xF0) >> 4, x & 0x0F]).collect();

        for _ in 0..self.additional_hashs {
            for byte in &mut hex {
                *byte = char::from_digit(*byte as u32, 16).unwrap() as u8;
            }
            hex = md5::compute(hex).iter().flat_map(|x| [(x & 0xF0) >> 4, x & 0x0F]).collect();
        }

        if let Some(triple) = hex.windows(3).find(|x| x.iter().all_equal()).map(|x| x[0]) {
            let mut quintuples = [false; 16];

            hex.windows(5).filter(|x| x.iter().all_equal()).for_each(|x| {
                quintuples[x[0] as usize] = true;
            });

            for (count, flag) in self.quintuples_count.iter_mut().zip(quintuples) {
                *count += flag as u16;
            }

            self.hash_infos.push_back(HashInfo { index, triple, quintuples });
        }
    }

    fn pop_front(&mut self) -> HashInfo {
        let hash_info = self.hash_infos.pop_front().unwrap();
        for (count, flag) in self.quintuples_count.iter_mut().zip(hash_info.quintuples) {
            *count -= flag as u16;
        }
        hash_info
    }

    fn compute_64th_key_index(&mut self, input: &str) -> usize {
        let mut key_count = 0;

        while self.is_empty() {
            self.compute_next_hash(input);
        }

        loop {
            let hash_info = self.pop_front();

            while self.current_index < hash_info.index + INTERVAL_LENGTH + 1 {
                self.compute_next_hash(input);
            }

            if self.quintuples_count[hash_info.triple as usize] != 0 {
                key_count += 1;
            }

            if key_count == 64 {
                break hash_info.index;
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("inputs/2016-day14.txt")?;
    let input = input.trim();

    let result1 = Queue::new(0).compute_64th_key_index(input);
    let result2 = Queue::new(2016).compute_64th_key_index(input);

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
