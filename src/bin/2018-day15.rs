use aoc::*;

use eyre::{bail, ensure};
use smallvec::SmallVec;

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::ops::ControlFlow;

const MAX_HP: i64 = 200;
const BASE_ATTACK_POWER: i64 = 3;

type Position = (usize, usize);

#[derive(Clone)]
enum Tile {
    Empty,
    Wall,
    Elf(usize),
    Goblin(usize),
}

impl Tile {
    fn is_empty(&self) -> bool {
        matches!(self, Tile::Empty)
    }
}

#[derive(Clone)]
struct Grid {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
}

impl Grid {
    fn new(width: usize, height: usize, tiles: Vec<Tile>) -> Result<Self> {
        ensure!(
            width * height == tiles.len(),
            "unable to construct Grid: width * height != tiles.len()"
        );

        Ok(Self {
            width,
            height,
            tiles,
        })
    }

    fn get_index(&self, row: usize, column: usize) -> usize {
        row * self.width + column
    }

    fn get_position(&self, index: usize) -> Position {
        let row = index / self.width;
        let column = index % self.width;
        (row, column)
    }

    fn adjacent_tile_indices(&self, row: usize, column: usize) -> SmallVec<[usize; 4]> {
        [
            (row > 0).then(|| self.get_index(row - 1, column)),
            (column > 0).then(|| self.get_index(row, column - 1)),
            (column < self.width - 1).then(|| self.get_index(row, column + 1)),
            (row < self.height - 1).then(|| self.get_index(row + 1, column)),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

#[derive(Clone)]
enum FighterId {
    Elf(usize),
    Goblin(usize),
}

trait ICreature {
    type EnemyType: ICreature;

    fn tile_index(&self) -> usize;
    fn tile_index_mut(&mut self) -> &mut usize;
    fn hp(&self) -> i64;
    fn hp_mut(&mut self) -> &mut i64;
    fn enemy_id(tile: &Tile) -> Option<usize>;
}

macro_rules! new_creature {
    ($type_name:ident, $attack_power_field:ident, $enemy_tile_path:path, $enemy_name:ident) => {
        #[derive(Clone)]
        struct $type_name {
            tile_index: usize,
            hp: i64,
        }

        impl ICreature for $type_name {
            type EnemyType = $enemy_name;

            fn tile_index(&self) -> usize {
                self.tile_index
            }
            fn tile_index_mut(&mut self) -> &mut usize {
                &mut self.tile_index
            }
            fn hp(&self) -> i64 {
                self.hp
            }
            fn hp_mut(&mut self) -> &mut i64 {
                &mut self.hp
            }
            fn enemy_id(tile: &Tile) -> Option<usize> {
                match *tile {
                    $enemy_tile_path(id) => Some(id),
                    _ => None,
                }
            }
        }
    };
}

new_creature!(Elf, elf_attack_power, Tile::Goblin, Goblin);
new_creature!(Goblin, goblin_attack_power, Tile::Elf, Elf);

#[derive(Clone)]
struct Battle {
    elf_attack_power: i64,
    goblin_attack_power: i64,
    grid: Grid,
    fighter_ids: Vec<FighterId>,
    elfs: Vec<Option<Elf>>,
    goblins: Vec<Option<Goblin>>,
}

struct State {
    position: Position,
    steps: usize,
    distance: usize,
    goal_position: Position,
    start_tile_index: usize,
}

impl State {
    fn new(
        position: Position,
        steps: usize,
        (goal_row, goal_column): Position,
        start_tile_index: usize,
    ) -> Self {
        let (row, column) = position;
        let distance = row.abs_diff(goal_row) + column.abs_diff(goal_column);

        Self {
            position,
            steps,
            distance,
            goal_position: (goal_row, goal_column),
            start_tile_index,
        }
    }

    fn estimate(&self) -> (usize, Position, usize) {
        (
            self.steps + self.distance,
            self.goal_position,
            self.start_tile_index,
        )
    }
}

impl Eq for State {}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.estimate().eq(&other.estimate())
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.estimate().cmp(&self.estimate())
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Default)]
struct Buffer {
    current_states: BinaryHeap<State>,
    previous_positions: HashSet<(Position, Position, usize)>,
}

fn movement<Creature: ICreature>(
    adjacent_tile_indices: &[usize],
    enemies: &mut [Option<Creature::EnemyType>],
    grid: &mut Grid,
    buffer: &mut Buffer,
) -> Option<(usize, bool)> {
    buffer.current_states.clear();
    buffer.current_states.extend(
        enemies
            .iter()
            .flatten()
            .flat_map(|enemy| {
                let grid = &*grid;
                let (enemy_row, enemy_column) = grid.get_position(enemy.tile_index());

                grid.adjacent_tile_indices(enemy_row, enemy_column)
                    .into_iter()
                    .filter(|&adjacent_enemy_tile_index| {
                        grid.tiles[adjacent_enemy_tile_index].is_empty()
                    })
                    .map(move |adjacent_enemy_tile_index| {
                        let (adjacent_enemy_row, adjacent_enemy_column) =
                            grid.get_position(adjacent_enemy_tile_index);

                        adjacent_tile_indices
                            .iter()
                            .filter(|&&start_tile_index| grid.tiles[start_tile_index].is_empty())
                            .map(move |&start_tile_index| {
                                State::new(
                                    grid.get_position(start_tile_index),
                                    1,
                                    (adjacent_enemy_row, adjacent_enemy_column),
                                    start_tile_index,
                                )
                            })
                    })
            })
            .flatten(),
    );

    buffer.previous_positions.clear();

    buffer.previous_positions.extend(
        buffer
            .current_states
            .iter()
            .map(|state| (state.position, state.goal_position, state.start_tile_index)),
    );

    loop {
        match buffer.current_states.pop() {
            None => break None,
            Some(state) => {
                if state.position == state.goal_position {
                    let in_range = grid.get_position(state.start_tile_index) == state.goal_position;
                    break Some((state.start_tile_index, in_range));
                }

                let mut process_neighbors = |new_row, new_column| {
                    let new_position = (new_row, new_column);
                    let new_steps = state.steps + 1;
                    let new_key = (new_position, state.goal_position, state.start_tile_index);

                    if grid.tiles[grid.get_index(new_row, new_column)].is_empty()
                        && buffer.previous_positions.insert(new_key)
                    {
                        buffer.current_states.push(State::new(
                            new_position,
                            new_steps,
                            state.goal_position,
                            state.start_tile_index,
                        ));
                    }
                };

                let (state_row, state_column) = state.position;

                if state_row > 0 {
                    process_neighbors(state_row - 1, state_column);
                }
                if state_row < grid.height - 1 {
                    process_neighbors(state_row + 1, state_column);
                }
                if state_column > 0 {
                    process_neighbors(state_row, state_column - 1);
                }
                if state_column < grid.width - 1 {
                    process_neighbors(state_row, state_column + 1);
                }
            }
        }
    }
}

fn compute_attack_target_id<Creature: ICreature>(
    adjacent_tile_indices: &[usize],
    enemies: &[Option<Creature::EnemyType>],
    grid: &mut Grid,
) -> Option<usize> {
    adjacent_tile_indices
        .iter()
        .filter_map(|&tile_index| {
            Creature::enemy_id(&grid.tiles[tile_index])
                .and_then(|id| enemies[id].as_ref().map(|enemy| (id, enemy)))
        })
        .min_by_key(|&(_, enemy)| (enemy.hp(), enemy.tile_index()))
        .map(|(id, _)| id)
}

fn attack<Creature: ICreature>(
    enemy: &mut Option<<Creature as ICreature>::EnemyType>,
    attack_power: i64,
    grid: &mut Grid,
    casualties: &mut bool,
) -> Result<()> {
    let enemy_creature = enemy.as_mut().value()?;
    *enemy_creature.hp_mut() -= attack_power;

    if enemy_creature.hp() <= 0 {
        grid.tiles[enemy_creature.tile_index()] = Tile::Empty;
        *casualties = true;
        *enemy = None;
    }

    Ok(())
}

fn take_turn<Creature: ICreature>(
    creature: Option<&mut Creature>,
    attack_power: i64,
    enemies: &mut [Option<Creature::EnemyType>],
    grid: &mut Grid,
    casualties: &mut bool,
    buffer: &mut Buffer,
) -> Result<ControlFlow<()>> {
    let creature = match creature {
        Some(creature) => creature,
        None => return Ok(ControlFlow::Continue(())),
    };

    if enemies.iter().flatten().next().is_none() {
        return Ok(ControlFlow::Break(()));
    }

    let (row, column) = grid.get_position(creature.tile_index());
    let adjacent_tile_indices = grid.adjacent_tile_indices(row, column);

    match compute_attack_target_id::<Creature>(&adjacent_tile_indices, enemies, grid) {
        Some(enemy_id) => {
            attack::<Creature>(&mut enemies[enemy_id], attack_power, grid, casualties)?
        }
        None => {
            let (new_tile_index, in_range) =
                match movement::<Creature>(&adjacent_tile_indices, enemies, grid, buffer) {
                    Some(movement) => movement,
                    None => return Ok(ControlFlow::Continue(())),
                };

            let old_tile_index = creature.tile_index();
            *creature.tile_index_mut() = new_tile_index;
            grid.tiles.swap(old_tile_index, new_tile_index);

            if in_range {
                let (row, column) = grid.get_position(creature.tile_index());

                let enemy_id = compute_attack_target_id::<Creature>(
                    &grid.adjacent_tile_indices(row, column),
                    enemies,
                    grid,
                )
                .value()?;

                attack::<Creature>(&mut enemies[enemy_id], attack_power, grid, casualties)?;
            }
        }
    }

    Ok(ControlFlow::Continue(()))
}

fn run(battle: Battle, buffer: &mut Buffer) -> Result<(i64, bool)> {
    let Battle {
        elf_attack_power,
        goblin_attack_power,
        mut grid,
        mut fighter_ids,
        mut elfs,
        mut goblins,
    } = battle;

    let mut turns = 0;

    'run: loop {
        let mut casualties = false;

        for fighter_id in &fighter_ids {
            let action = match *fighter_id {
                FighterId::Elf(id) => take_turn(
                    elfs[id].as_mut(),
                    elf_attack_power,
                    &mut goblins,
                    &mut grid,
                    &mut casualties,
                    buffer,
                )?,
                FighterId::Goblin(id) => take_turn(
                    goblins[id].as_mut(),
                    goblin_attack_power,
                    &mut elfs,
                    &mut grid,
                    &mut casualties,
                    buffer,
                )?,
            };

            if action == ControlFlow::Break(()) {
                break 'run;
            }
        }

        if casualties {
            fighter_ids.retain(|fighter_id| match *fighter_id {
                FighterId::Elf(id) => elfs[id].is_some(),
                FighterId::Goblin(id) => goblins[id].is_some(),
            })
        }

        fighter_ids.sort_unstable_by_key(|fighter_id| match *fighter_id {
            FighterId::Elf(id) => elfs[id].as_ref().map(|x| x.tile_index),
            FighterId::Goblin(id) => goblins[id].as_ref().map(|x| x.tile_index),
        });

        turns += 1;
    }

