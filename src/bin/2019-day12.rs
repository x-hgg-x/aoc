use aoc::*;

use itertools::{izip, Itertools};
use regex::Regex;

fn gcd(mut x: usize, mut y: usize) -> usize {
    while y != 0 {
        (x, y) = (y, x % y);
    }
    x
}

fn lcm(x: usize, y: usize) -> usize {
    x * y / gcd(x, y)
}

fn step(positions: &mut [[i64; 3]], velocities: &mut [[i64; 3]]) {
    for (index1, &[x1, y1, z1]) in positions.iter().enumerate() {
        for (index2, &[x2, y2, z2]) in positions.iter().enumerate().skip(index1 + 1) {
            let (sx, sy, sz) = ((x2 - x1).signum(), (y2 - y1).signum(), (z2 - z1).signum());

            let [vx1, vy1, vz1] = &mut velocities[index1];
            (*vx1, *vy1, *vz1) = (*vx1 + sx, *vy1 + sy, *vz1 + sz);

            let [vx2, vy2, vz2] = &mut velocities[index2];
            (*vx2, *vy2, *vz2) = (*vx2 - sx, *vy2 - sy, *vz2 - sz);
        }
    }

    for ([x, y, z], &[vx, vy, vz]) in positions.iter_mut().zip(&*velocities) {
        (*x, *y, *z) = (*x + vx, *y + vy, *z + vz);
    }
}

fn total_energy(mut positions: Vec<[i64; 3]>, mut velocities: Vec<[i64; 3]>) -> i64 {
    for _ in 0..1000 {
        step(&mut positions, &mut velocities);
    }

    positions.iter().zip(&velocities).map(|([x, y, z], &[vx, vy, vz])| (x.abs() + y.abs() + z.abs()) * (vx.abs() + vy.abs() + vz.abs())).sum()
}

fn compute_cycle_size(initial_positions: Vec<[i64; 3]>, initial_velocities: Vec<[i64; 3]>) -> usize {
    let mut positions = initial_positions.clone();
    let mut velocities = initial_velocities.clone();

    let mut steps = 0;
    let mut cycle_sizes = [0; 3];

    while cycle_sizes.contains(&0) {
        step(&mut positions, &mut velocities);
        steps += 1;

        for (dim, cycle_size) in cycle_sizes.iter_mut().enumerate() {
            let mut iter = izip!(&positions, &velocities, &initial_positions, &initial_velocities);
            if *cycle_size == 0 && iter.all(|(p, v, ip, iv)| p[dim] == ip[dim] && v[dim] == iv[dim]) {
                *cycle_size = steps;
            }
        }
    }

    lcm(lcm(cycle_sizes[0], cycle_sizes[1]), cycle_sizes[2])
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let re = Regex::new(r#"(?m)^<x=(.+?), y=(.+?), z=(.+?)>$"#)?;

    let initial_positions: Vec<_> = re.captures_iter(&input).map(|cap| Result::Ok([cap[1].parse()?, cap[2].parse()?, cap[3].parse()?])).try_collect()?;
    let initial_velocities = vec![[0; 3]; initial_positions.len()];

    let result1 = total_energy(initial_positions.clone(), initial_velocities.clone());
    let result2 = compute_cycle_size(initial_positions, initial_velocities);

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
