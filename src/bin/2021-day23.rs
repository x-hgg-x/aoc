use aoc::*;

use smallvec::SmallVec;

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::hash_map::{Entry, HashMap};
use std::iter;

const ROOM_INDICES: [u8; 4] = [2, 4, 6, 8];
const EMPTY: u8 = u8::MAX;

#[derive(Clone, Eq, PartialEq, Hash)]
struct Burrow {
    hallway: [u8; 11],
    rooms: [SmallVec<[u8; 4]>; 4],
}

impl Burrow {
    fn get_available_room_position(&self, id: u8) -> Option<usize> {
        let room = self.rooms[id as usize].as_slice();

        match room.iter().position(|&x| x != EMPTY) {
            None => Some(room.len() - 1),
            Some(position) if position > 0 && room[position..].iter().all(|&x| x == id) => {
                Some(position - 1)
            }
            _ => None,
        }
    }
}

#[derive(Copy, Clone)]
enum Position {
    Room(u8, u8),
    Hallway(u8),
}

#[derive(Copy, Clone)]
struct Amphipod {
    id: u8,
    position: Position,
}

impl Amphipod {
    fn energy(&self) -> u64 {
        10u64.pow(self.id as u32)
    }
}

#[derive(Clone)]
struct State {
    burrow: Burrow,
    amphipods: SmallVec<[Amphipod; 16]>,
    energy: u64,
    energy_needed: u64,
}

impl State {
    fn new(burrow: Burrow, amphipods: SmallVec<[Amphipod; 16]>, energy: u64) -> Self {
        let mut state = Self {
            burrow,
            amphipods,
            energy,
            energy_needed: 0,
        };

        state.energy_needed = state.energy_needed();
        state
    }

    fn energy_needed(&self) -> u64 {
        self.amphipods
            .iter()
            .map(|amphipod| match amphipod.position {
                Position::Room(room_index, _) if amphipod.id == room_index => 0,
                Position::Room(room_index, room_position) => {
                    let hallway_index = ROOM_INDICES[room_index as usize];

                    let steps = hallway_index.abs_diff(ROOM_INDICES[amphipod.id as usize])
                        + (room_position + 2);

                    steps as u64 * amphipod.energy()
                }
                Position::Hallway(hallway_index) => {
                    let steps = hallway_index.abs_diff(ROOM_INDICES[amphipod.id as usize]) + 1;
                    steps as u64 * amphipod.energy()
                }
            })
            .sum()
    }

