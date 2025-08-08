use aoc::*;

use eyre::ensure;
use itertools::{Itertools, izip};

use std::iter::{self, repeat_n};

const STEPS: usize = 50;

struct Table {
    data: [u64; 8],
}

impl Table {
    fn parse(s: &[u8]) -> Self {
        let mut data = [0u64; 8];

        for (bits, elem) in iter::zip(s.chunks_exact(64), &mut data) {
            *elem = bits
                .iter()
                .enumerate()
                .map(|(index, &x)| ((x == b'#') as u64) << index)
                .sum()
        }

        Self { data }
    }

    fn get(&self, bit: usize) -> bool {
        self.data[bit / 64] & (1 << (bit % 64)) != 0
    }

    fn first(&self) -> bool {
        self.get(0)
    }

    fn last(&self) -> bool {
        self.get((1 << 9) - 1)
    }
}

struct Rect {
    row_offset: usize,
    col_offset: usize,
    row_size: usize,
    col_size: usize,
}

struct Image {
    width: usize,
    pixels: Vec<Option<bool>>,
    default: bool,
    inner_rect: Rect,
}

impl Image {
    fn new(
        width: usize,
        height: usize,
        pixels: Vec<Option<bool>>,
        inner_rect: Rect,
    ) -> Result<Self> {
        ensure!(
            width * height == pixels.len(),
            "unable to construct Image: width * height != pixels.len()"
        );

        Ok(Self {
            width,
            pixels,
            default: false,
            inner_rect,
        })
    }

    fn enhance(&mut self, buf: &mut Vec<Option<bool>>, table: &Table) {
        let Image {
            width,
            ref pixels,
            inner_rect:
                Rect {
                    row_offset,
                    col_offset,
                    row_size,
                    col_size,
                },
            ..
        } = *self;

        let buf_iter = buf
            .chunks_exact_mut(self.width)
            .skip(row_offset - 1)
            .take(row_size + 2)
            .flat_map(|row| &mut row[col_offset - 1..][..col_size + 2]);

        let pixels_iter = pixels
            .chunks_exact(width)
            .tuple_windows()
            .skip(row_offset - 2)
            .take(row_size + 2)
            .flat_map(|(row_0, row_1, row_2)| {
                let iter0 = row_0.windows(3).skip(col_offset - 2).take(col_size + 2);
                let iter1 = row_1.windows(3).skip(col_offset - 2).take(col_size + 2);
                let iter2 = row_2.windows(3).skip(col_offset - 2).take(col_size + 2);

                izip!(iter0, iter1, iter2).map(|(x0, x1, x2)| {
                    let inner_iter0 = x0.iter().rev();
                    let inner_iter1 = x1.iter().rev();
                    let inner_iter2 = x2.iter().rev();

                    let bit = inner_iter2
                        .chain(inner_iter1)
                        .chain(inner_iter0)
                        .enumerate()
                        .map(|(index, &x)| {
                            let value = x.unwrap_or(self.default);
                            (value as usize) << index
                        })
                        .sum();

                    Some(table.get(bit))
                })
            });

        for (buf_elem, new_pixel) in buf_iter.zip(pixels_iter) {
            *buf_elem = new_pixel;
        }

        self.inner_rect.row_offset -= 1;
        self.inner_rect.col_offset -= 1;
        self.inner_rect.row_size += 2;
        self.inner_rect.col_size += 2;

        self.default = if self.default {
            table.last()
        } else {
            table.first()
        };

        std::mem::swap(&mut self.pixels, buf);
    }
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let mut lines = input.lines();
    let first_line = lines.next().value()?.as_bytes();
    lines.next();

    let table = Table::parse(first_line);

    let base_width = lines.clone().next().value()?.len();
    let base_height = lines.clone().count();

    let width = 2 * (STEPS + 1) + base_width;
    let height = 2 * (STEPS + 1) + base_height;

    let pixels = repeat_n(None, width * (STEPS + 1))
        .chain(lines.flat_map(|line| {
            repeat_n(None, STEPS + 1)
                .chain(line.bytes().map(|x| Some(x == b'#')))
                .chain(repeat_n(None, STEPS + 1))
        }))
        .chain(repeat_n(None, width * (STEPS + 1)))
        .collect_vec();

    let mut image = Image::new(
        width,
        height,
        pixels,
        Rect {
            row_offset: STEPS + 1,
            col_offset: STEPS + 1,
            row_size: base_height,
            col_size: base_width,
        },
    )?;

    let mut buf = image.pixels.clone();

    for _ in 0..2 {
        image.enhance(&mut buf, &table);
    }

    let result1 = image
        .pixels
        .iter()
        .copied()
        .filter(|x| matches!(x, Some(true)))
        .count();

    for _ in 2..STEPS {
        image.enhance(&mut buf, &table);
    }

    let result2 = image
        .pixels
        .iter()
        .copied()
        .filter(|x| matches!(x, Some(true)))
        .count();

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
