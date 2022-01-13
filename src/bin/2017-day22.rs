use aoc::*;

use num_complex::Complex;

use std::collections::HashMap;

#[derive(Copy, Clone)]
enum Node {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

impl Default for Node {
    fn default() -> Self {
        Node::Clean
    }
}

fn run1(mut grid: HashMap<Complex<i64>, Node>) -> usize {
    let mut current_position = Complex::new(0, 0);
    let mut direction = Complex::new(0, 1);
    let mut infection_count = 0usize;

    for _ in 0..10000 {
        let infected = grid.entry(current_position).or_default();
        match *infected {
            Node::Clean => {
                direction *= Complex::new(0, 1);
                infection_count += 1;
                *infected = Node::Infected;
            }
            Node::Infected => {
                direction *= Complex::new(0, -1);
                *infected = Node::Clean;
            }
            _ => (),
        }
        current_position += direction;
    }

    infection_count
}

fn run2(mut grid: HashMap<Complex<i64>, Node>) -> usize {
    let mut current_position = Complex::new(0, 0);
    let mut direction = Complex::new(0, 1);
    let mut infection_count = 0usize;

    for _ in 0..10_000_000 {
        let infected = grid.entry(current_position).or_default();
        match *infected {
            Node::Clean => {
                direction *= Complex::new(0, 1);
                *infected = Node::Weakened;
            }
            Node::Weakened => {
                infection_count += 1;
                *infected = Node::Infected;
            }
            Node::Infected => {
                direction *= Complex::new(0, -1);
                *infected = Node::Flagged;
            }
            Node::Flagged => {
                direction *= -1;
                *infected = Node::Clean;
            }
        }
        current_position += direction;
    }

    infection_count
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let mut grid = HashMap::new();

    let width = input.lines().next().value()?.len() as i64;
    let height = input.lines().count() as i64;

    for (row, line) in input.lines().enumerate() {
        for (column, x) in line.bytes().enumerate() {
            if x == b'#' {
                grid.insert(Complex::new(column as i64 - (width - 1) / 2, (height - 1) / 2 - row as i64), Node::Infected);
            }
        }
    }

    let result1 = run1(grid.clone());
    let result2 = run2(grid);

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
