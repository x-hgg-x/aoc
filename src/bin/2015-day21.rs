use itertools::{iproduct, Itertools};
use regex::Regex;
use smallvec::{smallvec, SmallVec};

use std::fs;
use std::iter::{once, Sum};

#[derive(Default)]
struct Equipment {
    cost: i32,
    damage: i32,
    armor: i32,
}

impl Equipment {
    fn new(cost: i32, damage: i32, armor: i32) -> Self {
        Self { cost, damage, armor }
    }
}

impl<'a> Sum<&'a Self> for Equipment {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::default(), |x1, x2| Self { cost: x1.cost + x2.cost, damage: x1.damage + x2.damage, armor: x1.armor + x2.armor })
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("inputs/2015-day21.txt")?;

    let re = Regex::new(r#"Hit Points: (\d+)\s+Damage: (\d+)\s+Armor: (\d+)"#).unwrap();

    let (boss_hp, boss_damage, boss_armor): (i32, i32, i32) =
        re.captures(&input).map(|cap| (cap[1].parse().unwrap(), cap[2].parse().unwrap(), cap[3].parse().unwrap())).unwrap();

    const HP: i32 = 100;

    let weapons = vec![Equipment::new(8, 4, 0), Equipment::new(10, 5, 0), Equipment::new(25, 6, 0), Equipment::new(40, 7, 0), Equipment::new(74, 8, 0)];

    let armors = vec![
        Equipment::new(0, 0, 0),
        Equipment::new(13, 0, 1),
        Equipment::new(31, 0, 2),
        Equipment::new(53, 0, 3),
        Equipment::new(75, 0, 4),
        Equipment::new(102, 0, 5),
    ];

    let rings = vec![
        Equipment::new(25, 1, 0),
        Equipment::new(50, 2, 0),
        Equipment::new(100, 3, 0),
        Equipment::new(20, 0, 1),
        Equipment::new(40, 0, 2),
        Equipment::new(80, 0, 3),
    ];

    let rings_combinations = once(SmallVec::<[_; 2]>::new())
        .chain(rings.iter().tuple_combinations().map(|(x,)| smallvec![x]))
        .chain(rings.iter().tuple_combinations().map(|(x, y)| smallvec![x, y]));

    let battles = iproduct!(&weapons, &armors, rings_combinations)
        .map(|(weapon, armor, rings)| {
            let rings: Equipment = rings.iter().cloned().sum();

            let cost = weapon.cost + armor.cost + rings.cost;
            let damage = weapon.damage + armor.damage + rings.damage;
            let armor = weapon.armor + armor.armor + rings.armor;

            let player_damage_by_turn = (damage - boss_armor).max(1);
            let player_turns = 1 + (boss_hp - 1) / player_damage_by_turn;

            let boss_damage_by_turn = (boss_damage - armor).max(1);
            let boss_turns = 1 + (HP - 1) / boss_damage_by_turn;

            let win = player_turns <= boss_turns;

            (cost, win)
        })
        .collect_vec();

    let (result1, result2) = battles
        .iter()
        .scan((i32::MAX, i32::MIN), |state, &(cost, win)| {
            if win {
                state.0 = state.0.min(cost);
            } else {
                state.1 = state.1.max(cost);
            }
            Some(*state)
        })
        .last()
        .unwrap();

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
