use aoc::*;

use eyre::ensure;
use itertools::Itertools;

use std::ops::RangeInclusive;

const RANGE: RangeInclusive<i64> = 200_000_000_000_000..=400_000_000_000_000;

#[derive(Clone, Copy)]
struct Vector {
    x: i128,
    y: i128,
    z: i128,
}

impl Vector {
    fn new(x: i128, y: i128, z: i128) -> Self {
        Self { x, y, z }
    }

    fn add(self, other: Self) -> Self {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    fn sub(self, other: Self) -> Self {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    fn cross(self, other: Self) -> Self {
        Vector {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    fn reduce(self) -> Self {
        let gcd = gcd(gcd(self.x, self.y), self.z);
        Vector {
            x: self.x / gcd,
            y: self.y / gcd,
            z: self.z / gcd,
        }
    }

    fn sum(self) -> i128 {
        self.x + self.y + self.z
    }
}

fn gcd(mut x: i128, mut y: i128) -> i128 {
    while y != 0 {
        (x, y) = (y, x % y);
    }
    x
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let positions_velocities: Vec<([i64; 3], [i64; 3])> = input
        .lines()
        .map(|line| {
            let (positions, velocities) = line.split(" @ ").next_tuple().value()?;

            Result::Ok((
                (positions.split(", ").map(|s| Ok(s.parse()?)))
                    .try_process(|mut iter| iter.next_array())?
                    .value()?,
                (velocities.split(", ").map(|s| Ok(s.parse()?)))
                    .try_process(|mut iter| iter.next_array())?
                    .value()?,
            ))
        })
        .try_collect()?;

    let result1 = (positions_velocities.iter().tuple_combinations())
        .filter(
            |(([px1, py1, _], [vx1, vy1, _]), ([px2, py2, _], [vx2, vy2, _]))| {
                let determinant = vy1 * vx2 - vx1 * vy2;

                if determinant == 0 {
                    return false;
                }

                let t1 = (vx2 * (py2 - py1) - vy2 * (px2 - px1)) / determinant;
                let t2 = (vx1 * (py2 - py1) - vy1 * (px2 - px1)) / determinant;

                let x = px1 + t1 * vx1;
                let y = py1 + t1 * vy1;

                t1 >= 0 && t2 >= 0 && RANGE.contains(&x) && RANGE.contains(&y)
            },
        )
        .count();

    let ((p0, v0), (p1, v1), (p2, v2)) = positions_velocities[..3]
        .iter()
        .map(|&([px, py, pz], [vx, vy, vz])| {
            let p = Vector::new(px as i128, py as i128, pz as i128);
            let v = Vector::new(vx as i128, vy as i128, vz as i128);
            (p, v)
        })
        .next_tuple()
        .value()?;

    let p3 = p1.sub(p0);
    let p4 = p2.sub(p0);
    let v3 = v1.sub(v0);
    let v4 = v2.sub(v0);

    let q = v3.cross(p3).reduce();
    let r = v4.cross(p4).reduce();
    let s = q.cross(r).reduce();

    let t1 = (p3.y * s.x - p3.x * s.y) / (v3.x * s.y - v3.y * s.x);
    let t2 = (p4.y * s.x - p4.x * s.y) / (v4.x * s.y - v4.y * s.x);

    ensure!(t1 != t2, "unsupported input");

    let a = p0.add(p3).sum();
    let b = p0.add(p4).sum();
    let c = v3.sub(v4).sum();

    let result2 = (t2 * a - t1 * b + t2 * t1 * c) / (t2 - t1);

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
