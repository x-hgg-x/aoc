use aoc::*;

use eyre::{bail, Report};
use itertools::Itertools;

use std::cmp::Ordering;
use std::collections::hash_map::{Entry, HashMap};
use std::collections::BinaryHeap;
use std::iter::once;

const X_FACTOR: usize = 16807;
const Y_FACTOR: usize = 48271;
const GEOLOGIC_INDEX_MODULO: usize = 20183;

struct Cave {
    erosion_levels: Vec<Vec<usize>>,
    size: (usize, usize),
    factor: (usize, usize),
    target_position: (usize, usize),
    depth: usize,
}

impl Cave {
    fn add_dim_0(&mut self) -> Result<()> {
        self.erosion_levels.first_mut().value()?.push((self.factor.0 * self.size.0 + self.depth) % GEOLOGIC_INDEX_MODULO);

        let mut remaining = self.erosion_levels.as_mut_slice();

        for _ in 0..self.size.1 - 1 {
            let (prev_row, row) = remaining.split_at_mut(1);
            row[0].push((prev_row[0].last().value()? * row[0].last().value()? + self.depth) % GEOLOGIC_INDEX_MODULO);
            remaining = row;
        }

        self.size.0 += 1;

        Ok(())
    }

    fn add_dim_1(&mut self) -> Result<()> {
        let first = (self.factor.1 * self.size.1 + self.depth) % GEOLOGIC_INDEX_MODULO;

        self.erosion_levels.push(
            once(first)
                .chain(self.erosion_levels.last().value()?[1..].iter().scan(first, |left, &top| {
                    *left = (*left * top + self.depth) % GEOLOGIC_INDEX_MODULO;
                    Some(*left)
                }))
                .collect(),
        );

        self.size.1 += 1;

        Ok(())
    }