    fn estimate(&self) -> u64 {
        self.energy + self.energy_needed
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

enum Tile {
    Empty,
    Amphipod(u8),
}

impl Tile {
    fn parse(x: u8) -> Option<Self> {
        match x {
            b'.' => Some(Tile::Empty),
            b'A'..=b'D' => Some(Tile::Amphipod(x - b'A')),
            _ => None,
        }
    }
}

fn parse_input(input: &str) -> Result<State> {
    let mut lines = input.lines();

    let mut amphipods = SmallVec::new();
    let mut hallway = [0; 11];
    let mut rooms = <[SmallVec<_>; 4]>::default();

    for ((index, hallway_elem), tile) in iter::zip(
        hallway.iter_mut().enumerate(),
        lines.by_ref().nth(1).value()?.bytes().flat_map(Tile::parse),
    ) {
        match tile {
            Tile::Empty => *hallway_elem = EMPTY,
            Tile::Amphipod(id) => {
                *hallway_elem = id;

                amphipods.push(Amphipod {
                    id,
                    position: Position::Hallway(index as u8),
                });
            }
        }
    }

    for (room_position, room_line) in lines.take(2).enumerate() {
        for ((room_index, room), tile) in iter::zip(
            rooms.iter_mut().enumerate(),
            room_line.bytes().flat_map(Tile::parse),
        ) {
            match tile {
                Tile::Empty => room.push(EMPTY),
                Tile::Amphipod(id) => {
                    room.push(id);

                    amphipods.push(Amphipod {
                        id,
                        position: Position::Room(room_index as u8, room_position as u8),
                    });
                }
            }
        }
    }

    Ok(State::new(Burrow { hallway, rooms }, amphipods, 0))
}

fn add_amphipods(initial_state: &mut State) {
    for amphipod in &mut initial_state.amphipods {
        if let Position::Room(_, room_position @ 1) = &mut amphipod.position {
            *room_position = 3;
        }
    }

    initial_state.amphipods.extend_from_slice(&[
        Amphipod {
            id: 3,
            position: Position::Room(0, 1),
        },
        Amphipod {
            id: 3,
            position: Position::Room(0, 2),
        },
        Amphipod {
            id: 2,
            position: Position::Room(1, 1),
        },
        Amphipod {
            id: 1,
            position: Position::Room(1, 2),
        },
        Amphipod {
            id: 1,
            position: Position::Room(2, 1),
        },
        Amphipod {
            id: 0,
            position: Position::Room(2, 2),
        },
        Amphipod {
            id: 0,
            position: Position::Room(3, 1),
        },
        Amphipod {
            id: 2,
            position: Position::Room(3, 2),
        },
    ]);

    initial_state.burrow.rooms[0].insert_from_slice(1, &[3, 3]);
    initial_state.burrow.rooms[1].insert_from_slice(1, &[2, 1]);
    initial_state.burrow.rooms[2].insert_from_slice(1, &[1, 0]);
    initial_state.burrow.rooms[3].insert_from_slice(1, &[0, 2]);
}

fn solve(initial_state: State) -> u64 {
    let mut previous_states = HashMap::new();
    let mut current_states = BinaryHeap::from([initial_state]);

    loop {
        if let Some(state) = current_states.pop() {
            if state.energy_needed() == 0 {
                break state.energy;
            }

            match previous_states.entry(state.burrow.clone()) {
                Entry::Occupied(mut entry) => {
                    let energy = entry.get_mut();
                    if *energy <= state.energy {
                        continue;
                    }
                    *energy = state.energy;
                }
                Entry::Vacant(entry) => {
                    entry.insert(state.energy);
                }
            }

            for (amphipod_index, amphipod) in state.amphipods.iter().enumerate() {
                match amphipod.position {
                    Position::Room(room_index, room_position) => {
                        let room_index = room_index as usize;
                        let room_position = room_position as usize;

                        if state.burrow.rooms[room_index][..room_position]
                            .iter()
                            .any(|&x| x != EMPTY)
                        {
                            continue;
                        }

                        let hallway_index = ROOM_INDICES[room_index] as usize;

                        let iter_left = state
                            .burrow
                            .hallway
                            .iter()
                            .enumerate()
                            .take(hallway_index)
                            .rev()
                            .take_while(|&(_, &value)| value == EMPTY);

                        let iter_right = state
                            .burrow
                            .hallway
                            .iter()
                            .enumerate()
                            .skip(hallway_index + 1)
                            .take_while(|&(_, &value)| value == EMPTY);

                        current_states.extend(
                            iter::chain(iter_left, iter_right)
                                .filter(|&(index, _)| !ROOM_INDICES.contains(&(index as u8)))
                                .map(|(index, _)| {
                                    let mut burrow = state.burrow.clone();

                                    burrow.rooms[room_index][room_position] = EMPTY;

                                    burrow.hallway[index] = amphipod.id;

                                    let mut amphipods = state.amphipods.clone();
                                    amphipods[amphipod_index].position =
                                        Position::Hallway(index as u8);

                                    let steps = hallway_index.abs_diff(index) + 1 + room_position;
                                    let energy = state.energy + steps as u64 * amphipod.energy();

                                    State::new(burrow, amphipods, energy)
                                }),
                        );
                    }
                    Position::Hallway(hallway_index) => {
                        if let Some(room_position) =
                            state.burrow.get_available_room_position(amphipod.id)
                        {
                            let room_hallway_index = ROOM_INDICES[amphipod.id as usize];

                            let range = if hallway_index < room_hallway_index {
                                hallway_index as usize + 1..room_hallway_index as usize
                            } else {
                                room_hallway_index as usize + 1..hallway_index as usize
                            };

                            if state.burrow.hallway[range.clone()]
                                .iter()
                                .all(|&value| value == EMPTY)
                            {
                                let mut burrow = state.burrow.clone();
                                burrow.rooms[amphipod.id as usize][room_position] = amphipod.id;
                                burrow.hallway[hallway_index as usize] = EMPTY;

                                let mut amphipods = state.amphipods.clone();

                                amphipods[amphipod_index].position =
                                    Position::Room(amphipod.id, room_position as u8);

                                let steps = range.len() + 2 + room_position;
                                let energy = state.energy + steps as u64 * amphipod.energy();

                                current_states.push(State::new(burrow, amphipods, energy));
                            }
                        }
                    }
                }
            }
        }
    }
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let mut initial_state = parse_input(&input)?;
    let result1 = solve(initial_state.clone());

    add_amphipods(&mut initial_state);
    let result2 = solve(initial_state);

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
