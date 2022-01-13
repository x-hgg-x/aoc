use aoc::*;

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let steps = input.trim().parse::<usize>()?;

    let mut buffer = Vec::with_capacity(2018);
    buffer.push(0usize);

    let mut current_position = 0;

    for i in 1..=2017 {
        current_position = (current_position + steps) % buffer.len() + 1;
        buffer.insert(current_position, i);
    }
    let result1 = buffer[(current_position + 1) % buffer.len()];

    let mut after_0 = buffer[1];
    for i in 2018..=50_000_000 {
        let len = buffer.len() + i - 2018;
        current_position = (current_position + steps) % len + 1;
        if current_position == 1 {
            after_0 = i;
        }
    }
    let result2 = after_0;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
