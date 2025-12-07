use aoc::*;

use eyre::{bail, ensure};
use itertools::izip;
use num_complex::Complex;
use smallvec::SmallVec;

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::iter;

const DIRECTIONS: [Complex<i64>; 4] = [
    Complex::new(0, 1),
    Complex::new(0, -1),
    Complex::new(-1, 0),
    Complex::new(1, 0),
];

const POSITION_OFFSETS: [Complex<i64>; 4] = [
    Complex::new(1, 1),
    Complex::new(1, -1),
    Complex::new(-1, 1),
    Complex::new(-1, -1),
];

struct Map {
    width: usize,
    tiles: Vec<u8>,
    initial_position: Complex<i64>,
    keys: SmallVec<[(Complex<i64>, u32); 26]>,
    doors: HashMap<Complex<i64>, u32>,
}

impl Map {
    fn new(
        width: usize,
        height: usize,
        tiles: Vec<u8>,
        initial_position: Complex<i64>,
        keys: SmallVec<[(Complex<i64>, u32); 26]>,
        doors: HashMap<Complex<i64>, u32>,
    ) -> Result<Self> {
        ensure!(
            width * height == tiles.len(),
            "unable to construct Map: width * height != tiles.len()"
        );

        Ok(Self {
            width,
            tiles,
            initial_position,
            keys,
            doors,
        })
    }

    fn get_index(&self, position: Complex<i64>) -> usize {
        position.im as usize * self.width + position.re as usize
    }
}

fn parse_grid(input: &str) -> Result<Map> {
    let mut initial_position = None;
    let mut tiles = Vec::new();
    let mut keys = SmallVec::new();
    let mut doors = HashMap::new();

    for (i_row, line) in input.lines().enumerate() {
        let i_row = i_row as i64;

        for (i_col, x) in line.bytes().enumerate() {
            let i_col = i_col as i64;

            match x {
                b'@' => initial_position = Some(Complex::new(i_col, i_row)),
                b'a'..=b'z' => keys.push((Complex::new(i_col, i_row), 1u32 << (x - b'a'))),
                b'A'..=b'Z' => {
                    doors.insert(Complex::new(i_col, i_row), 1u32 << (x - b'A'));
                }
                _ => (),
            }

            tiles.push(x);
        }
    }

    let width = input.lines().next().value()?.len();
    let height = input.lines().count();

    Map::new(width, height, tiles, initial_position.value()?, keys, doors)
}

type ReachableKeys = Vec<((i64, i64), u32, u32, usize)>;

fn compute_reachable_keys_cache(map: &Map) -> HashMap<(i64, i64), ReachableKeys> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    map.keys
        .iter()
        .map(|&(position, _)| position)
        .chain([map.initial_position])
        .map(|start_position| {
            let mut reachable_keys = Vec::new();

            visited.clear();
            queue.clear();
            queue.push_back((start_position, 0u32, 0usize));

            while let Some((position, doors, distance)) = queue.pop_front() {
                if !visited.insert(position) {
                    continue;
                }

                if let key @ b'a'..=b'z' = map.tiles[map.get_index(position)] {
                    reachable_keys.push((
                        (position.re, position.im),
                        1u32 << (key - b'a'),
                        doors,
                        distance,
                    ));
                }

                let iter = DIRECTIONS.into_iter().filter_map(|direction| {
                    let new_position = position + direction;
                    let tile = map.tiles[map.get_index(new_position)];

                    (tile != b'#').then(|| {
                        (
                            new_position,
                            doors | *map.doors.get(&new_position).unwrap_or(&0),
                            distance + 1,
                        )
                    })
                });

                queue.extend(iter);
            }

            ((start_position.re, start_position.im), reachable_keys)
        })
        .collect()
}

fn solve(map: &Map) -> Result<usize> {
    let reachable_keys_cache = compute_reachable_keys_cache(map);

    let all_keys = map.keys.iter().fold(0, |acc, (_, key)| acc | key);

    let mut visited = HashSet::new();

    let mut queue = BinaryHeap::from([Reverse((
        0usize,
        (map.initial_position.re, map.initial_position.im),
        0u32,
    ))]);

    while let Some(Reverse((distance, position, collected_keys))) = queue.pop() {
        if collected_keys == all_keys {
            return Ok(distance);
        }

        if !visited.insert((position, collected_keys)) {
            continue;
        }

        reachable_keys_cache[&position]
            .iter()
            .filter(|&&(_, key, doors, _)| {
                (collected_keys & key == 0) && (collected_keys & doors == doors)
            })
            .for_each(|&(position, key, _, key_distance)| {
                queue.push(Reverse((
                    distance + key_distance,
                    position,
                    collected_keys | key,
                )));
            });
    }

    bail!("unable to collect all keys")
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let mut map = parse_grid(&input)?;

    let result1 = solve(&map)?;

    for direction in iter::chain(DIRECTIONS, [0.into()]) {
        let index = map.get_index(map.initial_position + direction);
        map.tiles[index] = b'#';
    }

    let mut all_splitted_keys = [0u32; 4];

    let mut splitted_keys = <[SmallVec<_>; 4]>::default();
    for &key in &map.keys {
        let diff = key.0 - map.initial_position;
        let signum = Complex::new(diff.re.signum(), diff.im.signum());

        for (keys, all_keys, offset) in
            izip!(&mut splitted_keys, &mut all_splitted_keys, POSITION_OFFSETS)
        {
            if offset == signum {
                keys.push(key);
                *all_keys |= key.1;
            }
        }
    }

    let mut splitted_doors = <[HashMap<_, _>; 4]>::default();
    for (&position, &door) in &map.doors {
        let diff = position - map.initial_position;
        let signum = Complex::new(diff.re.signum(), diff.im.signum());

        for (doors, all_keys, offset) in
            izip!(&mut splitted_doors, &all_splitted_keys, POSITION_OFFSETS)
        {
            if offset == signum && all_keys | door == door {
                doors.insert(position, door);
            }
        }
    }

    let initial_position = map.initial_position;

    let result2 = izip!(splitted_keys, splitted_doors, POSITION_OFFSETS)
        .map(|(keys, doors, offset)| {
            map.initial_position = initial_position + offset;
            map.keys = keys;
            map.doors = doors;
            solve(&map)
        })
        .try_sum::<usize>()?;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
