use aoc::*;

use eyre::bail;
use itertools::Itertools;
use regex::Regex;

use std::cmp::Reverse;

#[derive(Clone)]
struct Group<'a> {
    unit_count: i64,
    unit_hp: i64,
    weaknesses: Vec<&'a str>,
    immunities: Vec<&'a str>,
    attack_damage: i64,
    attack_type: &'a str,
    initiative: i64,
    effective_power: i64,
}

#[derive(Clone)]
enum GroupId {
    ImmuneSystem(usize),
    Infection(usize),
}

#[derive(Clone)]
struct Battle<'a> {
    immune_system: Vec<Option<Group<'a>>>,
    infection: Vec<Option<Group<'a>>>,
    target_selection_order: Vec<GroupId>,
    attack_order: Vec<GroupId>,
}

struct Buffer {
    available_immune_system_ids: Vec<usize>,
    available_infection_ids: Vec<usize>,
    immune_system_attacks: Vec<Option<(usize, i64)>>,
    infection_attacks: Vec<Option<(usize, i64)>>,
}

fn compute_target_selection(
    army: &[Option<Group>],
    enemy_army: &[Option<Group>],
    group_index: usize,
    available_ids: &mut Vec<usize>,
    attacks: &mut [Option<(usize, i64)>],
) {
    if let Some(group) = army[group_index].as_ref() {
        let target = available_ids
            .iter()
            .enumerate()
            .flat_map(|(index, &enemy_index)| {
                enemy_army[enemy_index]
                    .as_ref()
                    .map(|enemy_group| (index, enemy_index, enemy_group))
            })
            .flat_map(|(index, enemy_index, enemy_group)| {
                if enemy_group.immunities.contains(&group.attack_type) {
                    return None;
                }

                let attack_factor = if enemy_group.weaknesses.contains(&group.attack_type) {
                    2
                } else {
                    1
                };

                Some((
                    (index, enemy_index, attack_factor),
                    (
                        attack_factor,
                        enemy_group.effective_power,
                        enemy_group.initiative,
                    ),
                ))
            })
            .max_by(|(_, x), (_, y)| x.cmp(y))
            .map(|(x, _)| x);

        if let Some((index, enemy_index, attack_factor)) = target {
            available_ids.swap_remove(index);
            attacks[group_index] = Some((enemy_index, attack_factor));
        }
    }
}

fn attack(
    army: &[Option<Group>],
    enemy_army: &mut [Option<Group>],
    group_index: usize,
    attacks: &[Option<(usize, i64)>],
    boost: i64,
    locked: &mut bool,
) {
    if let (Some(group), Some((enemy_index, attack_factor))) =
        (army[group_index].as_ref(), attacks[group_index])
        && let Some(ref mut enemy_group) = enemy_army[enemy_index]
    {
        let casualties =
            attack_factor * group.unit_count * (group.attack_damage + boost) / enemy_group.unit_hp;

        enemy_group.unit_count -= casualties;

        if casualties > 0 {
            *locked = false;
        }

        if enemy_group.unit_count < 0 {
            enemy_army[enemy_index] = None;
        }
    }
}

fn run(mut battle: Battle, buffer: &mut Buffer, boost: i64) -> std::result::Result<i64, i64> {
    battle
        .immune_system
        .iter_mut()
        .flatten()
        .for_each(|group| group.effective_power = group.unit_count * (group.attack_damage + boost));

    loop {
        buffer.available_immune_system_ids.clear();
        (buffer.available_immune_system_ids).extend(0..battle.immune_system.len());
        buffer.available_infection_ids.clear();
        (buffer.available_infection_ids).extend(0..battle.infection.len());
        buffer.immune_system_attacks.fill(None);
        buffer.infection_attacks.fill(None);

        (battle.target_selection_order).sort_unstable_by_key(|group_id| {
            let group = match *group_id {
                GroupId::ImmuneSystem(index) => &battle.immune_system[index],
                GroupId::Infection(index) => &battle.infection[index],
            };
            (group.as_ref()).map(|x| (Reverse(x.effective_power), Reverse(x.initiative)))
        });

        for group_id in &battle.target_selection_order {
            match *group_id {
                GroupId::ImmuneSystem(group_index) => {
                    compute_target_selection(
                        &battle.immune_system,
                        &battle.infection,
                        group_index,
                        &mut buffer.available_immune_system_ids,
                        &mut buffer.immune_system_attacks,
                    );
                }
                GroupId::Infection(group_index) => {
                    compute_target_selection(
                        &battle.infection,
                        &battle.immune_system,
                        group_index,
                        &mut buffer.available_infection_ids,
                        &mut buffer.infection_attacks,
                    );
                }
            };
        }

        let mut locked = true;

        for group_id in &battle.attack_order {
            match *group_id {
                GroupId::ImmuneSystem(group_index) => {
                    attack(
                        &battle.immune_system,
                        &mut battle.infection,
                        group_index,
                        &buffer.immune_system_attacks,
                        boost,
                        &mut locked,
                    );
                }
                GroupId::Infection(group_index) => {
                    attack(
                        &battle.infection,
                        &mut battle.immune_system,
                        group_index,
                        &buffer.infection_attacks,
                        0,
                        &mut locked,
                    );
                }
            }
        }

        if locked {
            break Err(0);
        }

        let iter =
            battle.immune_system.iter_mut().flatten().map(|group| {
                group.effective_power = group.unit_count * (group.attack_damage + boost)
            });

        if iter.count() == 0 {
            break Err(battle
                .infection
                .iter_mut()
                .flatten()
                .map(|group| group.unit_count)
                .sum::<i64>());
        }

        let iter = battle
            .infection
            .iter_mut()
            .flatten()
            .map(|group| group.effective_power = group.unit_count * group.attack_damage);

        if iter.count() == 0 {
            break Ok(battle
                .immune_system
                .iter_mut()
                .flatten()
                .map(|group| group.unit_count)
                .sum::<i64>());
        }
    }
}

