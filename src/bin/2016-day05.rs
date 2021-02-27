use smallvec::SmallVec;

use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("inputs/2016-day05.txt")?;
    let input = input.trim();

    let sub_hashes: Vec<_> = (0..)
        .map(|n| md5::compute(format!("{}{}", input, n)))
        .filter(|digest| digest[..2] == [0, 0] && digest[2] <= 0x0F)
        .map(|digest| (digest[2] as u32 % 16, digest[3] as u32 >> 4))
        .scan(
            (true, SmallVec::from_buf([false; 8])),
            |state, (fifth, sixth)| {
                if state.1.iter().all(|&x| x) {
                    state.0 = false;
                }

                if fifth < 8 && !state.1[fifth as usize] {
                    state.1[fifth as usize] = true;
                }

                Some((state.0, fifth, sixth))
            },
        )
        .take_while(|&(flag, _, _)| flag)
        .map(|(_, fifth, sixth)| (fifth, sixth))
        .collect();

    let result1: String = sub_hashes
        .iter()
        .map(|&(fifth, _)| std::char::from_digit(fifth, 16).unwrap())
        .take(8)
        .collect();

    let mut password = ['_'; 8];
    for &(fifth, sixth) in sub_hashes.iter().rev() {
        if fifth < 8 {
            password[fifth as usize] = std::char::from_digit(sixth, 16).unwrap();
        }
    }
    let result2: String = password.iter().collect();

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
