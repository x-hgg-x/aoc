use aoc::*;

use eyre::bail;
use itertools::Itertools;
use smallvec::SmallVec;

use std::collections::{HashMap, VecDeque};

enum State {
    NeedInput,
    HasOutputs(SmallVec<[i64; 3]>),
    Finished,
}

#[derive(Clone)]
struct Intcode {
    program: HashMap<usize, i64>,
    ip: usize,
    relative_base: i64,
    inputs: VecDeque<i64>,
}

impl Intcode {
    fn new(program: HashMap<usize, i64>, inputs: VecDeque<i64>) -> Self {
        Self { program, ip: 0, relative_base: 0, inputs }
    }

    fn get_input(&mut self, arg_position: usize, instruction: i64) -> Result<i64> {
        let arg = *self.program.entry(self.ip + arg_position).or_default();

        match instruction / 10i64.pow(1 + arg_position as u32) % 10 {
            0 => Ok(*self.program.entry(usize::try_from(arg)?).or_default()),
            1 => Ok(arg),
            2 => Ok(*self.program.entry(usize::try_from(self.relative_base + arg)?).or_default()),
            other => bail!("unknown parameter mode: {other}"),
        }
    }

    fn get_register(&mut self, arg_position: usize, instruction: i64) -> Result<&mut i64> {
        let arg = *self.program.entry(self.ip + arg_position).or_default();

        match instruction / 10i64.pow(1 + arg_position as u32) % 10 {
            0 => Ok(self.program.entry(usize::try_from(arg)?).or_default()),
            2 => Ok(self.program.entry(usize::try_from(self.relative_base + arg)?).or_default()),
            other => bail!("invalid parameter mode: {other}"),
        }
    }

    fn run(&mut self) -> Result<State> {
        let mut outputs = SmallVec::new();

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
                3 => {
                    let input = self.inputs.pop_front().unwrap_or(-1);
                    let arg1 = self.get_register(1, instruction)?;
                    *arg1 = input;
                    self.ip += 2;
                    return Ok(State::NeedInput);
                }
                4 => {
                    let arg1 = self.get_input(1, instruction)?;
                    self.ip += 2;
                    outputs.push(arg1);
                    if outputs.len() == 3 {
                        return Ok(State::HasOutputs(outputs));
                    }
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

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);
    let input = input.trim();

    let program: HashMap<usize, i64> = input.split(',').enumerate().map(|(pos, val)| Result::Ok((pos, val.parse()?))).try_collect()?;

    let mut computers = vec![Intcode::new(program, VecDeque::new()); 50];
    let computers: &mut [_; 50] = computers.as_mut_slice().try_into()?;

    for (index, computer) in computers.iter_mut().enumerate() {
        computer.inputs.push_back(index as i64);
    }

    let mut last_nat_received_packet = None;
    let mut first_y_sent_to_nat = None;
    let mut last_y_sent_by_nat = None;

    'outer: loop {
        let mut idle = computers.iter().all(|x| x.inputs.is_empty());

        for index in 0..computers.len() {
            match computers[index].run()? {
                State::Finished => break 'outer,
                State::NeedInput => continue,
                State::HasOutputs(outputs) => {
                    idle = false;

                    let [address, x, y] = <[_; 3]>::try_from(outputs.as_slice())?;

                    if address == 255 {
                        if first_y_sent_to_nat.is_none() {
                            first_y_sent_to_nat = Some(y);
                        }
                        last_nat_received_packet = Some([x, y]);
                    } else {
                        computers[address as usize].inputs.extend([x, y]);
                    }
                }
            }
        }

        if idle && computers.iter().all(|x| x.inputs.is_empty()) {
            if let Some([x, y]) = last_nat_received_packet {
                computers[0].inputs.extend([x, y]);

                if last_y_sent_by_nat == Some(y) {
                    break;
                } else {
                    last_y_sent_by_nat = Some(y);
                }
            }
        }
    }

    let result1 = first_y_sent_to_nat.value()?;
    let result2 = last_y_sent_by_nat.value()?;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
