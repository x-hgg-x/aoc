use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("inputs/2015-day04.txt")?;
    let input = input.trim();

    let mut result1: u64 = 1;
    loop {
        let digest = md5::compute(format!("{}{}", input, result1));
        if digest[..2] == [0, 0] && digest[2] <= 0x0F {
            break;
        }
        result1 += 1;
    }

    let mut result2: u64 = 1;
    loop {
        let digest = md5::compute(format!("{}{}", input, result2));
        if digest[..3] == [0, 0, 0] {
            break;
        }
        result2 += 1;
    }

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
