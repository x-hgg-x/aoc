use aoc::*;

use itertools::Itertools;

fn run(mut jumps: Vec<i64>, part2: bool) -> Result<usize> {
    let mut steps = 0;
    let mut ip = 0;
    let range = 0..jumps.len().try_into()?;

    while range.contains(&ip) {
        let jump = &mut jumps[ip as usize];
        ip += *jump;
        if part2 && *jump >= 3 {
            *jump -= 1;
        } else {
            *jump += 1;
        }
        steps += 1;
    }
    Ok(steps)
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let jumps: Vec<_> = input.split_ascii_whitespace().map(|x| x.parse::<i64>()).try_collect()?;

    let result1 = run(jumps.clone(), false)?;
    let result2 = run(jumps, true)?;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
