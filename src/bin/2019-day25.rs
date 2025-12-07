use aoc::*;

use eyre::bail;
use itertools::Itertools;
use regex::Regex;
use smallvec::SmallVec;

use std::collections::{HashMap, VecDeque};
use std::iter;

enum State {
    NeedInput { outputs: Vec<i64> },
    Finished,
}

#[derive(Clone)]
struct Intcode {
    program: HashMap<usize, i64>,
    ip: usize,
    relative_base: i64,
    inputs: VecDeque<i64>,
    outputs: Vec<i64>,
}

impl Intcode {
    fn new(program: HashMap<usize, i64>, inputs: VecDeque<i64>) -> Self {
        Self {
            program,
            ip: 0,
            relative_base: 0,
            inputs,
            outputs: Vec::new(),
        }
    }

    fn get_input(&mut self, arg_position: usize, instruction: i64) -> Result<i64> {
        let arg = *self.program.entry(self.ip + arg_position).or_default();

        match instruction / 10i64.pow(1 + arg_position as u32) % 10 {
            0 => Ok(*self.program.entry(usize::try_from(arg)?).or_default()),
            1 => Ok(arg),
            2 => Ok(*self
                .program
                .entry(usize::try_from(self.relative_base + arg)?)
                .or_default()),
            other => bail!("unknown parameter mode: {other}"),
        }
    }

    fn get_register(&mut self, arg_position: usize, instruction: i64) -> Result<&mut i64> {
        let arg = *self.program.entry(self.ip + arg_position).or_default();

        match instruction / 10i64.pow(1 + arg_position as u32) % 10 {
            0 => Ok(self.program.entry(usize::try_from(arg)?).or_default()),
            2 => Ok(self
                .program
                .entry(usize::try_from(self.relative_base + arg)?)
                .or_default()),
            other => bail!("invalid parameter mode: {other}"),
        }
    }

    fn run(&mut self) -> Result<State> {
        loop {
            let instruction = *self.program.entry(self.ip).or_default();
            match instruction % 100 {
                1 => {
                    let arg1 = self.get_input(1, instruction)?;
                    let arg2 = self.get_input(2, instruction)?;
                    let arg3 = self.get_register(3, instruction)?;
                    *arg3 = arg1 + arg2;
                    self.ip += 4;
                }
                2 => {
                    let arg1 = self.get_input(1, instruction)?;
                    let arg2 = self.get_input(2, instruction)?;
                    let arg3 = self.get_register(3, instruction)?;
                    *arg3 = arg1 * arg2;
                    self.ip += 4;
                }
                3 => match self.inputs.pop_front() {
                    Some(input) => {
                        let arg1 = self.get_register(1, instruction)?;
                        *arg1 = input;
                        self.ip += 2;
                    }
                    None => {
                        let outputs = self.outputs.clone();
                        self.outputs.clear();
                        return Ok(State::NeedInput { outputs });
                    }
                },
                4 => {
                    let arg1 = self.get_input(1, instruction)?;
                    self.ip += 2;
                    self.outputs.push(arg1);
                }
                5 => {
                    let arg1 = self.get_input(1, instruction)?;
                    let arg2 = self.get_input(2, instruction)?;
                    if arg1 != 0 {
                        self.ip = arg2.try_into()?;
                    } else {
                        self.ip += 3;
                    }
                }
                6 => {
                    let arg1 = self.get_input(1, instruction)?;
                    let arg2 = self.get_input(2, instruction)?;
                    if arg1 == 0 {
                        self.ip = arg2.try_into()?;
                    } else {
                        self.ip += 3;
                    }
                }
                7 => {
                    let arg1 = self.get_input(1, instruction)?;
                    let arg2 = self.get_input(2, instruction)?;
                    let arg3 = self.get_register(3, instruction)?;
                    *arg3 = (arg1 < arg2).into();
                    self.ip += 4;
                }
                8 => {
                    let arg1 = self.get_input(1, instruction)?;
                    let arg2 = self.get_input(2, instruction)?;
                    let arg3 = self.get_register(3, instruction)?;
                    *arg3 = (arg1 == arg2).into();
                    self.ip += 4;
                }
                9 => {
                    let arg1 = self.get_input(1, instruction)?;
                    self.relative_base += arg1;
                    self.ip += 2;
                }
                99 => return Ok(State::Finished),
                other => bail!("unknown opcode: {other}"),
            }
        }
    }
}

