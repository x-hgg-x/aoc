use aoc::*;

use eyre::{bail, ensure};
use itertools::{izip, Itertools};

use std::iter::once;

const SIZE: usize = 100;
const WIDTH: usize = SIZE + 2;
const HEIGHT: usize = SIZE + 2;

struct Grid {
    lights: Vec<bool>,
    stuck: bool,
}

impl Grid {
    fn new(lights: Vec<bool>, stuck: bool) -> Result<Self> {
        ensure!(WIDTH * HEIGHT == lights.len(), "unable to construct Grid: width * height != lights.len()");

        let mut grid = Self { lights, stuck };

        if stuck {
            Self::stick_lights(&mut grid);
        }

        Ok(grid)
    }

    fn get_index(&self, row: usize, column: usize) -> usize {
        row * WIDTH + column
    }

    fn stick_lights(&mut self) {
        let stuck_index = [(1, 1), (1, WIDTH - 2), (HEIGHT - 2, 1), (HEIGHT - 2, WIDTH - 2)];

        for &(row, column) in &stuck_index {
            let index = self.get_index(row, column);
            self.lights[index] = true
        }
    }

    fn step(&mut self, n: u32, buf: &mut Vec<bool>) -> &mut Self {
        for _ in 0..n {
            let iter = self.lights.chunks_exact(WIDTH).tuple_windows().flat_map(|(row_0, row_1, row_2)| {
                let inner_iter = izip!(row_0.windows(3), row_1.windows(3), row_2.windows(3)).map(|(x0, x1, x2)| {
                    let center = x1[1];
                    let neighbor_lights = x0.iter().chain([&x1[0], &x1[2]]).chain(x2).copied().map_into::<usize>().sum::<usize>();

                    match (center, neighbor_lights) {
                        (true, 2..=3) => true,
                        (true, _) => false,
                        (false, 3) => true,
                        (center, _) => center,
                    }
                });

                once(false).chain(inner_iter).chain(once(false))
            });

            buf.clear();
            buf.extend([false; WIDTH].into_iter().chain(iter).chain([false; WIDTH]));
            std::mem::swap(buf, &mut self.lights);

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

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let lights = input
        .lines()
        .flat_map(|line| {
            let iter = line.bytes().map(|x| match x {
                b'.' => Ok(false),
                b'#' => Ok(true),
                _ => bail!("unknown tile"),
            });
            once(Ok(false)).chain(iter).chain(once(Ok(false)))
        })
        .try_process(|iter| [false; WIDTH].into_iter().chain(iter).chain([false; WIDTH]).collect_vec())?;

    let mut buf = Vec::with_capacity(lights.len());

    let result1 = Grid::new(lights.clone(), false)?.step(100, &mut buf).count();
    let result2 = Grid::new(lights, true)?.step(100, &mut buf).count();

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
