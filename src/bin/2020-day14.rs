use aoc::*;

use eyre::bail;
use itertools::Itertools;
use regex::Regex;
use smallvec::SmallVec;

use std::collections::HashMap;
use std::iter::once;

#[derive(Clone)]
struct Mask {
    m0: u64,
    m1: u64,
    mx: u64,
    mx_indices: SmallVec<[u8; 36]>,
}

enum Instruction {
    SetMask(Mask),
    SetMemory { index: u64, value: u64 },
}

fn compute_floating_masks(mask: &Mask) -> impl Iterator<Item = u64> + '_ {
    once(0).chain((1u64..(1 << mask.mx.count_ones())).scan((0, 0), |(floating, gray), index| {
        let new_gray = index ^ (index >> 1);
        let bit_changed = *gray ^ new_gray;
        *gray = new_gray;

        let bit = 1 << mask.mx_indices[bit_changed.trailing_zeros() as usize];
        if new_gray & bit_changed == 0 {
            *floating &= !bit;
        } else {
            *floating |= bit;
        }

        Some(*floating)
    }))
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let regex_set_mask = Regex::new(r#"^mask = ([01X]{36})$"#)?;
    let regex_set_memory = Regex::new(r#"^mem\[(\d+)\] = (\d+)$"#)?;

    let instructions: Vec<_> = input
        .lines()
        .map(|line| {
            if let Some(cap) = regex_set_mask.captures(line) {
                let mut m0 = 0;
                let mut m1 = 0;
                let mut mx = 0;
                let mut mx_indices = SmallVec::new();

                for (index, &x) in cap[1].as_bytes().iter().rev().enumerate() {
                    match x {
                        b'1' => {
                            let bit = 1 << index;
                            m0 += bit;
                            m1 += bit;
                        }
                        b'X' => {
                            let bit = 1 << index;
                            m1 += bit;
                            mx += bit;
                            mx_indices.push(index as u8);
                        }
                        _ => (),
                    }
                }

                Ok(Instruction::SetMask(Mask { m0, m1, mx, mx_indices }))
            } else if let Some(cap) = regex_set_memory.captures(line) {
                Ok(Instruction::SetMemory { index: cap[1].parse()?, value: cap[2].parse()? })
            } else {
                bail!("unknown instruction: {line}");
            }
        })
        .try_collect()?;

    let mut memory1 = HashMap::new();
    let mut memory2 = HashMap::new();

    let mut mask = Mask { m0: 0, m1: !0, mx: 0, mx_indices: SmallVec::new() };

    for instruction in &instructions {
        match *instruction {
            Instruction::SetMask(ref new_mask) => mask = new_mask.clone(),
            Instruction::SetMemory { index, value } => {
                memory1.insert(index, value & mask.m1 | mask.m0);

                for floating in compute_floating_masks(&mask) {
                    memory2.insert(index & (!mask.mx) | mask.m0 | floating, value);
                }
            }
        }
    }

    let result1 = memory1.values().sum::<u64>();
    let result2 = memory2.values().sum::<u64>();

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
