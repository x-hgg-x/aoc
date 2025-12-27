use aoc::*;

use itertools::Itertools;
use smallvec::SmallVec;

use std::cmp::Reverse;

const JOKER: u8 = 11;

struct Hand {
    cards: [u8; 5],
    cards_with_joker: [u8; 5],
    kind: u8,
    kind_with_joker: u8,
    bid: u64,
}

fn compute_total_winnings(hands: &[Hand]) -> u64 {
    hands
        .iter()
        .enumerate()
        .map(|(idx, x)| (idx as u64 + 1) * x.bid)
        .sum()
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let mut hands: Vec<_> = input
        .lines()
        .map(|line| {
            let (cards, bid) = line.split_ascii_whitespace().next_tuple().value()?;

            let mut has_joker = false;

            let cards = cards
                .bytes()
                .filter_map(|x| match x {
                    b'0'..=b'9' => Some(x - b'0'),
                    b'T' => Some(10),
                    b'J' => {
                        has_joker = true;
                        Some(11)
                    }
                    b'Q' => Some(12),
                    b'K' => Some(13),
                    b'A' => Some(14),
                    _ => None,
                })
                .collect_array()
                .value()?;

            let cards_with_joker = cards
                .into_iter()
                .map(|x| if x == JOKER { 1 } else { x })
                .collect_array::<5>()
                .value()?;

            let bid = bid.parse()?;

            let mut sorted_cards = cards;
            sorted_cards.sort_unstable();

            let mut counts: SmallVec<[_; 5]> = sorted_cards
                .into_iter()
                .dedup_with_count()
                .map(|(count, card)| (count as u8, card))
                .collect();

            counts.sort_unstable_by_key(|&(count, _)| Reverse(count));

            let (kind, kind_with_joker) = match counts[..] {
                [(5, _), ..] => (6, 6),
                [(4, _), ..] => {
                    if has_joker {
                        (5, 6)
                    } else {
                        (5, 5)
                    }
                }
                [(3, _), (2, _), ..] => {
                    if has_joker {
                        (4, 6)
                    } else {
                        (4, 4)
                    }
                }
                [(3, _), (1, _), ..] => {
                    if has_joker {
                        (3, 5)
                    } else {
                        (3, 3)
                    }
                }
                [(2, card1), (2, card2), ..] => {
                    if card1 == JOKER || card2 == JOKER {
                        (2, 5)
                    } else if has_joker {
                        (2, 4)
                    } else {
                        (2, 2)
                    }
                }
                [(2, _), ..] => {
                    if has_joker {
                        (1, 3)
                    } else {
                        (1, 1)
                    }
                }
                _ => {
                    if has_joker {
                        (0, 1)
                    } else {
                        (0, 0)
                    }
                }
            };

            Result::Ok(Hand {
                cards,
                cards_with_joker,
                kind,
                kind_with_joker,
                bid,
            })
        })
        .try_collect()?;

    hands.sort_unstable_by_key(|x| (x.kind, x.cards));
    let result1 = compute_total_winnings(&hands);

    hands.sort_unstable_by_key(|x| (x.kind_with_joker, x.cards_with_joker));
    let result2 = compute_total_winnings(&hands);

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
