use eyre::Result;
use itertools::Itertools;
use regex::Regex;

use std::fs;

#[derive(Clone)]
struct Particle {
    index: usize,
    position: (i64, i64, i64),
    velocity: (i64, i64, i64),
    acceleration: (i64, i64, i64),
    destroyed: bool,
}

fn main() -> Result<()> {
    let input = fs::read_to_string("inputs/2017-day20.txt")?;

    let re = Regex::new(r#"(?m)^p=<(.+?)>, v=<(.+?)>, a=<(.+?)>$"#)?;

    let mut particles = re
        .captures_iter(&input)
        .enumerate()
        .map(|(index, cap)| {
            let position = cap[1].split(',').map(|x| x.trim().parse().unwrap()).next_tuple().unwrap();
            let velocity = cap[2].split(',').map(|x| x.trim().parse().unwrap()).next_tuple().unwrap();
            let acceleration = cap[3].split(',').map(|x| x.trim().parse().unwrap()).next_tuple().unwrap();
            Particle { index, position, velocity, acceleration, destroyed: false }
        })
        .collect_vec();

    let mut destroyed = Vec::new();

    for _ in 0..1000 {
        for particle in particles.iter_mut().chain(&mut destroyed) {
            particle.velocity.0 += particle.acceleration.0;
            particle.velocity.1 += particle.acceleration.1;
            particle.velocity.2 += particle.acceleration.2;
            particle.position.0 += particle.velocity.0;
            particle.position.1 += particle.velocity.1;
            particle.position.2 += particle.velocity.2;
        }

        particles.sort_unstable_by_key(|x| x.position);

        for index in 0..particles.len() - 1 {
            let slice = &mut particles[index..index + 2];
            if slice.iter().map(|x| x.position).all_equal() {
                slice.iter_mut().for_each(|x| x.destroyed = true);
            }
        }

        destroyed.extend(particles.iter().filter(|x| x.destroyed).cloned());
        particles.retain(|x| !x.destroyed);
    }

    let result1 = particles.iter().chain(&destroyed).min_by_key(|x| x.position.0.abs() + x.position.1.abs() + x.position.2.abs()).map(|x| x.index).unwrap();
    let result2 = particles.len();

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