    fn region(&mut self, position: (usize, usize)) -> Result<Region> {
        if position.0 >= self.size.0 {
            for _ in 0..=position.0 - self.size.0 {
                self.add_dim_0()?;
            }
        }

        if position.1 >= self.size.1 {
            for _ in 0..=position.1 - self.size.1 {
                self.add_dim_1()?;
            }
        }

        (self.erosion_levels[position.1][position.0] % 3).try_into()
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Region {
    Rocky,
    Wet,
    Narrow,
}

impl TryFrom<usize> for Region {
    type Error = Report;

    fn try_from(value: usize) -> Result<Self> {
        match value {
            0 => Ok(Self::Rocky),
            1 => Ok(Self::Wet),
            2 => Ok(Self::Narrow),
            _ => bail!("unknown region"),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Tool {
    None,
    Torch,
    ClimbingGear,
}

struct State {
    position: (usize, usize),
    region: Region,
    current_time: usize,
    tool: Tool,
    distance: usize,
}

impl State {
    fn new(position: (usize, usize), region: Region, current_time: usize, tool: Tool, cave: &Cave) -> Self {
        let abs_diff = (
            if position.0 >= cave.target_position.0 { position.0 - cave.target_position.0 } else { cave.target_position.0 - position.0 },
            if position.1 >= cave.target_position.1 { position.1 - cave.target_position.1 } else { cave.target_position.1 - position.1 },
        );

        Self { position, region, current_time, tool, distance: abs_diff.0 + abs_diff.1 }
    }

    fn estimate(&self) -> usize {
        self.current_time + self.distance
    }

    fn switchable_tool(&self) -> Result<Tool> {
        match (self.region, self.tool) {
            (Region::Rocky, Tool::None) | (Region::Wet, Tool::Torch) | (Region::Narrow, Tool::ClimbingGear) => bail!("incorrect tool for the region"),
            (Region::Rocky, Tool::Torch) => Ok(Tool::ClimbingGear),
            (Region::Rocky, Tool::ClimbingGear) => Ok(Tool::Torch),
            (Region::Wet, Tool::None) => Ok(Tool::ClimbingGear),
            (Region::Wet, Tool::ClimbingGear) => Ok(Tool::None),
            (Region::Narrow, Tool::None) => Ok(Tool::Torch),
            (Region::Narrow, Tool::Torch) => Ok(Tool::None),
        }
    }

    fn inaccessible_region(&self) -> Region {
        match self.tool {
            Tool::None => Region::Rocky,
            Tool::Torch => Region::Wet,
            Tool::ClimbingGear => Region::Narrow,
        }
    }
}

impl Eq for State {}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.estimate().eq(&other.estimate())
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.estimate().cmp(&self.estimate())
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn insert_new_state(
    previous_positions: &mut HashMap<((usize, usize), Tool), usize>,
    current_states: &mut BinaryHeap<State>,
    new_position: (usize, usize),
    new_region: Region,
    new_time: usize,
    new_tool: Tool,
    cave: &Cave,
) {
    match previous_positions.entry((new_position, new_tool)) {
        Entry::Occupied(mut entry) => {
            let old_time = entry.get_mut();
            if new_time >= *old_time {
                return;
            }
            *old_time = new_time;
        }
        Entry::Vacant(entry) => {
            entry.insert(new_time);
        }
    };

    current_states.push(State::new(new_position, new_region, new_time, new_tool, cave));
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let (depth_line, target_line) = input.lines().next_tuple().value()?;

    let depth = depth_line.split(": ").last().value()?.parse()?;
    let (target_x, target_y) = target_line.split(": ").last().value()?.split(',').map(|x| Ok(x.parse()?)).try_process(|mut iter| iter.next_tuple())?.value()?;

    let mut cave = if target_x >= target_y {
        Cave {
            erosion_levels: vec![(0..=target_x).map(|x| (X_FACTOR * x + depth) % GEOLOGIC_INDEX_MODULO).collect()],
            size: (target_x + 1, 1),
            factor: (X_FACTOR, Y_FACTOR),
            target_position: (target_x, target_y),
            depth,
        }
    } else {
        Cave {
            erosion_levels: vec![(0..=target_y).map(|y| (Y_FACTOR * y + depth) % GEOLOGIC_INDEX_MODULO).collect()],
            size: (target_y + 1, 1),
            factor: (Y_FACTOR, X_FACTOR),
            target_position: (target_y, target_x),
            depth,
        }
    };

    for _ in 0..cave.target_position.1 {
        cave.add_dim_1()?;
    }

    *cave.erosion_levels.last_mut().value()?.last_mut().value()? = depth % GEOLOGIC_INDEX_MODULO;

    let result1 = cave.erosion_levels.iter().flatten().map(|&level| level % 3).sum::<usize>();

    let initial_position = (0, 0);
    let initial_region = Region::Rocky;
    let initial_time = 0;
    let initial_tool = Tool::Torch;
    let initial_state = State::new(initial_position, initial_region, initial_time, initial_tool, &cave);

    let mut previous_positions = HashMap::new();
    previous_positions.insert((initial_position, initial_tool), initial_time);

    let mut current_states = BinaryHeap::new();
    current_states.push(initial_state);

    let result2 = loop {
        if let Some(state) = current_states.pop() {
            if state.position == cave.target_position && state.tool == Tool::Torch {
                break state.current_time;
            }

            insert_new_state(
                &mut previous_positions,
                &mut current_states,
                state.position,
                state.region,
                state.current_time + 7,
                state.switchable_tool()?,
                &cave,
            );

            let mut process_neighbors = |new_position| {
                let new_region = cave.region(new_position)?;
                let new_time = state.current_time + 1;

                if new_region != state.inaccessible_region() {
                    insert_new_state(&mut previous_positions, &mut current_states, new_position, new_region, new_time, state.tool, &cave);
                }

                Result::Ok(())
            };

            if state.position.0 > 0 {
                process_neighbors((state.position.0 - 1, state.position.1))?;
            }
            if state.position.1 > 0 {
                process_neighbors((state.position.0, state.position.1 - 1))?;
            }
            process_neighbors((state.position.0 + 1, state.position.1))?;
            process_neighbors((state.position.0, state.position.1 + 1))?;
        }
    };

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