const DIRECTION_INPUTS: [&[u8]; 4] = [b"north\n", b"south\n", b"west\n", b"east\n"];
const REVERSE_DIRECTION_INPUTS: [&[u8]; 4] = [b"south\n", b"north\n", b"east\n", b"west\n"];

const REVERSE_PATH_DIRECTIONS: [Direction; 4] = [
    Direction::South,
    Direction::North,
    Direction::East,
    Direction::West,
];

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq)]
enum Direction {
    North = 0,
    South = 1,
    West = 2,
    East = 3,
}

struct ExplorationState {
    items: Vec<String>,
    current_path: Vec<Direction>,
    checkpoint_path: Vec<Direction>,
    last_direction: Direction,
}

fn door_to_direction(door: &str) -> Result<Direction> {
    match door {
        "north\n" => Ok(Direction::North),
        "south\n" => Ok(Direction::South),
        "west\n" => Ok(Direction::West),
        "east\n" => Ok(Direction::East),
        other => bail!("unknown direction: {other}"),
    }
}

fn go_to_room(intcode: &mut Intcode, current_path: &[Direction], destination_path: &[Direction]) {
    let min_len = || destination_path.len().min(current_path.len());

    let common_path_size = current_path
        .iter()
        .zip(destination_path)
        .position(|(x, y)| x != y)
        .unwrap_or_else(min_len);

    let iter = current_path
        .iter()
        .rev()
        .map(|&x| REVERSE_PATH_DIRECTIONS[x as usize])
        .take(current_path.len() - common_path_size)
        .chain(destination_path.iter().copied().skip(common_path_size))
        .flat_map(|x| (DIRECTION_INPUTS[x as usize].iter().copied()).map_into::<i64>());

    intcode.inputs.extend(iter);
}

