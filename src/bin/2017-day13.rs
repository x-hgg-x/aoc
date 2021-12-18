use aoc::*;

use itertools::Itertools;

struct Layer {
    depth: usize,
    range: usize,
    period: usize,
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let layers: Vec<_> = input
        .lines()
        .map(|line| {
            let (depth, range) = line.split(": ").map(|x| Ok(x.parse()?)).try_process(|mut iter| iter.next_tuple())?.value()?;
            let period = if range == 0 { 0 } else { (range - 1) * 2 };
            Result::Ok(Layer { depth, range, period })
        })
        .try_collect()?;

    let result1 = layers.iter().filter(|x| x.depth % x.period == 0).map(|x| x.depth * x.range).sum::<usize>();

    let mut delay = 0;
    while layers.iter().any(|x| (delay + x.depth) % x.period == 0) {
        delay += 1;
    }

    let result2 = delay;

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
