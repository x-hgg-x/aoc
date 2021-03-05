use std::collections::HashSet;
use std::fs;
use std::mem::swap;

#[derive(Clone, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("inputs/2016-day13.txt")?;

    let favorite_number: i32 = input.trim().parse()?;

    let is_valid = |&Point { x, y }: &Point| {
        let is_pos = x >= 0 && y >= 0;
        is_pos && (x * x + 3 * x + 2 * x * y + y + y * y + favorite_number).count_ones() % 2 == 0
    };

    let start = Point::new(1, 1);
    let goal = Point::new(31, 39);

    let directions = [
        Point::new(-1, 0),
        Point::new(1, 0),
        Point::new(0, -1),
        Point::new(0, 1),
    ];

    let mut current_points = Vec::new();
    current_points.push(start.clone());

    let mut previous_points = HashSet::new();
    previous_points.insert(start);

    let mut next_points = Vec::new();

    let mut goal_steps = None;
    let mut reachable_points = None;

    let mut steps = 0;
    let (result1, result2) = loop {
        if let (Some(goal_steps), Some(reachable_points)) = (goal_steps, reachable_points) {
            break (goal_steps, reachable_points);
        }

        steps += 1;

        for current_point in &current_points {
            for direction in directions.iter() {
                let next_point = Point {
                    x: current_point.x + direction.x,
                    y: current_point.y + direction.y,
                };

                if next_point == goal {
                    goal_steps = Some(steps);
                }

                if !previous_points.contains(&next_point) && is_valid(&next_point) {
                    previous_points.insert(next_point.clone());
                    next_points.push(next_point);
                }
            }
        }

        swap(&mut current_points, &mut next_points);
        next_points.clear();

        if steps == 50 {
            reachable_points = Some(previous_points.len());
        }
    };

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
