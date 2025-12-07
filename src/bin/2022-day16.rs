use aoc::*;

use eyre::ensure;
use itertools::Itertools;
use regex::Regex;

use std::collections::VecDeque;
use std::collections::hash_map::{Entry, HashMap};
use std::iter;

struct Valve<'a> {
    id: u8,
    name: &'a str,
    flow: u64,
    links: Vec<&'a str>,
}

struct State {
    valve_id: u8,
    remaining: u64,
    pressure: u64,
    open_valves: u64,
}

fn compute_best_pressures(
    connections: &[HashMap<u8, (Vec<u8>, u64)>],
    start_valve_id: u8,
    total_time: u64,
) -> HashMap<u64, u64> {
    let initial_state = State {
        valve_id: start_valve_id,
        remaining: total_time,
        pressure: 0,
        open_valves: 0,
    };

    let mut current_states = vec![initial_state];
    let mut best_pressures = HashMap::new();

    while let Some(state) = current_states.pop() {
        best_pressures
            .entry(state.open_valves)
            .and_modify(|old| {
                if state.pressure > *old {
                    *old = state.pressure
                }
            })
            .or_insert(state.pressure);

        current_states.extend(connections[state.valve_id as usize].iter().flat_map(
            |(&target, &(ref path, target_flow))| {
                if target_flow == 0 || state.open_valves & (1 << target as u64) != 0 {
                    return None;
                }

                let reward = target_flow * state.remaining.checked_sub(path.len() as u64 + 1)?;

                Some(State {
                    valve_id: target,
                    remaining: state.remaining - path.len() as u64 - 1,
                    pressure: state.pressure + reward,
                    open_valves: state.open_valves | (1 << target as u64),
                })
            },
        ));
    }

    best_pressures
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let re =
        Regex::new(r#"(?m)^Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? (.+?)$"#)?;

    let mut valves_map = HashMap::new();

    let valves: Vec<_> = re
        .captures_iter(&input)
        .enumerate()
        .map(|(id, cap)| {
            let name = cap.get(1).value()?.as_str();
            let flow = cap[2].parse()?;
            let links = cap.get(3).value()?.as_str().split(", ").collect();

            valves_map.insert(name, id);

            Result::Ok(Valve {
                id: id as u8,
                name,
                flow,
                links,
            })
        })
        .try_collect()?;

    ensure!(valves.len() <= u64::BITS as usize, "input too long");

    let connections = valves
        .iter()
        .map(|start_valve| {
            let mut connections = HashMap::from([(start_valve.id, (vec![], start_valve.flow))]);
            let mut current = VecDeque::from([(start_valve.name, vec![])]);

            while let Some((name, path)) = current.pop_front() {
                for link in &valves[valves_map[name]].links {
                    let valve = &valves[valves_map[link]];

                    if let Entry::Vacant(entry) = connections.entry(valve.id) {
                        let new_path = iter::chain(&path, &[valve.id]).copied().collect_vec();
                        entry.insert((new_path.clone(), valve.flow));
                        current.push_back((link, new_path));
                    }
                }
            }
            connections
        })
        .collect_vec();

    let start_valve_id = valves.iter().position(|valve| valve.name == "AA").value()? as u8;

    let result1 = *compute_best_pressures(&connections, start_valve_id, 30)
        .values()
        .max()
        .value()?;

    let result2 = compute_best_pressures(&connections, start_valve_id, 26)
        .iter()
        .tuple_combinations()
        .filter(|&((&open_valves_1, _), (&open_valves_2, _))| {
            let count1 = open_valves_1.count_ones();
            let count2 = open_valves_2.count_ones();
            let count = (open_valves_1 | open_valves_2).count_ones();
            count == count1 + count2
        })
        .map(|((_, &pressure_1), (_, &pressure_2))| pressure_1 + pressure_2)
        .max()
        .value()?;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
