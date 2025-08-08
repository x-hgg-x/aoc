use aoc::*;

use itertools::{Itertools, iproduct};
use regex::Regex;
use smallvec::{SmallVec, smallvec};

use std::iter::{Sum, once};

const HP: i64 = 100;

#[derive(Default)]
struct Equipment {
    cost: i64,
    damage: i64,
    armor: i64,
}

impl Equipment {
    fn new(cost: i64, damage: i64, armor: i64) -> Self {
        Self { cost, damage, armor }
    }
}

impl<'a> Sum<&'a Self> for Equipment {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::default(), |x1, x2| Self { cost: x1.cost + x2.cost, damage: x1.damage + x2.damage, armor: x1.armor + x2.armor })
    }
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let re = Regex::new(r#"Hit Points: (\d+)\s+Damage: (\d+)\s+Armor: (\d+)"#)?;

    let cap = re.captures(&input).value()?;
    let boss_hp: i64 = cap[1].parse()?;
    let boss_damage: i64 = cap[2].parse()?;
    let boss_armor: i64 = cap[3].parse()?;

    let weapons = [Equipment::new(8, 4, 0), Equipment::new(10, 5, 0), Equipment::new(25, 6, 0), Equipment::new(40, 7, 0), Equipment::new(74, 8, 0)];

    let armors = [
        Equipment::new(0, 0, 0),
        Equipment::new(13, 0, 1),
        Equipment::new(31, 0, 2),
        Equipment::new(53, 0, 3),
        Equipment::new(75, 0, 4),
        Equipment::new(102, 0, 5),
    ];

    let rings = [
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
            let rings: Equipment = rings.iter().copied().sum();

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

    let (result1, result2) = battles.iter().fold(
        (i64::MAX, i64::MIN),
        |(min_cost, max_cost), &(cost, win)| {
            if win { (cost.min(min_cost), max_cost) } else { (min_cost, cost.max(max_cost)) }
        },
    );

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
