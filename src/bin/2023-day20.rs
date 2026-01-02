use aoc::*;

use eyre::{bail, ensure};
use itertools::Itertools;
use regex::Regex;
use smallvec::SmallVec;

use std::collections::{HashMap, VecDeque};

#[derive(Copy, Clone)]
enum Pulse {
    Low = 0,
    High = 1,
}

enum ModuleState<'a> {
    None,
    FlipFlop(bool),
    Conjunction(HashMap<&'a str, Pulse>),
}

struct Module<'a> {
    state: ModuleState<'a>,
    destinations: Vec<&'a str>,
}

fn gcd(mut x: u64, mut y: u64) -> u64 {
    while y != 0 {
        (x, y) = (y, x % y);
    }
    x
}

fn lcm(x: u64, y: u64) -> u64 {
    x * y / gcd(x, y)
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let re = Regex::new(r#"(?m)^([%&]?)(\w+) -> (.+)$"#)?;

    let mut graph = HashMap::new();
    let mut inverted_graph = HashMap::<_, Vec<_>>::new();

    for cap in re.captures_iter(&input) {
        let state = match &cap[1] {
            "" => ModuleState::None,
            "%" => ModuleState::FlipFlop(false),
            "&" => ModuleState::Conjunction(HashMap::new()),
            _ => bail!("invalid prefix"),
        };

        let name = cap.get(2).value()?.as_str();
        let destinations = cap.get(3).value()?.as_str().split(", ").collect_vec();

        let module = Module {
            state,
            destinations: destinations.clone(),
        };

        graph.insert(name, module);
        inverted_graph.entry(name).or_default();

        for destination in destinations {
            inverted_graph.entry(destination).or_default().push(name);
        }
    }

    for (name, module) in &mut graph {
        if let ModuleState::Conjunction(memory) = &mut module.state {
            *memory = inverted_graph[name]
                .iter()
                .map(|&input_name| (input_name, Pulse::Low))
                .collect();
        }
    }

    let rx_inputs = &inverted_graph["rx"];
    ensure!(rx_inputs.len() == 1, "unsupported input");

    let rx_input = &graph[rx_inputs[0]];

    ensure!(
        matches!(rx_input.state, ModuleState::Conjunction(_)),
        "unsupported input"
    );

    let mut cycles: HashMap<_, SmallVec<[u64; 2]>> = inverted_graph[rx_inputs[0]]
        .iter()
        .map(|&name| (name, SmallVec::new()))
        .collect();

    let mut presses = 0u64;
    let mut pulse_counts = [0i64; 2];
    let mut pulse_count_product = None;

    let mut current_pulses = VecDeque::new();

    let initial_pulses = (graph["broadcaster"].destinations.iter())
        .map(|&destination| ("broadcaster", Pulse::Low, destination))
        .collect_vec();

    let (result1, result2) = 'outer: loop {
        presses += 1;

        current_pulses.clear();
        current_pulses.extend(initial_pulses.iter().copied());

        pulse_counts[0] += 1;

        while let Some((input_name, pulse, output_name)) = current_pulses.pop_front() {
            pulse_counts[pulse as usize] += 1;

            let Some(module) = graph.get_mut(output_name) else {
                continue;
            };

            if matches!(pulse, Pulse::High)
                && let Some(cycle) = cycles.get_mut(input_name)
            {
                match cycle.len() {
                    0 => cycle.push(presses),
                    1 => {
                        ensure!(presses - cycle[0] == cycle[0]);
                        cycle.push(presses);
                    }
                    _ => (),
                }
            }

            if cycles.values().all(|cycle| cycle.len() == 2) {
                let result1 = pulse_count_product.value()?;
                let result2 = cycles.values().map(|x| x[0]).fold(1, lcm);
                break 'outer (result1, result2);
            }

            match &mut module.state {
                ModuleState::None => (),
                ModuleState::FlipFlop(state) => {
                    if matches!(pulse, Pulse::Low) {
                        let new_pulse = if *state { Pulse::Low } else { Pulse::High };

                        current_pulses.extend(
                            (module.destinations.iter()).map(|&x| (output_name, new_pulse, x)),
                        );

                        *state = !*state;
                    }
                }
                ModuleState::Conjunction(memory) => {
                    *memory.get_mut(input_name).value()? = pulse;

                    let new_pulse = if memory.values().all(|pulse| matches!(pulse, Pulse::High)) {
                        Pulse::Low
                    } else {
                        Pulse::High
                    };

                    current_pulses
                        .extend((module.destinations.iter()).map(|&x| (output_name, new_pulse, x)));
                }
            }
        }

        if presses == 1000 {
            pulse_count_product = Some(pulse_counts.iter().product::<i64>());
        }
    };

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
