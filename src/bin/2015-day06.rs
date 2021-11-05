use eyre::Result;
use itertools::Itertools;
use regex::Regex;

use std::fs;

fn parse_regex<'a>(re: &Regex, text: &'a str) -> (&'a str, usize, usize, usize, usize) {
    re.captures_iter(text)
        .map(|x| (x.get(1).unwrap().as_str(), x[2].parse().unwrap(), x[3].parse().unwrap(), x[4].parse().unwrap(), x[5].parse().unwrap()))
        .next()
        .unwrap()
}

fn set_lights<'a, F, Func>(input: &'a str, re: &Regex, grid: &mut [[i8; 1000]; 1000], f: F)
where
    F: Fn(&'a str) -> Func,
    Func: Fn(i8) -> i8,
{
    for line in input.lines() {
        let (instruction, start_line, start_col, end_line, end_col) = parse_regex(re, line);

        let func = f(instruction);

        for grid_line in &mut grid[start_line..=end_line] {
            for elem in &mut grid_line[start_col..=end_col] {
                *elem = func(*elem);
            }
        }
    }
}

fn main() -> Result<()> {
    let input = fs::read_to_string("inputs/2015-day06.txt")?;

    let re = Regex::new(r#"(.*?) (\d+),(\d+) through (\d+),(\d+)"#)?;

    let f1 = |instruction| match instruction {
        "turn on" => |x: i8| x * x,
        "turn off" => |x: i8| -x * x,
        "toggle" => |x: i8| -x,
        other => panic!("unknown instruction: {}", other),
    };

    let f2 = |instruction| match instruction {
        "turn on" => |x: i8| x + 1,
        "turn off" => |x: i8| (x - 1).max(0),
        "toggle" => |x: i8| x + 2,
        other => panic!("unknown instruction: {}", other),
    };

    let mut grid = [[-1_i8; 1000]; 1000];
    set_lights(&input, &re, &mut grid, f1);

    let result1 = grid.iter().map(|x| x.iter().filter(|&&x| x == 1).count()).sum::<usize>();

    grid = [[0_i8; 1000]; 1000];
    set_lights(&input, &re, &mut grid, f2);

    let result2 = grid.iter().map(|x| x.iter().copied().map_into::<i64>().sum::<i64>()).sum::<i64>();

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
