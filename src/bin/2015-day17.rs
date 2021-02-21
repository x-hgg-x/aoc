use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("inputs/2015-day17.txt")?;

    let set: Vec<u32> = input
        .split_ascii_whitespace()
        .flat_map(|x| x.parse().ok())
        .collect();

    let max: u32 = 1 << set.len();

    const SUM: u32 = 150;

    let iter = (1..max)
        .scan((0, 0), |state, index| {
            let new_gray = index ^ (index >> 1);
            let bit_changed = state.1 ^ new_gray;
            let diff = set[bit_changed.trailing_zeros() as usize];
            if new_gray & bit_changed == 0 {
                state.0 -= diff;
            } else {
                state.0 += diff;
            }
            state.1 = new_gray;

            Some((state.0, new_gray.count_ones()))
        })
        .filter(|&(sum, _)| sum == SUM)
        .map(|(_, size)| size);

    let result1 = iter.clone().count();

    let min = iter.clone().min().unwrap();
    let result2 = iter.filter(|&x| x == min).count();

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
