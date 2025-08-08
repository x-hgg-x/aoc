use aoc::*;

use itertools::{Itertools, izip};
use regex::Regex;

use std::iter;

struct State {
    resources: [u64; 4],
    robots: [u64; 4],
    remaining: u64,
}

impl State {
    fn max_geodes(&self) -> u64 {
        self.resources[3] + self.robots[3] * self.remaining
    }
}

struct Blueprint {
    id: u64,
    costs: [[u64; 4]; 4],
    max_robots: [u64; 4],
}

impl Blueprint {
    fn max_geodes(&self, remaining: u64) -> u64 {
        let mut max_geodes = 0;

        let initial_state = State { resources: [0; 4], robots: [1, 0, 0, 0], remaining };
        let mut current_states = vec![initial_state];

        while let Some(state) = current_states.pop() {
            current_states.extend((0..4).flat_map(|i_blueprint| {
                if state.robots[i_blueprint] >= self.max_robots[i_blueprint] {
                    return None;
                }

                let cost = &self.costs[i_blueprint];

                let wait_time = (0..4)
                    .map(|i_resource| {
                        if state.resources[i_resource] >= cost[i_resource] {
                            0
                        } else if state.robots[i_resource] == 0 {
                            u64::MAX - 1
                        } else {
                            (cost[i_resource] - state.resources[i_resource] - 1) / state.robots[i_resource] + 1
                        }
                    })
                    .max()
                    .unwrap_or_default();

                let elasped = wait_time + 1;
                if state.remaining <= elasped {
                    return None;
                }

                let new_remaining = state.remaining - elasped;

                let mut new_resources = state.resources;
                izip!(&mut new_resources, &state.robots, cost).for_each(|(resource, robot, c)| *resource = *resource + robot * elasped - c);

                let mut new_robots = state.robots;
                new_robots[i_blueprint] += 1;

                let new_state = State { resources: new_resources, robots: new_robots, remaining: new_remaining };
                let new_max_geodes = new_state.max_geodes();

                if new_max_geodes + (new_remaining * (new_remaining - 1)) / 2 > max_geodes {
                    max_geodes = max_geodes.max(new_max_geodes);
                    Some(new_state)
                } else {
                    None
                }
            }));
        }

        max_geodes
    }
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let re = Regex::new(concat!(
        r#"(?m)^Blueprint (\d+): Each ore robot costs (\d+) ore. "#,
        r#"Each clay robot costs (\d+) ore. "#,
        r#"Each obsidian robot costs (\d+) ore and (\d+) clay. "#,
        r#"Each geode robot costs (\d+) ore and (\d+) obsidian.$"#,
    ))?;

    let blueprints: Vec<_> = re
        .captures_iter(&input)
        .map(|cap| {
            let id = cap[1].parse()?;
            let ore_robot_ore_cost = cap[2].parse()?;
            let clay_robot_ore_cost = cap[3].parse()?;
            let obsidian_robot_ore_cost = cap[4].parse()?;
            let obsidian_robot_clay_cost = cap[5].parse()?;
            let geode_robot_ore_cost = cap[6].parse()?;
            let geode_robot_obsidian_cost = cap[7].parse()?;

            let costs = [
                [ore_robot_ore_cost, 0, 0, 0],
                [clay_robot_ore_cost, 0, 0, 0],
                [obsidian_robot_ore_cost, obsidian_robot_clay_cost, 0, 0],
                [geode_robot_ore_cost, 0, geode_robot_obsidian_cost, 0],
            ];

            let max_robots = costs.iter().fold([0, 0, 0, u64::MAX], |mut max_cost, cost| {
                iter::zip(&mut max_cost, cost).for_each(|(max, &c)| *max = c.max(*max));
                max_cost
            });

            Result::Ok(Blueprint { id, costs, max_robots })
        })
        .try_collect()?;

    let result1 = blueprints.iter().map(|blueprint| blueprint.id * blueprint.max_geodes(24)).sum::<u64>();
    let result2 = blueprints.iter().take(3).map(|blueprint| blueprint.max_geodes(32)).product::<u64>();

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
