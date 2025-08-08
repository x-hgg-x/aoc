use aoc::*;

use eyre::bail;
use itertools::Itertools;
use smallvec::SmallVec;

use std::collections::{HashSet, VecDeque};
use std::iter::{self, once};

enum GameResult {
    Player1Win(VecDeque<u8>),
    Player2Win(VecDeque<u8>),
}

struct Game {
    player1_cards: VecDeque<u8>,
    player2_cards: VecDeque<u8>,
    previous_states: HashSet<SmallVec<[u8; 51]>>,
}

impl Game {
    fn new(player1_cards: VecDeque<u8>, player2_cards: VecDeque<u8>) -> Self {
        Self {
            player1_cards,
            player2_cards,
            previous_states: HashSet::new(),
        }
    }
}

fn compute_score(cards: &VecDeque<u8>) -> u64 {
    iter::zip(1.., cards.iter().rev())
        .map(|(index, &card)| index * card as u64)
        .sum()
}

fn play_normal_game(mut game: Game) -> Result<u64> {
    loop {
        match (
            game.player1_cards.pop_front(),
            game.player2_cards.pop_front(),
        ) {
            (Some(card1), Some(card2)) => {
                if card1 < card2 {
                    game.player2_cards.extend([card2, card1]);
                } else {
                    game.player1_cards.extend([card1, card2]);
                }
            }
            (None, Some(card2)) => {
                game.player2_cards.push_front(card2);
                return Ok(compute_score(&game.player2_cards));
            }
            (Some(card1), None) => {
                game.player1_cards.push_front(card1);
                return Ok(compute_score(&game.player1_cards));
            }
            (None, None) => bail!("invalid configuration"),
        }
    }
}

fn play_recursive_game(game: Game) -> Result<u64> {
    let mut sub_games = vec![game];
    let mut played_cards = Vec::new();
    let mut last_result = None;

    loop {
        match sub_games.last_mut() {
            None => {
                let winner_cards = match last_result.value()? {
                    GameResult::Player1Win(cards) => cards,
                    GameResult::Player2Win(cards) => cards,
                };
                return Ok(compute_score(&winner_cards));
            }
            Some(Game {
                player1_cards,
                player2_cards,
                previous_states,
            }) => {
                if !previous_states.insert(
                    once(&(player1_cards.len() as u8))
                        .chain(player1_cards.iter())
                        .chain(player2_cards.iter())
                        .copied()
                        .collect(),
                ) {
                    last_result = Some(GameResult::Player1Win(
                        sub_games.pop().value()?.player1_cards,
                    ));

                    if let Some(game) = sub_games.last_mut() {
                        let (card1, card2) = played_cards.pop().value()?;
                        game.player1_cards.extend([card1, card2]);
                    }

                    continue;
                }

                match (player1_cards.pop_front(), player2_cards.pop_front()) {
                    (Some(card1), Some(card2)) => {
                        let card1_value = card1 as usize;
                        let card2_value = card2 as usize;

                        if player1_cards.len() >= card1_value && player2_cards.len() >= card2_value
                        {
                            let player1_new_cards =
                                player1_cards.iter().copied().take(card1_value).collect();

                            let player2_new_cards =
                                player2_cards.iter().copied().take(card2_value).collect();

                            sub_games.push(Game::new(player1_new_cards, player2_new_cards));
                            played_cards.push((card1, card2));
                        } else if card1 < card2 {
                            player2_cards.extend([card2, card1]);
                        } else {
                            player1_cards.extend([card1, card2]);
                        }
                    }
                    (None, Some(card2)) => {
                        player2_cards.push_front(card2);

                        last_result = Some(GameResult::Player2Win(
                            sub_games.pop().value()?.player2_cards,
                        ));

                        if let Some(game) = sub_games.last_mut() {
                            let (card1, card2) = played_cards.pop().value()?;
                            game.player2_cards.extend([card2, card1]);
                        }
                    }
                    (Some(card1), None) => {
                        player1_cards.push_front(card1);

                        last_result = Some(GameResult::Player1Win(
                            sub_games.pop().value()?.player1_cards,
                        ));

                        if let Some(game) = sub_games.last_mut() {
                            let (card1, card2) = played_cards.pop().value()?;
                            game.player1_cards.extend([card1, card2]);
                        }
                    }
                    (None, None) => bail!("invalid configuration"),
                }
            }
        }
    }
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let (player1_input, player2_input) = input.split("\n\n").next_tuple().value()?;

    let player1_cards: VecDeque<u8> = player1_input
        .lines()
        .skip(1)
        .map(|line| line.parse())
        .try_collect()?;

    let player2_cards: VecDeque<u8> = player2_input
        .lines()
        .skip(1)
        .map(|line| line.parse())
        .try_collect()?;

    let result1 = play_normal_game(Game::new(player1_cards.clone(), player2_cards.clone()))?;
    let result2 = play_recursive_game(Game::new(player1_cards, player2_cards))?;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
