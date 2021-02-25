use regex::Regex;

use std::fs;

type F = Box<dyn Fn(&str) -> Box<dyn Fn(i8) -> i8>>;

fn parse_regex<'a>(re: &Regex, text: &'a str) -> (&'a str, usize, usize, usize, usize) {
    re.captures_iter(text)
        .map(|x| {
            (
                x.get(1).unwrap().as_str(),
                x[2].parse().unwrap(),
                x[3].parse().unwrap(),
                x[4].parse().unwrap(),
                x[5].parse().unwrap(),
            )
        })
        .next()
        .unwrap()
}

fn set_lights(input: &str, re: &Regex, grid: &mut [[i8; 1000]; 1000], f: F) {
    for line in input.lines() {
        let (instruction, start_line, start_col, end_line, end_col) = parse_regex(&re, line);

        let func = f(&instruction);

        for grid_line in &mut grid[start_line..=end_line] {
            for elem in &mut grid_line[start_col..=end_col] {
                *elem = func(*elem);
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("inputs/2015-day06.txt")?;

    let re = Regex::new(r#"(.*?) (\d+),(\d+) through (\d+),(\d+)"#).unwrap();

    let f1: F = Box::new(|instruction| {
        Box::new(match instruction {
            "turn on" => |x: i8| x * x,
            "turn off" => |x: i8| -x * x,
            "toggle" => |x: i8| -x,
            other => panic!("unknown instruction: {}", other),
        })
    });

    let f2: F = Box::new(|instruction| {
        Box::new(match instruction {
            "turn on" => |x: i8| x + 1,
            "turn off" => |x: i8| (x - 1).max(0),
            "toggle" => |x: i8| x + 2,
            other => panic!("unknown instruction: {}", other),
        })
    });

    let mut grid = [[-1_i8; 1000]; 1000];
    set_lights(&input, &re, &mut grid, f1);

    let result1: usize = grid
        .iter()
        .map(|x| x.iter().filter(|&&x| x == 1).count())
        .sum();

    grid = [[0_i8; 1000]; 1000];
    set_lights(&input, &re, &mut grid, f2);

    let result2: i32 = grid
        .iter()
        .map(|x| x.iter().map(|&x| -> i32 { x.into() }).sum::<i32>())
        .sum();

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