fn parse_initial_battle(input: &str) -> Result<Battle<'_>> {
    let regex_armies = Regex::new(r#"(?ms)^Immune System:$(.*)^Infection:$(.*)$"#)?;
    let regex_units = Regex::new(
        r#"(?m)^(\d+) units each with (\d+) hit points (\(.+?\) )?with an attack that does (\d+) (\w+) damage at initiative (\d+)$"#,
    )?;
    let regex_defenses = Regex::new(r#"(weak|immune) to ([a-z, ]+)"#)?;

    let parse_army = |army| -> Result<Vec<_>> {
        regex_units
            .captures_iter(army)
            .map(|cap_unit| {
                let mut weaknesses = Vec::new();
                let mut immunities = Vec::new();

                if let Some(defenses) = cap_unit.get(3) {
                    for cap_defense in regex_defenses.captures_iter(defenses.as_str()) {
                        let iter = cap_defense.get(2).value()?.as_str().split(", ");

                        match &cap_defense[1] {
                            "weak" => weaknesses.extend(iter),
                            "immune" => immunities.extend(iter),
                            other => bail!("unknown defense type: {other}"),
                        }
                    }
                }

                let unit_count = cap_unit[1].parse()?;
                let unit_hp = cap_unit[2].parse()?;
                let attack_damage = cap_unit[4].parse()?;
                let attack_type = cap_unit.get(5).value()?.as_str();
                let initiative = cap_unit[6].parse()?;
                let effective_power = unit_count * attack_damage;

                Ok(Some(Group {
                    unit_count,
                    unit_hp,
                    weaknesses,
                    immunities,
                    attack_damage,
                    attack_type,
                    initiative,
                    effective_power,
                }))
            })
            .try_collect()
    };

    let cap_army = regex_armies.captures(input).value()?;

    let immune_system = parse_army(cap_army.get(1).value()?.as_str())?;
    let infection = parse_army(cap_army.get(2).value()?.as_str())?;

    let target_selection_order = (0..immune_system.len())
        .map(GroupId::ImmuneSystem)
        .chain((0..infection.len()).map(GroupId::Infection))
        .collect_vec();

    let attack_order = target_selection_order.clone();

    let mut initial_battle = Battle {
        immune_system,
        infection,
        target_selection_order,
        attack_order,
    };

    (initial_battle.attack_order).sort_unstable_by_key(|group_id| {
        let group = match *group_id {
            GroupId::ImmuneSystem(index) => &initial_battle.immune_system[index],
            GroupId::Infection(index) => &initial_battle.infection[index],
        };
        group.as_ref().map(|x| Reverse(x.initiative))
    });

    Ok(initial_battle)
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let initial_battle = parse_initial_battle(&input)?;

    let mut buffer = Buffer {
        available_immune_system_ids: (0..initial_battle.immune_system.len()).collect(),
        available_infection_ids: (0..initial_battle.infection.len()).collect(),
        immune_system_attacks: vec![None; initial_battle.immune_system.len()],
        infection_attacks: vec![None; initial_battle.infection.len()],
    };

    let mut iter = (0..).map(|boost| run(initial_battle.clone(), &mut buffer, boost));

    let (result1, result2) = match iter.next().value()? {
        Ok(result) => (result, result),
        Err(result1) => {
            let result2 = iter.find_map(|x| x.ok()).value()?;
            (result1, result2)
        }
    };

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
