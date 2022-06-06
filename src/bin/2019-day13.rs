use aoc::*;

use eyre::bail;
use itertools::Itertools;
use num_complex::Complex;
use smallvec::SmallVec;

use std::collections::HashMap;
use std::io::{StdoutLock, Write};
use std::iter::{once, repeat};
use std::time::Duration;

enum State {
    NeedInput,
    HasOutputs(SmallVec<[i64; 3]>),
    Finished,
}

struct Intcode {
    program: HashMap<usize, i64>,
    ip: usize,
    relative_base: i64,
    input: Option<i64>,
}

impl Intcode {
    fn new(program: HashMap<usize, i64>, input: Option<i64>) -> Self {
        Self { program, ip: 0, relative_base: 0, input }
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
                3 => match self.input.take() {
                    None => return Ok(State::NeedInput),
                    Some(input) => {
                        let arg1 = self.get_register(1, instruction)?;
                        *arg1 = input;
                        self.ip += 2;
                    }
                },
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

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq)]
enum Tile {
    Empty = b' ',
    Wall = b'#',
    Block = b'x',
    Paddle = b'-',
    Ball = b'O',
}

fn compute_grid_parameters(mut intcode: Intcode) -> Result<(usize, usize, usize, Complex<i64>)> {
    let mut grid = Vec::new();
    loop {
        match intcode.run()? {
            State::Finished => break,
            State::NeedInput => bail!("invalid program state"),
            State::HasOutputs(outputs) => {
                let position = Complex::new(outputs[0], outputs[1]);

                let tile = match outputs[2] {
                    0 => Tile::Empty,
                    1 => Tile::Wall,
                    2 => Tile::Block,
                    3 => Tile::Paddle,
                    4 => Tile::Ball,
                    other => bail!("unknown tile: {other}"),
                };

                grid.push((position, tile));
            }
        }
    }

    let block_count = grid.iter().filter(|&&(_, tile)| tile == Tile::Block).count();

    let mut min_re = 0;
    let mut max_re = 0;
    let mut min_im = 0;
    let mut max_im = 0;

    for (position, _) in &grid {
        min_re = min_re.min(position.re);
        max_re = max_re.max(position.re);
        min_im = min_im.min(position.im);
        max_im = max_im.max(position.im);
    }

    let width = (max_re - min_re + 1) as usize;
    let height = (max_im - min_im + 1) as usize;

    Ok((block_count, width, height, Complex::new(min_re, min_im)))
}

fn draw(image: &mut [u8], grid: &[(Complex<i64>, Tile)], stdout: &mut StdoutLock, score: i64, width: usize, height: usize, origin: Complex<i64>) -> Result<()> {
    for &(position, tile) in grid {
        if position != Complex::new(-1, 0) {
            let x = (position - origin).re as usize;
            let y = (position - origin).im as usize;
            image[(width + 1) * y + x] = tile as u8;
        }
    }

    writeln!(stdout, "\nScore: {score}\n\n{}\x1b[{}A", String::from_utf8_lossy(image), height + 4)?;
    std::thread::sleep(Duration::from_millis(1));
    Ok(())
}

fn play(mut intcode: Intcode, width: usize, height: usize, origin: Complex<i64>) -> Result<i64> {
    let mut stdout = std::io::stdout().lock();

    let mut ball_x = 0i64;
    let mut paddle_x = 0i64;
    let mut score = 0;

    let mut image = repeat(repeat(b' ').take(width).chain(once(b'\n'))).take(height).flatten().collect_vec();

    let mut grid = Vec::new();
    loop {
        match intcode.run()? {
            State::Finished => {
                draw(&mut image, &grid, &mut stdout, score, width, height, origin)?;
                writeln!(stdout, "\x1b[{}B", height + 4)?;
                break;
            }
            State::NeedInput => {
                draw(&mut image, &grid, &mut stdout, score, width, height, origin)?;
                intcode.input = Some((ball_x - paddle_x).signum())
            }
            State::HasOutputs(outputs) => {
                let position = Complex::new(outputs[0], outputs[1]);

                if position != Complex::new(-1, 0) {
                    let tile = match outputs[2] {
                        0 => Tile::Empty,
                        1 => Tile::Wall,
                        2 => Tile::Block,
                        3 => {
                            paddle_x = position.re;
                            Tile::Paddle
                        }
                        4 => {
                            ball_x = position.re;
                            Tile::Ball
                        }
                        other => bail!("unknown tile: {other}"),
                    };
                    grid.push((position, tile));
                } else {
                    score = outputs[2];
                }
            }
        }
    }

    Ok(score)
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);
    let input = input.trim();

    let mut program: HashMap<usize, i64> = input.split(',').enumerate().map(|(pos, val)| Result::Ok((pos, val.parse()?))).try_collect()?;

    let (block_count, width, height, origin) = compute_grid_parameters(Intcode::new(program.clone(), None))?;
    let result1 = block_count;

    program.insert(0, 2);
    let result2 = play(Intcode::new(program, None), width, height, origin)?;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
