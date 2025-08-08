use aoc::*;

use itertools::{Itertools, iproduct};

use std::collections::HashMap;

#[derive(Clone, Eq, PartialEq, Hash)]
struct Game {
    positions: [u16; 2],
    scores: [u16; 2],
}

impl Game {
    fn new(positions: [u16; 2]) -> Self {
        Self { positions, scores: [0, 0] }
    }

    fn advance(&mut self, index: usize, dice: u16) {
        self.positions[index] = (self.positions[index] + dice - 1) % 10 + 1;
        self.scores[index] += self.positions[index];
    }

    fn play_deterministic(&mut self) -> u64 {
        let mut dice_count = 0;
        let mut index = 0;

        let mut dice_iter = (1..=100).cycle().tuples().map(|(x, y, z)| x + y + z);

        loop {
            if let Some(dice) = dice_iter.next() {
                self.advance(index, dice);
                dice_count += 3;

                if self.scores[index] >= 1000 {
                    return dice_count * self.scores[0].min(self.scores[1]) as u64;
                }
            }

            index ^= 1;
        }
    }

    fn play_dirac(self) -> u64 {
        let dices_sums = iproduct!(1..=3, 1..=3, 1..=3)
            .map(|(x, y, z)| x + y + z)
            .sorted_unstable()
            .dedup_with_count()
            .map(|(count, dice_sum)| (count as u64, dice_sum))
            .collect_vec();

        let mut index = 0;
        let mut wins = [0, 0];

        let mut games = HashMap::from([(self, 1)]);
        let mut buf = HashMap::new();

        loop {
            buf.clear();

            for (game, universe_count) in &games {
                for &(count, dice) in &dices_sums {
                    let mut next_game = game.clone();
                    next_game.advance(index, dice);

                    if next_game.scores[index] >= 21 {
                        wins[index] += universe_count * count;
                    } else {
                        *buf.entry(next_game).or_default() += universe_count * count;
                    }
                }
            }

            std::mem::swap(&mut games, &mut buf);

            if games.is_empty() {
                break;
            }

            index ^= 1;
        }

        wins[0].max(wins[1])
    }
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let initial_positions = input
        .lines()
        .map(|line| Ok(line.split_whitespace().last().value()?.parse()?))
        .try_process(|mut iter| iter.next_tuple().map(|(x1, x2)| [x1, x2]))?
        .value()?;

    let result1 = Game::new(initial_positions).play_deterministic();
    let result2 = Game::new(initial_positions).play_dirac();

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
