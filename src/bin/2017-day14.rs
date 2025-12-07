use std::iter;

use aoc::*;

use eyre::ensure;
use itertools::Itertools;
use smallvec::SmallVec;

struct Grid {
    width: usize,
    height: usize,
    tiles: Vec<bool>,
}

impl Grid {
    fn new(width: usize, height: usize, tiles: Vec<bool>) -> Result<Self> {
        ensure!(
            width * height == tiles.len(),
            "unable to construct Grid: width * height != tiles.len()"
        );

        Ok(Self {
            width,
            height,
            tiles,
        })
    }

    fn get_index(&self, row: usize, column: usize) -> usize {
        row * self.width + column
    }

    fn get_position(&self, index: usize) -> (usize, usize) {
        let row = index / self.width;
        let column = index % self.width;
        (row, column)
    }
}

fn knot_hash_round(
    list: &mut [u8],
    lengths: &[usize],
    current_position: &mut usize,
    skip: &mut usize,
) {
    let size = list.len();

    for &len in lengths {
        if len >= 2 {
            let offset = *current_position % size;
            list.rotate_left(offset);
            list[..len].reverse();
            list.rotate_right(offset);
        }

        *current_position += len + *skip;
        *skip += 1;
    }
}

fn knot_hash(input: &[u8]) -> SmallVec<[bool; 128]> {
    let lengths = iter::chain(input, &[17, 31, 73, 47, 23])
        .map(|&x| x as usize)
        .collect_vec();

    let mut list = (0..=u8::MAX).collect_vec();
    let mut current_position = 0;
    let mut skip = 0;

    for _ in 0..64 {
        knot_hash_round(&mut list, &lengths, &mut current_position, &mut skip);
    }

    list.chunks_exact(16)
        .flat_map(|elem| {
            let out = elem.iter().fold(0, |acc, x| acc ^ x);
            (0..8).rev().map(move |bit| (out >> bit) & 1 != 0)
        })
        .collect()
}

fn next_hash_input(hash_input: &mut SmallVec<[u8; 12]>, len: usize) {
    for (pos, x) in hash_input[len..].iter_mut().enumerate().rev() {
        if *x < b'9' {
            *x += 1;
            break;
        } else if pos == 0 {
            hash_input[len..].fill(b'0');
            hash_input.push(b'0');
            hash_input[len] = b'1';
            break;
        } else {
            *x = b'0';
        }
    }
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);
    let input = input.trim().as_bytes();

    let mut tiles = Vec::with_capacity(128 * 128);

    let mut hash_input = SmallVec::<[u8; 12]>::from_slice(input);
    hash_input.extend_from_slice(b"-0");
    tiles.extend_from_slice(knot_hash(&hash_input).as_slice());

    for _ in 0..127 {
        next_hash_input(&mut hash_input, input.len() + 1);
        tiles.extend_from_slice(knot_hash(&hash_input).as_slice());
    }

    let mut grid = Grid::new(128, 128, tiles)?;

    let result1 = grid.tiles.iter().filter(|&&x| x).count();

    let mut regions_count = 0usize;
    let mut queue = Vec::new();

    for index in 0..grid.tiles.len() {
        if grid.tiles[index] {
            queue.clear();
            queue.push(index);

            while let Some(id) = queue.pop() {
                if grid.tiles[id] {
                    grid.tiles[id] = false;

                    let mut process_neighbors = |new_row, new_column| {
                        queue.push(grid.get_index(new_row, new_column));
                    };

                    let (row, column) = grid.get_position(id);

                    if row > 0 {
                        process_neighbors(row - 1, column);
                    }
                    if row < grid.height - 1 {
                        process_neighbors(row + 1, column);
                    }
                    if column > 0 {
                        process_neighbors(row, column - 1);
                    }
                    if column < grid.width - 1 {
                        process_neighbors(row, column + 1);
                    }
                }
            }
            regions_count += 1;
        }
    }

    let result2 = regions_count;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
