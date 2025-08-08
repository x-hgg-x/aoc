use aoc::*;

use eyre::ensure;
use itertools::{Itertools, iproduct};

use std::ops::Range;

const MAX_TURNS: usize = 6;

trait ToRange {
    fn to_range(self) -> Range<usize>;
}

impl ToRange for (usize, usize) {
    fn to_range(self) -> Range<usize> {
        self.0..self.1
    }
}

struct Grid<const N: usize> {
    dims: [usize; N],
    tiles: Vec<bool>,
    turn: usize,
}

impl<const N: usize> Grid<N> {
    fn new(dims: [usize; N], tiles: Vec<bool>) -> Result<Self> {
        ensure!(
            dims.iter().product::<usize>() == tiles.len(),
            "unable to construct Grid: invalid length"
        );

        Ok(Self {
            dims,
            tiles,
            turn: 0,
        })
    }

    fn get_index(&self, coords: [usize; N]) -> usize {
        let mut cumul_dims = 1;
        let mut index = 0;
        for (dim, coord) in self.dims.into_iter().zip(coords) {
            index += coord * cumul_dims;
            cumul_dims *= dim;
        }
        index
    }

    fn get_ranges(&self) -> [(usize, usize); N] {
        let mut ranges = [(0, 0); N];
        ranges[0] = (MAX_TURNS - self.turn, self.dims[0] - MAX_TURNS + self.turn);
        ranges[1] = (MAX_TURNS - self.turn, self.dims[1] - MAX_TURNS + self.turn);
        (ranges[2..].iter_mut()).for_each(|range| *range = (0, self.turn + 1));
        ranges
    }

    fn init(mut self, initial_tiles: &[bool]) -> Self {
        let ranges = self.get_ranges();

        for ((x1, x0), &tile) in
            iproduct!(ranges[1].to_range(), ranges[0].to_range()).zip(initial_tiles)
        {
            let mut coords = [0; N];
            coords[0] = x0;
            coords[1] = x1;

            let index = self.get_index(coords);
            self.tiles[index] = tile;
        }

        self.turn += 1;
        self
    }
}

impl Grid<3> {
    fn step3(&mut self, buffer: &mut Vec<bool>) {
        let ranges = self.get_ranges();

        let [range0, range1, range2] = [
            ranges[0].to_range(),
            ranges[1].to_range(),
            ranges[2].to_range(),
        ];

        let ranges_iter = iproduct!(range2.clone(), range1.clone(), range0.clone());

        buffer.clear();
        buffer.extend(ranges_iter.clone().map(|(x2, x1, x0)| {
            let neighbors_iter = iproduct!(
                [
                    (x2 as isize - 1).unsigned_abs(),
                    x2,
                    (x2 + 1).min(range2.end - 1)
                ],
                x1.saturating_sub(1).max(range1.start)..(x1 + 2).min(range1.end),
                x0.saturating_sub(1).max(range0.start)..(x0 + 2).min(range0.end)
            );

            let center = self.tiles[self.get_index([x0, x1, x2])];

            let neighbors_count = neighbors_iter
                .filter(|&(y2, y1, y0)| self.tiles[self.get_index([y0, y1, y2])])
                .count()
                - center as usize;

            matches!((center, neighbors_count), (true, 2 | 3) | (false, 3))
        }));

        for ((x2, x1, x0), &value) in ranges_iter.zip(&*buffer) {
            let index = self.get_index([x0, x1, x2]);
            self.tiles[index] = value;
        }

        self.turn += 1;
    }
}

impl Grid<4> {
    fn step4(&mut self, buffer: &mut Vec<bool>) {
        let ranges = self.get_ranges();

        let [range0, range1, range2, range3] = [
            ranges[0].to_range(),
            ranges[1].to_range(),
            ranges[2].to_range(),
            ranges[3].to_range(),
        ];

        let ranges_iter = iproduct!(
            range3.clone(),
            range2.clone(),
            range1.clone(),
            range0.clone()
        );

        buffer.clear();

        buffer.extend(ranges_iter.clone().map(|(x3, x2, x1, x0)| {
            let neighbors_iter = iproduct!(
                [
                    (x3 as isize - 1).unsigned_abs(),
                    x3,
                    (x3 + 1).min(range3.end - 1)
                ],
                [
                    (x2 as isize - 1).unsigned_abs(),
                    x2,
                    (x2 + 1).min(range2.end - 1)
                ],
                x1.saturating_sub(1).max(range1.start)..(x1 + 2).min(range1.end),
                x0.saturating_sub(1).max(range0.start)..(x0 + 2).min(range0.end)
            );

            let center = self.tiles[self.get_index([x0, x1, x2, x3])];
            let neighbors_count = neighbors_iter
                .filter(|&(y3, y2, y1, y0)| self.tiles[self.get_index([y0, y1, y2, y3])])
                .count()
                - center as usize;

            matches!((center, neighbors_count), (true, 2 | 3) | (false, 3))
        }));

        for ((x3, x2, x1, x0), &value) in ranges_iter.zip(&*buffer) {
            let index = self.get_index([x0, x1, x2, x3]);
            self.tiles[index] = value;
        }

        self.turn += 1;
    }
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let width = input.lines().next().value()?.len();
    let height = input.lines().count();

    let initial_tiles = input
        .bytes()
        .filter_map(|x| match x {
            b'.' => Some(false),
            b'#' => Some(true),
            _ => None,
        })
        .collect_vec();

    let mut buffer = Vec::new();

    let dims3 = [2 * MAX_TURNS + width, 2 * MAX_TURNS + height, MAX_TURNS + 1];

    let dims4 = [
        2 * MAX_TURNS + width,
        2 * MAX_TURNS + height,
        MAX_TURNS + 1,
        MAX_TURNS + 1,
    ];

    let tiles3 = vec![false; dims3.iter().product()];
    let tiles4 = vec![false; dims4.iter().product()];

    let mut grid3 = Grid::new(dims3, tiles3)?.init(&initial_tiles);
    let mut grid4 = Grid::new(dims4, tiles4)?.init(&initial_tiles);

    for _ in 0..MAX_TURNS {
        grid3.step3(&mut buffer);
        grid4.step4(&mut buffer);
    }

    let dim_0 = dims3[0];
    let dim_01 = dim_0 * dims3[1];
    let dim_012 = dim_01 * dims3[2];

    let count_01 = |tiles: &[bool]| tiles.iter().filter(|&&x| x).count();

    let count_012 = |tiles: &[bool]| {
        let (first, others) = tiles.split_at(dim_01);
        count_01(first) + 2 * others.chunks_exact(dim_01).map(count_01).sum::<usize>()
    };

    let count_0123 = |tiles: &[bool]| {
        let (first, others) = tiles.split_at(dim_012);
        count_012(first) + 2 * others.chunks_exact(dim_012).map(count_012).sum::<usize>()
    };

    let result1 = count_012(&grid3.tiles);
    let result2 = count_0123(&grid4.tiles);

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
