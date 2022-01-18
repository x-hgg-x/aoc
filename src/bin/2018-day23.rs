use aoc::*;

use itertools::Itertools;
use regex::Regex;

use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;
use std::iter;

const OCTANTS: [[i64; 3]; 8] = [[0, 0, 0], [0, 0, 1], [0, 1, 0], [0, 1, 1], [1, 0, 0], [1, 0, 1], [1, 1, 0], [1, 1, 1]];

struct Nanobot {
    position: [i64; 3],
    radius: i64,
}

struct State {
    intersecting_bots: usize,
    box_corner: [i64; 3],
    box_size: i64,
    box_corner_distance: i64,
}

impl State {
    fn new(intersecting_bots: usize, box_corner: [i64; 3], box_size: i64) -> Self {
        Self { intersecting_bots, box_corner, box_size, box_corner_distance: box_corner.into_iter().map(i64::abs).sum() }
    }

    fn estimate(&self) -> (Reverse<usize>, Reverse<i64>, i64) {
        (Reverse(self.intersecting_bots), Reverse(self.box_size), self.box_corner_distance)
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

fn intersecting_bots(box_corner: [i64; 3], box_size: i64, nanobots: &[Nanobot]) -> usize {
    nanobots
        .iter()
        .filter(|&nanobot| {
            let box_distance = iter::zip(nanobot.position, box_corner)
                .map(|(p, c)| {
                    let box_low = c;
                    let box_high = c + box_size - 1;

                    (box_low - p).max(0) + (p - box_high).max(0)
                })
                .sum::<i64>();

            box_distance <= nanobot.radius
        })
        .count()
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let re = Regex::new(r#"(?m)^pos=<(.+?),(.+?),(.+?)>, r=(.+?)$"#)?;

    let nanobots: Vec<_> = re
        .captures_iter(&input)
        .map(|cap| Result::Ok(Nanobot { position: [cap[1].trim().parse()?, cap[2].trim().parse()?, cap[3].trim().parse()?], radius: cap[4].trim().parse()? }))
        .try_collect()?;

    let largest_signal_nanobot = nanobots.iter().max_by_key(|nanobot| nanobot.radius).value()?;

    let result1 = nanobots
        .iter()
        .filter(|&nanobot| {
            let distance = nanobot.position.into_iter().zip(largest_signal_nanobot.position).map(|(p1, p2)| (p1 - p2).abs()).sum::<i64>();
            distance <= largest_signal_nanobot.radius
        })
        .count();

    let bounding_box = nanobots.iter().flat_map(|nanobot| nanobot.position.into_iter().map(|x| x.abs() as u64)).max().value()?;
    let max_box_size = i64::try_from(bounding_box.next_power_of_two())?;

    let initial_intersecting_bots = nanobots.len();
    let initial_box_corner = [-max_box_size; 3];
    let initial_box_size = 2 * max_box_size;
    let initial_state = State::new(initial_intersecting_bots, initial_box_corner, initial_box_size);

    let mut current_states = BinaryHeap::from([initial_state]);

    let result2 = loop {
        if let Some(state) = current_states.pop() {
            if state.box_size == 1 {
                break state.box_corner_distance;
            }

            let new_box_size = state.box_size / 2;

            for octant in OCTANTS {
                let mut new_box_corner = state.box_corner;
                new_box_corner.iter_mut().zip(octant).for_each(|(x, o)| *x += new_box_size * o);

                let new_intersecting_bots = intersecting_bots(new_box_corner, new_box_size, &nanobots);

                current_states.push(State::new(new_intersecting_bots, new_box_corner, new_box_size));
            }
        }
    };

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
