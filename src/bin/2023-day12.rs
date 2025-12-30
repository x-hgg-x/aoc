use aoc::*;

use eyre::bail;
use itertools::Itertools;

use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::iter::repeat_n;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum SpringState {
    Unknown,
    Damaged,
    Operational,
}

impl SpringState {
    fn parse(x: u8) -> Result<Self> {
        match x {
            b'?' => Ok(Self::Unknown),
            b'#' => Ok(Self::Damaged),
            b'.' => Ok(Self::Operational),
            _ => bail!("unknown spring state"),
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
struct State {
    spring_index: usize,
    spring_state: Option<SpringState>,
    current_group_size: usize,
    group_index: usize,
}

impl State {
    fn compute_valid_arrangements(
        &self,
        springs: &[u8],
        groups: &[usize],
        cache: &mut HashMap<State, u64>,
    ) -> Result<u64> {
        let State {
            spring_index,
            spring_state,
            current_group_size,
            group_index,
        } = *self;

        match cache.entry(self.clone()) {
            Entry::Occupied(entry) => return Ok(*entry.get()),
            Entry::Vacant(entry) => {
                if spring_index == springs.len() + 1 {
                    let value = (group_index == groups.len()).into();
                    return Ok(*entry.insert(value));
                }
            }
        }

        let spring_state = match spring_state {
            Some(x) => x,
            None => springs
                .get(spring_index)
                .copied()
                .map(SpringState::parse)
                .transpose()?
                .unwrap_or(SpringState::Operational),
        };

        let value = match spring_state {
            SpringState::Unknown => {
                let states = [
                    State {
                        spring_index,
                        spring_state: Some(SpringState::Damaged),
                        current_group_size,
                        group_index,
                    },
                    State {
                        spring_index,
                        spring_state: Some(SpringState::Operational),
                        current_group_size,
                        group_index,
                    },
                ];

                states
                    .into_iter()
                    .map(|state| state.compute_valid_arrangements(springs, groups, cache))
                    .try_sum()?
            }
            SpringState::Damaged
                if group_index < groups.len() && current_group_size < groups[group_index] =>
            {
                let state = State {
                    spring_index: spring_index + 1,
                    spring_state: None,
                    current_group_size: current_group_size + 1,
                    group_index,
                };
                state.compute_valid_arrangements(springs, groups, cache)?
            }
            SpringState::Operational if current_group_size == 0 => {
                let state = State {
                    spring_index: spring_index + 1,
                    spring_state: None,
                    current_group_size: 0,
                    group_index,
                };
                state.compute_valid_arrangements(springs, groups, cache)?
            }
            SpringState::Operational if current_group_size == groups[group_index] => {
                let state = State {
                    spring_index: spring_index + 1,
                    spring_state: None,
                    current_group_size: 0,
                    group_index: group_index + 1,
                };
                state.compute_valid_arrangements(springs, groups, cache)?
            }
            _ => 0,
        };

        Ok(*cache.entry(self.clone()).insert_entry(value).get())
    }
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let spring_groups: Vec<_> = input
        .lines()
        .map(|line| {
            let (springs, groups) = line.split_ascii_whitespace().next_tuple().value()?;

            let springs_len = springs.len();
            let springs = repeat_n(springs, 5).join("?").into_bytes();

            let groups: Vec<_> = groups.split(',').map(|x| x.parse()).try_collect()?;

            let groups_len = groups.len();
            let groups = repeat_n(groups, 5).flatten().collect_vec();

            Result::Ok(((springs_len, springs), (groups_len, groups)))
        })
        .try_collect()?;

    let initial_state = State {
        spring_index: 0,
        spring_state: None,
        current_group_size: 0,
        group_index: 0,
    };

    let mut cache = HashMap::new();

    let result1 = spring_groups
        .iter()
        .map(|&((springs_len, ref springs), (groups_len, ref groups))| {
            cache.clear();

            initial_state.compute_valid_arrangements(
                &springs[..springs_len],
                &groups[..groups_len],
                &mut cache,
            )
        })
        .try_sum::<u64>()?;

    let result2 = spring_groups
        .iter()
        .map(|&((_, ref springs), (_, ref groups))| {
            cache.clear();

            initial_state.compute_valid_arrangements(springs, groups, &mut cache)
        })
        .try_sum::<u64>()?;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