    let hp_sum = elfs
        .iter()
        .flatten()
        .map(|x| x.hp)
        .chain(goblins.iter().flatten().map(|x| x.hp))
        .sum::<i64>();

    let elf_casualties = elfs.iter().any(|elf| elf.is_none());

    Ok((turns * hp_sum, elf_casualties))
}

fn parse_initial_battle(input: &str) -> Result<Battle> {
    let width = input.lines().map(|line| line.len()).max().value()?;
    let height = input.lines().count();

    let mut tiles = Vec::with_capacity(width * height);
    let mut fighter_ids = Vec::new();
    let mut elfs = Vec::new();
    let mut goblins = Vec::new();

    (input.lines().map(|line| line.bytes()).enumerate()).try_for_each(|(row_index, row)| {
        for (column_index, x) in row.enumerate() {
            match x {
                b'.' => tiles.push(Tile::Empty),
                b'#' => tiles.push(Tile::Wall),
                b'E' => {
                    tiles.push(Tile::Elf(elfs.len()));
                    fighter_ids.push(FighterId::Elf(elfs.len()));

                    elfs.push(Some(Elf {
                        tile_index: row_index * width + column_index,
                        hp: MAX_HP,
                    }));
                }
                b'G' => {
                    tiles.push(Tile::Goblin(goblins.len()));
                    fighter_ids.push(FighterId::Goblin(goblins.len()));

                    goblins.push(Some(Goblin {
                        tile_index: row_index * width + column_index,
                        hp: MAX_HP,
                    }));
                }
                _ => bail!("unknown tile"),
            };
        }

        Ok(())
    })?;

    let grid = Grid::new(width, height, tiles)?;

    Ok(Battle {
        elf_attack_power: BASE_ATTACK_POWER,
        goblin_attack_power: BASE_ATTACK_POWER,
        grid,
        fighter_ids,
        elfs,
        goblins,
    })
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let initial_battle = parse_initial_battle(&input)?;

    let mut buffer = Buffer::default();

    let (result1, elf_casualties) = run(initial_battle.clone(), &mut buffer)?;

    let result2 = match elf_casualties {
        false => result1,
        true => (BASE_ATTACK_POWER + 1..)
            .find_map(|elf_attack_power| {
                (|| {
                    let battle = Battle {
                        elf_attack_power,
                        ..initial_battle.clone()
                    };
                    let (outcome, elf_casualties) = run(battle, &mut buffer)?;
                    Result::Ok((!elf_casualties).then_some(outcome))
                })()
                .transpose()
            })
            .transpose()?
            .value()?,
    };

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