fn explore(intcode: &mut Intcode) -> Result<ExplorationState> {
    let regex_room = Regex::new(r#"== (.+) =="#)?;
    let regex_doors = Regex::new(r#"(?s)Doors here lead:\n(.+?\n)\n"#)?;
    let regex_items = Regex::new(r#"(?s)Items here:\n(.+?\n)\n"#)?;

    let mut current_room = String::new();
    let mut current_path = Vec::new();
    let mut unknown_rooms = Vec::new();

    let mut visited_directions = HashMap::<_, SmallVec<[_; 4]>>::new();
    let mut items = Vec::new();
    let mut checkpoint_infos = None;

    loop {
        match intcode.run()? {
            State::Finished => bail!("early program termination"),
            State::NeedInput { outputs } => {
                if outputs.is_empty() {
                    bail!("empty output");
                }

                let text = String::from_utf8(outputs.iter().map(|&x| x as u8).collect_vec())?;

                current_room.clear();
                current_room.push_str(&regex_room.captures_iter(&text).last().value()?[1]);

                let items_iter = regex_items
                    .captures_iter(&text)
                    .last()
                    .and_then(|cap| cap.get(1))
                    .into_iter()
                    .flat_map(|x| x.as_str().split("- ").filter(|x| !x.is_empty()))
                    .filter(|&item| {
                        !matches!(
                            item,
                            "photons\n"
                                | "infinite loop\n"
                                | "molten lava\n"
                                | "giant electromagnet\n"
                                | "escape pod\n"
                        )
                    });

                for item in items_iter {
                    items.push(item.to_owned());
                    (intcode.inputs).extend(iter::chain(*b"take ", item.bytes()).map_into::<i64>());
                }

                let doors = regex_doors
                    .captures_iter(&text)
                    .last()
                    .and_then(|cap| cap.get(1))
                    .into_iter()
                    .flat_map(|x| x.as_str().split("- ").filter(|x| !x.is_empty()))
                    .collect_vec();

                if current_room == "Security Checkpoint" {
                    if checkpoint_infos.is_none() {
                        let previous_direction_index = *current_path.last().value()? as usize;
                        let reverse_direction = REVERSE_PATH_DIRECTIONS[previous_direction_index];
                        let reverse_door = REVERSE_DIRECTION_INPUTS[previous_direction_index];

                        let last_door = doors
                            .iter()
                            .find(|&door| door.as_bytes() != reverse_door)
                            .value()?;

                        let last_direction = door_to_direction(last_door)?;

                        checkpoint_infos = Some((current_path.clone(), last_direction));
                        current_path.pop();

                        visited_directions.insert(
                            current_room.clone(),
                            SmallVec::from_slice(&[reverse_direction]),
                        );
                        intcode
                            .inputs
                            .extend(reverse_door.iter().copied().map_into::<i64>());
                    }
                    continue;
                }

                for door in &doors {
                    let path_direction = door_to_direction(door)?;

                    let is_unknown = match visited_directions.get(&current_room) {
                        Some(directions) => !directions.contains(&path_direction),
                        None => {
                            visited_directions.insert(current_room.clone(), SmallVec::new());
                            true
                        }
                    };

                    if is_unknown
                        && current_path.last()
                            != Some(&REVERSE_PATH_DIRECTIONS[path_direction as usize])
                    {
                        let mut new_path = current_path.clone();
                        new_path.push(path_direction);
                        unknown_rooms.push((current_room.clone(), path_direction, new_path));
                    }
                }

                match unknown_rooms.pop() {
                    Some((room, path_direction, path)) => {
                        (visited_directions.get_mut(&room).value()?).push(path_direction);
                        go_to_room(intcode, &current_path, &path);
                        current_path = path;
                    }
                    None => {
                        let (checkpoint_path, last_direction) = checkpoint_infos.value()?;

                        return Ok(ExplorationState {
                            items,
                            current_path,
                            checkpoint_path,
                            last_direction,
                        });
                    }
                }
            }
        }
    }
}

fn go_to_security_checkpoint(intcode: &mut Intcode, exploration_state: &ExplorationState) {
    go_to_room(
        intcode,
        &exploration_state.current_path,
        &exploration_state.checkpoint_path,
    );

    let new_inputs = DIRECTION_INPUTS[exploration_state.last_direction as usize]
        .iter()
        .copied()
        .map_into()
        .collect_vec();

    let drop_item_inputs_iter = exploration_state
        .items
        .iter()
        .flat_map(|item| iter::chain(*b"drop ", item.bytes()))
        .map_into::<i64>();

    (intcode.inputs).extend(iter::chain(drop_item_inputs_iter, new_inputs));
}

fn force_pressure_sensitive_floor(
    intcode: &mut Intcode,
    exploration_state: &ExplorationState,
) -> Result<String> {
    let iter = (1u64..(1 << exploration_state.items.len()))
        .scan(0, |gray, index| {
            let new_gray = index ^ (index >> 1);
            let bit_changed = *gray ^ new_gray;
            *gray = new_gray;

            let item = &exploration_state.items[bit_changed.trailing_zeros() as usize];

            let action = if new_gray & bit_changed == 0 {
                "drop "
            } else {
                "take "
            };

            Some(
                action
                    .bytes()
                    .chain(item.bytes())
                    .chain(
                        DIRECTION_INPUTS[exploration_state.last_direction as usize]
                            .iter()
                            .copied(),
                    )
                    .map_into::<i64>(),
            )
        })
        .flatten();

    intcode.inputs.extend(iter);

    match intcode.run()? {
        State::Finished => {
            let outputs =
                String::from_utf8(intcode.outputs.iter().map(|&x| x as u8).collect_vec())?;

            Ok(outputs
                .split("\n\n")
                .filter(|x| !x.is_empty())
                .last()
                .value()?
                .lines()
                .join("\n"))
        }
        _ => bail!("unable to force pressure sensitive floor"),
    }
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);
    let input = input.trim();

    let program: HashMap<usize, i64> = input
        .split(',')
        .enumerate()
        .map(|(pos, val)| Result::Ok((pos, val.parse()?)))
        .try_collect()?;

    let mut intcode = Intcode::new(program, VecDeque::new());

    let exploration_state = explore(&mut intcode)?;
    go_to_security_checkpoint(&mut intcode, &exploration_state);

    let result = force_pressure_sensitive_floor(&mut intcode, &exploration_state)?;

    println!("{result}");
    Ok(())
}
