use md5::Digest;

use std::fs;

fn find_digest<F: Fn(&Digest) -> bool>(input: &str, f: F) -> i32 {
    (1..)
        .map(|n| (n, md5::compute(format!("{}{}", input, n))))
        .find(|(_, digest)| f(digest))
        .map(|(n, _)| n)
        .unwrap()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("inputs/2015-day04.txt")?;
    let input = input.trim();

    let result1 = find_digest(&input, |digest| digest[..2] == [0, 0] && digest[2] <= 0x0F);
    let result2 = find_digest(&input, |digest| digest[..3] == [0, 0, 0]);

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
