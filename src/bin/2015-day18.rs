use itertools::{iproduct, Itertools};

use std::fs;

struct Grid {
    width: usize,
    height: usize,
    lights: Vec<bool>,
    stuck: bool,
}

impl Grid {
    fn new(width: usize, height: usize, lights: Vec<bool>, stuck: bool) -> Result<Self, &'static str> {
        if width * height != lights.len() {
            return Err("unable to construct Grid: width * height != lights.len()");
        }

        let mut grid = Self { width, height, lights, stuck };

        if stuck {
            Self::stick_lights(&mut grid);
        }
        Ok(grid)
    }

    fn get_index(&self, row: usize, column: usize) -> usize {
        row * self.width + column
    }

    fn neighbor_lights(&self, row: usize, column: usize, state: bool) -> usize {
        let rows = row.saturating_sub(1)..=(row + 1).min(self.height - 1);
        let columns = column.saturating_sub(1)..=(column + 1).min(self.width - 1);

        iproduct!(rows, columns).map(|(row, column)| self.lights[self.get_index(row, column)]).filter(|&x| x).count() - state as usize
    }

    fn stick_lights(&mut self) {
        let stuck_index = [(0, 0), (0, self.width - 1), (self.height - 1, 0), (self.height - 1, self.width - 1)];

        for &(row, column) in &stuck_index {
            let index = self.get_index(row, column);
            self.lights[index] = true
        }
    }

    fn step(&mut self, n: u32) -> &mut Self {
        for _ in 0..n {
            self.lights = iproduct!(0..self.height, 0..self.width)
                .map(|(row, column)| {
                    let index = self.get_index(row, column);
                    let light_state = self.lights[index];
                    match (light_state, self.neighbor_lights(row, column, light_state)) {
                        (true, 2..=3) => true,
                        (true, _) => false,
                        (false, 3) => true,
                        (state, _) => state,
                    }
                })
                .collect();

            if self.stuck {
                self.stick_lights();
            }
        }
        self
    }

    fn count(&self) -> usize {
        self.lights.iter().filter(|&&x| x).count()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("inputs/2015-day18.txt")?;

    let lights = input
        .chars()
        .filter_map(|c| match c {
            '.' => Some(false),
            '#' => Some(true),
            _ => None,
        })
        .collect_vec();

    let result1 = Grid::new(100, 100, lights.clone(), false)?.step(100).count();
    let result2 = Grid::new(100, 100, lights, true)?.step(100).count();

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
