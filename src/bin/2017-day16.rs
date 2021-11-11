use eyre::Result;
use itertools::Itertools;

use std::collections::HashSet;
use std::fs;

enum DanceMove {
    Spin(usize),
    Exchange(usize, usize),
    Partner(u8, u8),
}

impl DanceMove {
    fn execute(&self, programs: &mut [u8]) {
        match *self {
            DanceMove::Spin(count) => programs.rotate_right(count),
            DanceMove::Exchange(pos1, pos2) => programs.swap(pos1, pos2),
            DanceMove::Partner(c1, c2) => {
                let pos1 = programs.iter().position(|&x| x == c1).unwrap();
                let pos2 = programs.iter().position(|&x| x == c2).unwrap();
                programs.swap(pos1, pos2);
            }
        }
    }
}

fn main() -> Result<()> {
    let input = fs::read_to_string("inputs/2017-day16.txt")?;
    let input = input.trim();

    let dance_moves = input
        .split(',')
        .map(|x| {
            let (dance_move, args) = x.split_at(1);
            match dance_move {
                "s" => DanceMove::Spin(args.parse().unwrap()),
                "x" => {
                    let mut iter = args.split('/');
                    let arg1 = iter.next().unwrap().parse().unwrap();
                    let arg2 = iter.next().unwrap().parse().unwrap();
                    DanceMove::Exchange(arg1, arg2)
                }
                "p" => {
                    let mut iter = args.split('/');
                    let arg1 = iter.next().unwrap().as_bytes()[0];
                    let arg2 = iter.next().unwrap().as_bytes()[0];
                    DanceMove::Partner(arg1, arg2)
                }
                other => panic!("unknown dance move: {}", other),
            }
        })
        .collect_vec();

    let start_programs = b"abcdefghijklmnop";

    let mut programs = *start_programs;
    for dance_move in &dance_moves {
        dance_move.execute(&mut programs);
    }

    let result1 = String::from_utf8_lossy(&programs).into_owned();

    let mut uniques_states = vec![*start_programs];
    let mut previous_states: HashSet<_> = uniques_states.iter().copied().collect();

    while previous_states.insert(programs) {
        uniques_states.push(programs);

        for dance_move in &dance_moves {
            dance_move.execute(&mut programs);
        }
    }

    let result2 = String::from_utf8_lossy(&uniques_states[(1_000_000_000 % uniques_states.len())]);

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
