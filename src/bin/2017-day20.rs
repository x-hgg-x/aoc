use aoc::*;

use itertools::Itertools;
use regex::Regex;

#[derive(Clone)]
struct Particle {
    index: usize,
    position: (i64, i64, i64),
    velocity: (i64, i64, i64),
    acceleration: (i64, i64, i64),
    destroyed: bool,
}

fn parse_vec3(s: &str) -> Result<(i64, i64, i64)> {
    s.split(',')
        .map(|x| Ok(x.trim().parse()?))
        .try_process(|mut iter| iter.next_tuple().value())?
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let re = Regex::new(r#"(?m)^p=<(.+?)>, v=<(.+?)>, a=<(.+?)>$"#)?;

    let mut particles: Vec<_> = re
        .captures_iter(&input)
        .enumerate()
        .map(|(index, cap)| {
            let position = parse_vec3(&cap[1])?;
            let velocity = parse_vec3(&cap[2])?;
            let acceleration = parse_vec3(&cap[3])?;

            Result::Ok(Particle {
                index,
                position,
                velocity,
                acceleration,
                destroyed: false,
            })
        })
        .try_collect()?;

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

    let result1 = particles
        .iter()
        .chain(&destroyed)
        .min_by_key(|x| x.position.0.abs() + x.position.1.abs() + x.position.2.abs())
        .map(|x| x.index)
        .value()?;

    let result2 = particles.len();

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
