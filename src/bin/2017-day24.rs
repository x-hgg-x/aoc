use aoc::*;

use itertools::Itertools;

struct State {
    last: u64,
    strength: u64,
    component_indices: Vec<usize>,
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let components: Vec<_> = input
        .lines()
        .map(|line| {
            let (left, right) = line
                .split('/')
                .map(|x| Ok(x.parse::<u64>()?))
                .try_process(|mut iter| iter.next_tuple())?
                .value()?;

            Result::Ok([left, right])
        })
        .try_collect()?;

    let mut queue = components
        .iter()
        .enumerate()
        .filter_map(|(index, &component)| match component {
            [0, last] | [last, 0] => Some(State {
                last,
                strength: last,
                component_indices: vec![index],
            }),
            _ => None,
        })
        .collect_vec();

    let mut max_strength = 0;
    let mut max_strength_longest_bridge = 0;
    let mut max_length = 0;

    while let Some(state) = queue.pop() {
        max_strength = max_strength.max(state.strength);

        if state.component_indices.len() >= max_length {
            max_length = state.component_indices.len();
            max_strength_longest_bridge = max_strength_longest_bridge.max(state.strength);
        }

        for (index, component) in components.iter().enumerate() {
            if let Some(position) = component.iter().position(|&x| x == state.last)
                && !state.component_indices.contains(&index)
            {
                let new_last = component[position ^ 1];
                let new_strength = state.strength + component.iter().sum::<u64>();
                let mut new_component_indices = state.component_indices.clone();
                new_component_indices.push(index);

                queue.push(State {
                    last: new_last,
                    strength: new_strength,
                    component_indices: new_component_indices,
                })
            }
        }
    }

    let result1 = max_strength;
    let result2 = max_strength_longest_bridge;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
