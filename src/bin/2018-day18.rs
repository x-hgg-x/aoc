use aoc::*;

use eyre::{bail, ensure};
use itertools::{izip, Itertools};

use std::collections::hash_map::{DefaultHasher, Entry, HashMap};
use std::hash::*;
use std::iter::once;

const SIZE: usize = 50;
const WIDTH: usize = SIZE + 2;
const HEIGHT: usize = SIZE + 2;

#[derive(Copy, Clone, Hash)]
enum Tile {
    Empty,
    OpenGround,
    Trees,
    Lumberyard,
}

fn step(tiles: &mut Vec<Tile>, buf: &mut Vec<Tile>) -> Result<()> {
    buf.clear();

    tiles
        .chunks_exact(WIDTH)
        .tuple_windows()
        .flat_map(|(row_0, row_1, row_2)| {
            let iter = izip!(row_0.windows(3), row_1.windows(3), row_2.windows(3)).map(|(x0, x1, x2)| {
                let center = x1[1];

                let (trees, lumberyards) = x0.iter().chain([&x1[0], &x1[2]]).chain(x2).fold((0usize, 0usize), |(trees, lumberyards), x| match x {
                    Tile::Trees => (trees + 1, lumberyards),
                    Tile::Lumberyard => (trees, lumberyards + 1),
                    _ => (trees, lumberyards),
                });

                Ok(match (center, trees, lumberyards) {
                    (Tile::OpenGround, 3.., _) => Tile::Trees,
                    (Tile::Trees, _, 3..) => Tile::Lumberyard,
                    (Tile::Lumberyard, 0, _) | (Tile::Lumberyard, _, 0) => Tile::OpenGround,
                    (Tile::Empty, ..) => bail!("unable to step simulation"),
                    (tile, ..) => tile,
                })
            });

            once(Ok(Tile::Empty)).chain(iter).chain(once(Ok(Tile::Empty)))
        })
        .try_process(|iter| buf.extend([Tile::Empty; WIDTH].into_iter().chain(iter).chain([Tile::Empty; WIDTH])))?;

    std::mem::swap(buf, tiles);

    Ok(())
}

fn resource_value(tiles: &[Tile]) -> usize {
    let (trees, lumberyards) = tiles.iter().fold((0, 0), |(trees, lumberyards), tile| match tile {
        Tile::Trees => (trees + 1, lumberyards),
        Tile::Lumberyard => (trees, lumberyards + 1),
        _ => (trees, lumberyards),
    });

    trees * lumberyards
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let mut tiles = input
        .lines()
        .flat_map(|line| {
            let iter = line.chars().map(|c| match c {
                '.' => Ok(Tile::OpenGround),
                '|' => Ok(Tile::Trees),
                '#' => Ok(Tile::Lumberyard),
                _ => bail!("unknown tile"),
            });
            once(Ok(Tile::Empty)).chain(iter).chain(once(Ok(Tile::Empty)))
        })
        .try_process(|iter| [Tile::Empty; WIDTH].into_iter().chain(iter).chain([Tile::Empty; WIDTH]).collect_vec())?;

    ensure!(WIDTH * HEIGHT == tiles.len(), "incorrect grid dimensions");

    let mut buf = Vec::with_capacity(tiles.len());

    for _ in 0..10 {
        step(&mut tiles, &mut buf)?;
    }

    let result1 = resource_value(&tiles);

    let mut previous_states = HashMap::new();
    let mut count = 10usize;

    let old_count = loop {
        let mut hasher = DefaultHasher::new();
        tiles.hash(&mut hasher);

        match previous_states.entry(hasher.finish()) {
            Entry::Occupied(entry) => break *entry.get(),
            Entry::Vacant(entry) => entry.insert(count),
        };

        step(&mut tiles, &mut buf)?;
        count += 1;
    };

    let start = old_count;
    let cycle_size = count - old_count;

    let mut uniques_states = Vec::with_capacity(cycle_size);
    uniques_states.push(resource_value(&tiles));
    for _ in 1..cycle_size {
        step(&mut tiles, &mut buf)?;
        uniques_states.push(resource_value(&tiles));
    }

    let result2 = uniques_states[(1_000_000_000 - start) % cycle_size];

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
