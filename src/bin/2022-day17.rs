use aoc::*;

use itertools::Itertools;
use num_complex::Complex;

use std::collections::HashSet;
use std::collections::hash_map::{Entry, HashMap};

const WIDTH: usize = 7;

const SHAPES: &[[Complex<u64>; 5]; 5] = &[
    [Complex::new(2, 0), Complex::new(2, 0), Complex::new(3, 0), Complex::new(4, 0), Complex::new(5, 0)],
    [Complex::new(2, 1), Complex::new(3, 1), Complex::new(3, 2), Complex::new(3, 0), Complex::new(4, 1)],
    [Complex::new(2, 0), Complex::new(3, 0), Complex::new(4, 0), Complex::new(4, 1), Complex::new(4, 2)],
    [Complex::new(2, 0), Complex::new(2, 0), Complex::new(2, 1), Complex::new(2, 2), Complex::new(2, 3)],
    [Complex::new(2, 0), Complex::new(2, 0), Complex::new(3, 0), Complex::new(2, 1), Complex::new(3, 1)],
];

#[derive(Copy, Clone)]
enum Direction {
    Left,
    Right,
}

#[derive(Eq, PartialEq, Hash)]
struct CycleState {
    relative_heights: [u64; WIDTH],
    shape_index: usize,
    jet_index: usize,
}

struct CycleInfos {
    highest: u64,
    total_shapes: usize,
}

#[derive(Default)]
struct State {
    max_heights: [u64; WIDTH],
    highest: u64,
    jet_index: usize,
    total_shapes: usize,
}

impl State {
    fn step(&mut self, chamber: &mut HashSet<Complex<u64>>, jets: &[Direction]) {
        let mut rocks = SHAPES[self.total_shapes % SHAPES.len()];

        for rock in &mut rocks {
            rock.im += self.highest + 4;
        }

        loop {
            match jets[self.jet_index % jets.len()] {
                Direction::Left => {
                    if rocks[0].re > 0 && rocks.iter().all(|rock| !chamber.contains(&(rock - Complex::new(1, 0)))) {
                        for rock in &mut rocks {
                            rock.re -= 1;
                        }
                    }
                }
                Direction::Right => {
                    if rocks[4].re < 6 && rocks.iter().all(|rock| !chamber.contains(&(rock + Complex::new(1, 0)))) {
                        for rock in &mut rocks {
                            rock.re += 1;
                        }
                    }
                }
            }

            self.jet_index += 1;

            if rocks.iter().all(|rock| !chamber.contains(&(rock - Complex::new(0, 1)))) {
                for rock in &mut rocks {
                    rock.im -= 1;
                }
            } else {
                for &rock in rocks.iter() {
                    chamber.insert(rock);
                    let max_height = &mut self.max_heights[rock.re as usize];
                    *max_height = rock.im.max(*max_height);
                }
                self.highest = rocks.iter().fold(self.highest, |max_height, rock| max_height.max(rock.im));
                break;
            }
        }

        self.total_shapes += 1;
    }
}

fn run(jets: &[Direction], count: usize, cycle_detection: bool) -> u64 {
    let mut chamber: HashSet<_> = (0..WIDTH as u64).map(|x| Complex::new(x, 0)).collect();
    let mut state = State::default();

    let mut cycle_states = HashMap::<CycleState, CycleInfos>::new();
    let mut cycle_found = false;
    let mut skipped_cycles = 0;
    let mut height_gain_in_cycle = 0;

    while state.total_shapes < count {
        state.step(&mut chamber, jets);

        if cycle_detection && !cycle_found {
            let min_height = state.max_heights.iter().copied().min().unwrap_or_default();
            let mut relative_heights = state.max_heights;
            for x in &mut relative_heights {
                *x -= min_height;
            }

            let cycle_state = CycleState { relative_heights, shape_index: state.total_shapes % SHAPES.len(), jet_index: state.jet_index % jets.len() };

            match cycle_states.entry(cycle_state) {
                Entry::Vacant(entry) => {
                    entry.insert(CycleInfos { highest: state.highest, total_shapes: state.total_shapes });
                }
                Entry::Occupied(entry) => {
                    let cycle_infos = entry.get();
                    let shapes_in_cycle = state.total_shapes - cycle_infos.total_shapes;
                    height_gain_in_cycle = state.highest - cycle_infos.highest;
                    skipped_cycles = (count - state.total_shapes) / shapes_in_cycle;
                    state.total_shapes += skipped_cycles * shapes_in_cycle;
                    cycle_found = true;
                }
            };
        }
    }

    state.highest + skipped_cycles as u64 * height_gain_in_cycle
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let jets = input
        .bytes()
        .flat_map(|x| match x {
            b'<' => Some(Direction::Left),
            b'>' => Some(Direction::Right),
            _ => None,
        })
        .collect_vec();

    let result1 = run(&jets, 2022, false);
    let result2 = run(&jets, 1_000_000_000_000, true);

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
