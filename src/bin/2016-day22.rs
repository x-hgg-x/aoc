use eyre::{ensure, Result};
use itertools::Itertools;
use regex::Regex;

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::fs;

struct Node {
    size: u64,
    used: u64,
}

struct Grid {
    width: usize,
    height: usize,
    nodes: Vec<Node>,
}

impl Grid {
    fn new(width: usize, height: usize, nodes: Vec<Node>) -> Result<Self> {
        ensure!(width * height == nodes.len(), "unable to construct Grid: width * height != nodes.len()");
        Ok(Self { width, height, nodes })
    }

    fn get_index(&self, row: usize, column: usize) -> usize {
        column * self.height + row
    }
}

struct State {
    hole_row: usize,
    hole_column: usize,
    steps: usize,
    distance: usize,
}

impl State {
    fn new(hole_row: usize, hole_column: usize, goal_row: usize, goal_column: usize, steps: usize) -> Self {
        let abs_diff_x = if hole_row >= goal_row { hole_row - goal_row } else { goal_row - hole_row };
        let abs_diff_y = if hole_column >= goal_column { hole_column - goal_column } else { goal_column - hole_column };
        let distance = abs_diff_x + abs_diff_y;

        Self { hole_row, hole_column, steps, distance }
    }
}

impl Eq for State {}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.distance.eq(&other.distance)
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn check_grid(grid: &Grid) -> Result<()> {
    let mut max_used = 0;
    let mut min_size = u64::MAX;
    for node in grid.nodes.chunks_exact(grid.height).flat_map(|nodes| &nodes[..2]) {
        max_used = max_used.max(node.used);
        min_size = min_size.min(node.size);
    }
    ensure!(max_used <= min_size, "no direct path from goal to start");
    Ok(())
}

fn main() -> Result<()> {
    let input = fs::read_to_string("inputs/2016-day22.txt")?;

    let regex_grid_size = Regex::new(r#"/dev/grid/node-x(\d+)-y(\d+)"#)?;
    let regex_node = Regex::new(r#"(?m)^\S+\s+(\d+)T\s+(\d+)T\s+\d+T\s+\d+%$"#)?;

    let (width, height) = (|| {
        let cap = regex_grid_size.captures(input.lines().last()?)?;
        Some((cap.get(1)?.as_str().parse::<usize>().unwrap() + 1, cap.get(2)?.as_str().parse::<usize>().unwrap() + 1))
    })()
    .unwrap();

    let nodes = regex_node.captures_iter(&input).map(|cap| Node { size: cap[1].parse().unwrap(), used: cap[2].parse().unwrap() }).collect_vec();
    let grid = Grid::new(width, height, nodes)?;

    let result1 = grid
        .nodes
        .iter()
        .tuple_combinations()
        .map(|(node1, node2)| {
            let total_used = node1.used + node2.used;
            (node2.used != 0 && total_used <= node1.size) as usize + (node1.used != 0 && total_used <= node2.size) as usize
        })
        .sum::<usize>();

    check_grid(&grid)?;

    let initial_hole_index = grid.nodes.iter().position(|node| node.used == 0).unwrap();
    let initial_hole_row = initial_hole_index % grid.height;
    let initial_hole_column = initial_hole_index / grid.height;

    let goal_row = 0;
    let goal_column = grid.width - 2;

    let mut previous_hole_indices = HashSet::new();
    previous_hole_indices.insert(initial_hole_index);

    let mut current_states = BinaryHeap::new();
    current_states.push(State::new(initial_hole_row, initial_hole_column, goal_row, goal_column, 0));

    let steps = loop {
        if let Some(state) = current_states.pop() {
            if (state.hole_row, state.hole_column) == (goal_row, goal_column) {
                break state.steps;
            }

            let state_hole_index = grid.get_index(state.hole_row, state.hole_column);
            let state_hole_node_size = grid.nodes[state_hole_index].size;

            if state.hole_row > 0 {
                let (new_hole_row, new_hole_column) = (state.hole_row - 1, state.hole_column);
                let new_hole_index = grid.get_index(new_hole_row, new_hole_column);
                if grid.nodes[new_hole_index].used <= state_hole_node_size && previous_hole_indices.insert(new_hole_index) {
                    current_states.push(State::new(new_hole_row, new_hole_column, goal_row, goal_column, state.steps + 1));
                }
            }
            if state.hole_row < height - 1 {
                let (new_hole_row, new_hole_column) = (state.hole_row + 1, state.hole_column);
                let new_hole_index = grid.get_index(new_hole_row, new_hole_column);
                if grid.nodes[new_hole_index].used <= state_hole_node_size && previous_hole_indices.insert(new_hole_index) {
                    current_states.push(State::new(new_hole_row, new_hole_column, goal_row, goal_column, state.steps + 1));
                }
            }
            if state.hole_column > 0 {
                let (new_hole_row, new_hole_column) = (state.hole_row, state.hole_column - 1);
                let new_hole_index = grid.get_index(new_hole_row, new_hole_column);
                if grid.nodes[new_hole_index].used <= state_hole_node_size && previous_hole_indices.insert(new_hole_index) {
                    current_states.push(State::new(new_hole_row, new_hole_column, goal_row, goal_column, state.steps + 1));
                }
            }
            if state.hole_column < width - 1 {
                let (new_hole_row, new_hole_column) = (state.hole_row, state.hole_column + 1);
                let new_hole_index = grid.get_index(new_hole_row, new_hole_column);
                if grid.nodes[new_hole_index].used <= state_hole_node_size && previous_hole_indices.insert(new_hole_index) {
                    current_states.push(State::new(new_hole_row, new_hole_column, goal_row, goal_column, state.steps + 1));
                }
            }
        }
    };

    let result2 = steps + 5 * (goal_column) + 1;

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
