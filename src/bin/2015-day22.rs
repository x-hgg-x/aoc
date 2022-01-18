use aoc::*;

use regex::Regex;

use std::cmp::Ordering;
use std::collections::BinaryHeap;

trait ISpell {
    fn mana() -> i64;
    fn max_timer() -> i64;
    fn current_timer(&self) -> i64;
    fn current_timer_mut(&mut self) -> &mut i64;
    fn get(spells: &Spells) -> &Self;
    fn get_mut(spells: &mut Spells) -> &mut Self;

    fn is_castable(&self, status: &Status) -> bool {
        status.player_mana >= Self::mana() && self.current_timer() <= 0
    }

    fn cast(&mut self, status: &mut Status) {
        *self.current_timer_mut() = Self::max_timer();
        status.mana_spent += Self::mana();
        status.player_mana -= Self::mana();
    }
}

macro_rules! new_spell {
    ($type_name:ident, $field_name:ident, $mana:expr, $max_timer:expr) => {
        #[derive(Default, Clone)]
        struct $type_name {
            timer: i64,
        }

        impl ISpell for $type_name {
            fn mana() -> i64 {
                $mana
            }
            fn max_timer() -> i64 {
                $max_timer
            }
            fn current_timer(&self) -> i64 {
                self.timer
            }
            fn current_timer_mut(&mut self) -> &mut i64 {
                &mut self.timer
            }
            fn get(spells: &Spells) -> &Self {
                &spells.$field_name
            }
            fn get_mut(spells: &mut Spells) -> &mut Self {
                &mut spells.$field_name
            }
        }
    };
}

new_spell!(MagicMissile, magic_missile, 53, 1);
new_spell!(Drain, drain, 73, 1);
new_spell!(Shield, shield, 113, 6);
new_spell!(Poison, poison, 173, 6);
new_spell!(Recharge, recharge, 229, 5);

impl MagicMissile {
    fn apply_effect(&mut self, status: &mut Status) {
        if self.timer > 0 {
            self.timer -= 1;
            status.boss_hp -= 4;
        }
    }
}

impl Drain {
    fn apply_effect(&mut self, status: &mut Status) {
        if self.timer > 0 {
            self.timer -= 1;
            status.player_hp += 2;
            status.boss_hp -= 2;
        }
    }
}

impl Shield {
    fn apply_effect(&mut self, status: &mut Status) {
        if self.timer > 0 {
            self.timer -= 1;
            status.player_armor = 7;
        } else {
            status.player_armor = 0;
        }
    }
}

impl Poison {
    fn apply_effect(&mut self, status: &mut Status) {
        if self.timer > 0 {
            self.timer -= 1;
            status.boss_hp -= 3;
        }
    }
}

impl Recharge {
    fn apply_effect(&mut self, status: &mut Status) {
        if self.timer > 0 {
            self.timer -= 1;
            status.player_mana += 101;
        }
    }
}

#[derive(Default, Clone)]
struct Spells {
    magic_missile: MagicMissile,
    drain: Drain,
    shield: Shield,
    poison: Poison,
    recharge: Recharge,
}

impl Spells {
    fn apply_effects(&mut self, status: &mut Status) {
        self.magic_missile.apply_effect(status);
        self.drain.apply_effect(status);
        self.shield.apply_effect(status);
        self.poison.apply_effect(status);
        self.recharge.apply_effect(status);
    }
}

#[derive(Clone)]
struct Status {
    mana_spent: i64,
    player_hp: i64,
    player_armor: i64,
    player_mana: i64,
    boss_hp: i64,
    boss_damage: i64,
}

#[derive(Clone)]
struct GameState {
    hard_mode: bool,
    status: Status,
    spells: Spells,
}

impl GameState {
    fn new(hard_mode: bool, player_hp: i64, player_mana: i64, boss_hp: i64, boss_damage: i64) -> Self {
        Self { hard_mode, status: Status { player_hp, mana_spent: 0, player_armor: 0, player_mana, boss_hp, boss_damage }, spells: Default::default() }
    }

    fn try_cast<Spell: ISpell>(&self, spell: &Spell) -> Option<GameResult> {
        spell.is_castable(&self.status).then(|| {
            let mut next_state = self.clone();
            let (status, spells) = (&mut next_state.status, &mut next_state.spells);

            Spell::get_mut(spells).cast(status);

            // Boss turn
            spells.apply_effects(status);
            if status.boss_hp <= 0 {
                return GameResult::GameWon(status.mana_spent);
            }

            status.player_hp -= (status.boss_damage - status.player_armor).max(1);
            if status.player_hp <= 0 {
                return GameResult::GameLost;
            }

            // Player turn
            spells.apply_effects(status);
            if status.boss_hp <= 0 {
                return GameResult::GameWon(status.mana_spent);
            }

            if self.hard_mode {
                status.player_hp -= 1;
                if status.player_hp <= 0 {
                    return GameResult::GameLost;
                }
            }

            GameResult::Unknown(next_state)
        })
    }
}

impl Eq for GameState {}

impl PartialEq for GameState {
    fn eq(&self, other: &Self) -> bool {
        self.status.mana_spent.eq(&other.status.mana_spent)
    }
}

impl Ord for GameState {
    fn cmp(&self, other: &Self) -> Ordering {
        self.status.mana_spent.cmp(&other.status.mana_spent)
    }
}

impl PartialOrd for GameState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

enum GameResult {
    GameWon(i64),
    GameLost,
    Unknown(GameState),
}

fn process_result(heap: &mut BinaryHeap<GameState>, min_mana: &mut i64, current_state: &GameState, spell: &impl ISpell) {
    if let Some(game_result) = current_state.try_cast(spell) {
        match game_result {
            GameResult::GameWon(mana_spent) => {
                *min_mana = mana_spent.min(*min_mana);

                while let Some(game_state) = heap.peek() {
                    if game_state.status.mana_spent < *min_mana {
                        break;
                    }
                    heap.pop();
                }
            }
            GameResult::Unknown(game_state) if game_state.status.mana_spent < *min_mana => {
                heap.push(game_state);
            }
            _ => (),
        }
    }
}

fn solve(hard_mode: bool, boss_hp: i64, boss_damage: i64) -> i64 {
    let mut heap = BinaryHeap::from([GameState::new(hard_mode, 50, 500, boss_hp, boss_damage)]);

    let mut min_mana = i64::MAX;
    while let Some(state) = heap.pop() {
        let spells = &state.spells;
        process_result(&mut heap, &mut min_mana, &state, &spells.magic_missile);
        process_result(&mut heap, &mut min_mana, &state, &spells.drain);
        process_result(&mut heap, &mut min_mana, &state, &spells.shield);
        process_result(&mut heap, &mut min_mana, &state, &spells.poison);
        process_result(&mut heap, &mut min_mana, &state, &spells.recharge);
    }
    min_mana
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let re = Regex::new(r#"Hit Points: (\d+)\s+Damage: (\d+)"#)?;

    let cap = re.captures(&input).value()?;
    let boss_hp: i64 = cap[1].parse()?;
    let boss_damage: i64 = cap[2].parse()?;

    let result1 = solve(false, boss_hp, boss_damage);
    let result2 = solve(true, boss_hp, boss_damage);

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
