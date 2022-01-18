use aoc::*;

use eyre::bail;
use itertools::Itertools;
use regex::Regex;
use smallvec::{smallvec, SmallVec};

use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

type IndicesVec = Option<SmallVec<[usize; 2]>>;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Pair {
    chip_floor: i8,
    gen_floor: i8,
}

impl Pair {
    fn new(chip_floor: i8, gen_floor: i8) -> Self {
        Self { chip_floor, gen_floor }
    }
}

#[derive(Clone)]
struct State {
    pairs: SmallVec<[Pair; 8]>,
    elevator_floor: i8,
}

impl Eq for State {}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.pairs.eq(&other.pairs) && self.elevator_floor.eq(&other.elevator_floor)
    }
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for pair in &self.pairs {
            pair.hash(state);
        }
        self.elevator_floor.hash(state);
    }
}

impl State {
    fn new(pairs: SmallVec<[Pair; 8]>, elevator_floor: i8) -> Self {
        Self { pairs, elevator_floor }
    }

    fn is_valid(&self) -> bool {
        if self.elevator_floor < 0 || self.elevator_floor > 3 {
            false
        } else {
            self.pairs.iter().all(|pair| {
                if pair.chip_floor == pair.gen_floor {
                    true
                } else {
                    self.pairs.iter().all(|other_pair| other_pair.gen_floor != pair.chip_floor)
                }
            })
        }
    }

    fn is_final(&self) -> bool {
        self.pairs.iter().all(|pair| pair.chip_floor == 3 && pair.gen_floor == 3)
    }

    fn next_states<'a>(&'a self, possible_moves: &'a [(IndicesVec, IndicesVec)]) -> impl Iterator<Item = State> + 'a {
        possible_moves
            .iter()
            .filter(move |(chips, generators)| {
                if let Some(chips) = chips {
                    let check = chips.iter().all(|&chip_index| self.pairs[chip_index].chip_floor == self.elevator_floor);
                    if !check {
                        return false;
                    }
                }

                if let Some(generators) = generators {
                    let check = generators.iter().all(|&generator_index| self.pairs[generator_index].gen_floor == self.elevator_floor);
                    if !check {
                        return false;
                    }
                }

                true
            })
            .flat_map(move |(chips, generators)| {
                let mut new_states = [self.clone(), self.clone()];

                if let Some(chips) = chips {
                    for &chip_index in chips {
                        new_states[0].pairs[chip_index].chip_floor += 1;
                        new_states[1].pairs[chip_index].chip_floor -= 1;
                    }
                }
                if let Some(generators) = generators {
                    for &generator_index in generators {
                        new_states[0].pairs[generator_index].gen_floor += 1;
                        new_states[1].pairs[generator_index].gen_floor -= 1;
                    }
                }
                new_states[0].pairs.sort_unstable();
                new_states[1].pairs.sort_unstable();

                new_states[0].elevator_floor += 1;
                new_states[1].elevator_floor -= 1;

                new_states.into_iter().filter(|state| state.is_valid())
            })
    }
}

fn solve(state: &State) -> usize {
    // Possible moves: 1 chip or 1 generator or 2 chips or 2 generators or a pair of chip/generator of the same type
    let possible_moves = (0..state.pairs.len())
        .tuple_combinations()
        .flat_map(|(x,)| [(Some(smallvec![x]), None), (None, Some(smallvec![x])), (Some(smallvec![x]), Some(smallvec![x]))])
        .chain((0..state.pairs.len()).tuple_combinations().flat_map(|(x, y)| [(Some(smallvec![x, y]), None), (None, Some(smallvec![x, y]))]))
        .collect_vec();

    let mut current_states = vec![state.clone()];
    let mut previous_states = HashSet::from([state.clone()]);
    let mut next_states = Vec::new();

    let mut steps = 0;
    loop {
        steps += 1;

        for current_state in &current_states {
            for next_state in current_state.next_states(&possible_moves) {
                if next_state.is_final() {
                    return steps;
                }

                if !previous_states.contains(&next_state) {
                    previous_states.insert(next_state.clone());
                    next_states.push(next_state);
                }
            }
        }

        std::mem::swap(&mut current_states, &mut next_states);
        next_states.clear();
    }
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let regex_floor = Regex::new(r#"(\w+) floor"#)?;
    let regex_gen_chip = Regex::new(r#"a (\w+)( generator|-compatible microchip)"#)?;

    let mut chips = HashMap::new();
    let mut generators = HashMap::new();

    for line in input.lines() {
        let floor: i8 = match &regex_floor.captures(line).value()?[1] {
            "first" => 0,
            "second" => 1,
            "third" => 2,
            "fourth" => 3,
            other => bail!("unknown floor: {other}"),
        };

        for cap in regex_gen_chip.captures_iter(line) {
            let element_type = cap.get(1).value()?.as_str();
            match &cap[2] {
                "-compatible microchip" => chips.insert(element_type, floor),
                " generator" => generators.insert(element_type, floor),
                other => bail!("unknown element: {other}"),
            };
        }
    }

    let pairs = chips.iter().map(|(&name, &chip_floor)| Pair::new(chip_floor, generators[name])).sorted_unstable().collect();

    let mut state = State::new(pairs, 0);
    let result1 = solve(&state);

    state.pairs.push(Pair::new(0, 0));
    state.pairs.push(Pair::new(0, 0));
    state.pairs.sort_unstable();
    let result2 = solve(&state);

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
