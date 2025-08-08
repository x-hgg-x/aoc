use aoc::*;

use itertools::Itertools;
use regex::Regex;

use std::iter;

#[derive(Clone)]
struct Cuboid {
    start: [i64; 3],
    end: [i64; 3],
}

impl Cuboid {
    fn intersect(&self, other: &Self) -> Option<Self> {
        let start = [self.start[0].max(other.start[0]), self.start[1].max(other.start[1]), self.start[2].max(other.start[2])];
        let end = [self.end[0].min(other.end[0]), self.end[1].min(other.end[1]), self.end[2].min(other.end[2])];
        iter::zip(start, end).all(|(s, e)| s <= e).then_some(Cuboid { start, end })
    }

    fn difference(&self, intersection: &Self) -> impl Iterator<Item = Cuboid> + use<> {
        [
            (self.start[0] < intersection.start[0]).then(|| Cuboid { start: self.start, end: [intersection.start[0] - 1, self.end[1], self.end[2]] }),
            (self.start[1] < intersection.start[1]).then(|| Cuboid {
                start: [intersection.start[0], self.start[1], self.start[2]],
                end: [intersection.end[0], intersection.start[1] - 1, self.end[2]],
            }),
            (self.start[2] < intersection.start[2]).then(|| Cuboid {
                start: [intersection.start[0], intersection.start[1], self.start[2]],
                end: [intersection.end[0], intersection.end[1], intersection.start[2] - 1],
            }),
            (intersection.end[0] < self.end[0]).then(|| Cuboid { start: [intersection.end[0] + 1, self.start[1], self.start[2]], end: self.end }),
            (intersection.end[1] < self.end[1]).then(|| Cuboid {
                start: [intersection.start[0], intersection.end[1] + 1, self.start[2]],
                end: [intersection.end[0], self.end[1], self.end[2]],
            }),
            (intersection.end[2] < self.end[2]).then(|| Cuboid {
                start: [intersection.start[0], intersection.start[1], intersection.end[2] + 1],
                end: [intersection.end[0], intersection.end[1], self.end[2]],
            }),
        ]
        .into_iter()
        .flatten()
    }

    fn volume(&self) -> i64 {
        iter::zip(self.start, self.end).map(|(start, end)| end - start + 1).product()
    }
}

struct Instruction {
    toogle: bool,
    cuboid: Cuboid,
}

fn step(instruction: &Instruction, cuboids: &mut Vec<Cuboid>, buf: &mut Vec<Cuboid>, total_volume: &mut i64) {
    if !cuboids.is_empty() {
        buf.clear();

        cuboids.retain(|cuboid| match cuboid.intersect(&instruction.cuboid) {
            None => true,
            Some(intersection) => {
                buf.extend(cuboid.difference(&intersection));
                *total_volume -= intersection.volume();
                false
            }
        });

        cuboids.extend_from_slice(buf);
    }

    if instruction.toogle {
        cuboids.push(instruction.cuboid.clone());
        *total_volume += instruction.cuboid.volume();
    }
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let re = Regex::new(r#"(?m)^(on|off) x=(.+?)\.\.(.+?),y=(.+?)\.\.(.+?),z=(.+?)\.\.(.+?)$"#)?;

    let instructions: Vec<_> = re
        .captures_iter(&input)
        .map(|cap| {
            let toogle = &cap[1] == "on";
            let start = [cap[2].parse()?, cap[4].parse()?, cap[6].parse()?];
            let end = [cap[3].parse()?, cap[5].parse()?, cap[7].parse()?];
            Result::Ok(Instruction { toogle, cuboid: Cuboid { start, end } })
        })
        .try_collect()?;

    let end_init_position = instructions
        .iter()
        .position(|instruction| {
            let Cuboid { start, end } = instruction.cuboid;
            [start, end].into_iter().flatten().any(|v| !(-50..=50).contains(&v))
        })
        .value()?;

    let mut cuboids = Vec::new();
    let mut buf = Vec::new();
    let mut total_volume = 0i64;

    for instruction in &instructions[..end_init_position] {
        step(instruction, &mut cuboids, &mut buf, &mut total_volume)
    }
    let result1 = total_volume;

    for instruction in &instructions[end_init_position..] {
        step(instruction, &mut cuboids, &mut buf, &mut total_volume)
    }
    let result2 = total_volume;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
