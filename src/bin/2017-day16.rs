use aoc::*;

use eyre::bail;
use itertools::Itertools;

use std::collections::HashSet;

enum DanceMove {
    Spin(usize),
    Exchange(usize, usize),
    Partner(u8, u8),
}

impl DanceMove {
    fn execute(&self, programs: &mut [u8]) -> Result<()> {
        match *self {
            DanceMove::Spin(count) => programs.rotate_right(count),
            DanceMove::Exchange(pos1, pos2) => programs.swap(pos1, pos2),
            DanceMove::Partner(c1, c2) => {
                let pos1 = programs.iter().position(|&x| x == c1).value()?;
                let pos2 = programs.iter().position(|&x| x == c2).value()?;
                programs.swap(pos1, pos2);
            }
        }

        Ok(())
    }
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);
    let input = input.trim();

    let dance_moves: Vec<_> = input
        .split(',')
        .map(|x| {
            let (dance_move, args) = x.split_at(1);

            Ok(match dance_move {
                "s" => DanceMove::Spin(args.parse()?),
                "x" => {
                    let (arg1, arg2) = args.split('/').next_tuple().value()?;
                    DanceMove::Exchange(arg1.parse()?, arg2.parse()?)
                }
                "p" => {
                    let (arg1, arg2) = args.split('/').map(|x| x.as_bytes()[0]).next_tuple().value()?;
                    DanceMove::Partner(arg1, arg2)
                }
                other => bail!("unknown dance move: {other}"),
            })
        })
        .try_collect()?;

    let start_programs = b"abcdefghijklmnop";

    let mut programs = *start_programs;
    for dance_move in &dance_moves {
        dance_move.execute(&mut programs)?;
    }

    let result1 = String::from_utf8_lossy(&programs).into_owned();

    let mut uniques_states = vec![*start_programs];
    let mut previous_states: HashSet<_> = uniques_states.iter().copied().collect();

    while previous_states.insert(programs) {
        uniques_states.push(programs);

        for dance_move in &dance_moves {
            dance_move.execute(&mut programs)?;
        }
    }

    let result2 = String::from_utf8_lossy(&uniques_states[1_000_000_000 % uniques_states.len()]);

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
