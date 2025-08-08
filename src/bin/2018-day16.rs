use aoc::*;

use eyre::{Report, bail, ensure};
use itertools::Itertools;
use regex::Regex;

struct Sample {
    before: [i64; 4],
    instruction: (usize, [i64; 3]),
    after: [i64; 4],
}

impl Sample {
    fn check(&self) -> Result<u16> {
        let mut valid_instructions = 0xFFFF;

        let (_, [a, b, c]) = self.instruction;
        let c = c as usize;

        let check = |instruction: Instruction, input_a, input_b, valid_instructions: &mut _| {
            instruction.check(
                self.before,
                self.after,
                input_a,
                input_b,
                c,
                valid_instructions,
            )
        };

        let valid_register_a = (0..4).contains(&a);
        let valid_register_b = (0..4).contains(&b);

        if valid_register_a && valid_register_b {
            let input_a = Input::Register(a as usize);
            let input_b = Input::Register(b as usize);

            let check = |instruction: Instruction, valid_instructions: &mut _| {
                check(instruction, input_a, input_b, valid_instructions)
            };

            check(Instruction::Addr, &mut valid_instructions)?;
            check(Instruction::Mulr, &mut valid_instructions)?;
            check(Instruction::Banr, &mut valid_instructions)?;
            check(Instruction::Borr, &mut valid_instructions)?;
            check(Instruction::Gtrr, &mut valid_instructions)?;
            check(Instruction::Eqrr, &mut valid_instructions)?;
        }

        if valid_register_a {
            let input_a = Input::Register(a as usize);
            let input_b = Input::Value(b);

            let check = |instruction: Instruction, valid_instructions: &mut _| {
                check(instruction, input_a, input_b, valid_instructions)
            };

            check(Instruction::Addi, &mut valid_instructions)?;
            check(Instruction::Muli, &mut valid_instructions)?;
            check(Instruction::Bani, &mut valid_instructions)?;
            check(Instruction::Bori, &mut valid_instructions)?;
            check(Instruction::Setr, &mut valid_instructions)?;
            check(Instruction::Gtri, &mut valid_instructions)?;
            check(Instruction::Eqri, &mut valid_instructions)?;
        }

        if valid_register_b {
            let input_a = Input::Value(a);
            let input_b = Input::Register(b as usize);

            let check = |instruction: Instruction, valid_instructions: &mut _| {
                check(instruction, input_a, input_b, valid_instructions)
            };

            check(Instruction::Gtir, &mut valid_instructions)?;
            check(Instruction::Eqir, &mut valid_instructions)?;
        }

        check(
            Instruction::Seti,
            Input::Value(a),
            Input::Value(b),
            &mut valid_instructions,
        )?;

        Ok(valid_instructions)
    }
}

#[derive(Copy, Clone)]
enum Input {
    Register(usize),
    Value(i64),
}

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
enum Instruction {
    Addr = 0,
    Addi = 1,
    Mulr = 2,
    Muli = 3,
    Banr = 4,
    Bani = 5,
    Borr = 6,
    Bori = 7,
    Setr = 8,
    Seti = 9,
    Gtir = 10,
    Gtri = 11,
    Gtrr = 12,
    Eqir = 13,
    Eqri = 14,
    Eqrr = 15,
    None = 16,
}

impl TryFrom<u32> for Instruction {
    type Error = Report;

    fn try_from(opcode: u32) -> Result<Self> {
        match opcode {
            0 => Ok(Self::Addr),
            1 => Ok(Self::Addi),
            2 => Ok(Self::Mulr),
            3 => Ok(Self::Muli),
            4 => Ok(Self::Banr),
            5 => Ok(Self::Bani),
            6 => Ok(Self::Borr),
            7 => Ok(Self::Bori),
            8 => Ok(Self::Setr),
            9 => Ok(Self::Seti),
            10 => Ok(Self::Gtir),
            11 => Ok(Self::Gtri),
            12 => Ok(Self::Gtrr),
            13 => Ok(Self::Eqir),
            14 => Ok(Self::Eqri),
            15 => Ok(Self::Eqrr),
            _ => bail!("unknown opcode"),
        }
    }
}

impl Instruction {
    fn execute(
        &self,
        registers: &mut [i64; 4],
        input_a: Input,
        input_b: Input,
        c: usize,
    ) -> Result<()> {
        match (self, input_a, input_b) {
            (Instruction::Addr, Input::Register(a), Input::Register(b)) => {
                registers[c] = registers[a] + registers[b]
            }
            (Instruction::Addi, Input::Register(a), Input::Value(b)) => {
                registers[c] = registers[a] + b
            }
            (Instruction::Mulr, Input::Register(a), Input::Register(b)) => {
                registers[c] = registers[a] * registers[b]
            }
            (Instruction::Muli, Input::Register(a), Input::Value(b)) => {
                registers[c] = registers[a] * b
            }
            (Instruction::Banr, Input::Register(a), Input::Register(b)) => {
                registers[c] = registers[a] & registers[b]
            }
            (Instruction::Bani, Input::Register(a), Input::Value(b)) => {
                registers[c] = registers[a] & b
            }
            (Instruction::Borr, Input::Register(a), Input::Register(b)) => {
                registers[c] = registers[a] | registers[b]
            }
            (Instruction::Bori, Input::Register(a), Input::Value(b)) => {
                registers[c] = registers[a] | b
            }
            (Instruction::Setr, Input::Register(a), _) => registers[c] = registers[a],
            (Instruction::Seti, Input::Value(a), _) => registers[c] = a,
            (Instruction::Gtir, Input::Value(a), Input::Register(b)) => {
                registers[c] = (a > registers[b]) as i64
            }
            (Instruction::Gtri, Input::Register(a), Input::Value(b)) => {
                registers[c] = (registers[a] > b) as i64
            }
            (Instruction::Gtrr, Input::Register(a), Input::Register(b)) => {
                registers[c] = (registers[a] > registers[b]) as i64
            }
            (Instruction::Eqir, Input::Value(a), Input::Register(b)) => {
                registers[c] = (a == registers[b]) as i64
            }
            (Instruction::Eqri, Input::Register(a), Input::Value(b)) => {
                registers[c] = (registers[a] == b) as i64
            }
            (Instruction::Eqrr, Input::Register(a), Input::Register(b)) => {
                registers[c] = (registers[a] == registers[b]) as i64
            }
            _ => bail!("unknown instruction"),
        }

        Ok(())
    }

    fn check(
        &self,
        mut before: [i64; 4],
        after: [i64; 4],
        input_a: Input,
        input_b: Input,
        c: usize,
        valid_instructions: &mut u16,
    ) -> Result<()> {
        self.execute(&mut before, input_a, input_b, c)?;

        if before != after {
            *valid_instructions &= !(1 << (*self as u8));
        }

        Ok(())
    }
}

fn check_instructions_mapping(
    mut opcodes: [Instruction; 16],
    possible_opcodes: &[u16; 16],
) -> Result<()> {
    opcodes.sort_unstable();

    ensure!(
        possible_opcodes.iter().all(|&x| x == 0),
        "unable to map opcodes to instructions"
    );

    ensure!(
        (opcodes.iter().enumerate()).all(|(index, &x)| x as usize == index),
        "unable to map opcodes to instructions"
    );

    Ok(())
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let re = Regex::new(
        r#"Before: \[(\d+), (\d+), (\d+), (\d+)\]\s+(\d+) (\d+) (\d+) (\d+)\s+After:  \[(\d+), (\d+), (\d+), (\d+)\]"#,
    )?;

    let mut possible_opcodes = [0xFFFFu16; 16];

    let result1 = re
        .captures_iter(&input)
        .map(|cap| {
            let sample = Sample {
                before: [
                    cap[1].parse()?,
                    cap[2].parse()?,
                    cap[3].parse()?,
                    cap[4].parse()?,
                ],
                instruction: (
                    cap[5].parse()?,
                    [cap[6].parse()?, cap[7].parse()?, cap[8].parse()?],
                ),
                after: [
                    cap[9].parse()?,
                    cap[10].parse()?,
                    cap[11].parse()?,
                    cap[12].parse()?,
                ],
            };

            let valid_instructions = sample.check()?;

            let (opcode_index, _) = sample.instruction;
            possible_opcodes[opcode_index] &= valid_instructions;

            Ok((valid_instructions.count_ones() >= 3) as u64)
        })
        .try_sum::<u64>()?;

    let mut opcodes = [Instruction::None; 16];

    while let Some((index, &possible_opcode)) =
        (possible_opcodes.iter()).find_position(|possible_opcode| possible_opcode.count_ones() == 1)
    {
        opcodes[index] = possible_opcode.trailing_zeros().try_into()?;
        (possible_opcodes.iter_mut()).for_each(|x| *x &= !possible_opcode);
    }

    check_instructions_mapping(opcodes, &possible_opcodes)?;

    let mut registers = [0; 4];

    input[re.find_iter(&input).last().value()?.end()..]
        .lines()
        .filter(|&line| !line.is_empty())
        .try_for_each(|line| {
            let (opcode_index, a, b, c) = line.split_ascii_whitespace().next_tuple().value()?;
            let instruction = opcodes[opcode_index.parse::<usize>()?];

            let (input_a, input_b) = match instruction {
                Instruction::Addr => (Input::Register(a.parse()?), Input::Register(b.parse()?)),
                Instruction::Addi => (Input::Register(a.parse()?), Input::Value(b.parse()?)),
                Instruction::Mulr => (Input::Register(a.parse()?), Input::Register(b.parse()?)),
                Instruction::Muli => (Input::Register(a.parse()?), Input::Value(b.parse()?)),
                Instruction::Banr => (Input::Register(a.parse()?), Input::Register(b.parse()?)),
                Instruction::Bani => (Input::Register(a.parse()?), Input::Value(b.parse()?)),
                Instruction::Borr => (Input::Register(a.parse()?), Input::Register(b.parse()?)),
                Instruction::Bori => (Input::Register(a.parse()?), Input::Value(b.parse()?)),
                Instruction::Setr => (Input::Register(a.parse()?), Input::Value(b.parse()?)),
                Instruction::Seti => (Input::Value(a.parse()?), Input::Value(b.parse()?)),
                Instruction::Gtir => (Input::Value(a.parse()?), Input::Register(b.parse()?)),
                Instruction::Gtri => (Input::Register(a.parse()?), Input::Value(b.parse()?)),
                Instruction::Gtrr => (Input::Register(a.parse()?), Input::Register(b.parse()?)),
                Instruction::Eqir => (Input::Value(a.parse()?), Input::Register(b.parse()?)),
                Instruction::Eqri => (Input::Register(a.parse()?), Input::Value(b.parse()?)),
                Instruction::Eqrr => (Input::Register(a.parse()?), Input::Register(b.parse()?)),
                _ => bail!("unknown instruction"),
            };

            instruction.execute(&mut registers, input_a, input_b, c.parse()?)
        })?;

    let result2 = registers[0];

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
