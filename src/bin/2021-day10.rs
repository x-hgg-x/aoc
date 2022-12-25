use aoc::*;

use itertools::Itertools;

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let mut buf = Vec::new();
    let mut corrupted_score = 0u64;

    let incomplete_scores = input
        .lines()
        .filter_map(|line| {
            buf.clear();

            for c in line.bytes() {
                match c {
                    b'(' | b'[' | b'{' | b'<' => buf.push(c),
                    b')' if buf.pop() != Some(b'(') => {
                        corrupted_score += 3;
                        return None;
                    }
                    b']' if buf.pop() != Some(b'[') => {
                        corrupted_score += 57;
                        return None;
                    }
                    b'}' if buf.pop() != Some(b'{') => {
                        corrupted_score += 1197;
                        return None;
                    }
                    b'>' if buf.pop() != Some(b'<') => {
                        corrupted_score += 25137;
                        return None;
                    }
                    _ => (),
                }
            }

            let mut incomplete_score = 0u64;

            for &c in buf.iter().rev() {
                match c {
                    b'(' => incomplete_score = incomplete_score * 5 + 1,
                    b'[' => incomplete_score = incomplete_score * 5 + 2,
                    b'{' => incomplete_score = incomplete_score * 5 + 3,
                    b'<' => incomplete_score = incomplete_score * 5 + 4,
                    _ => (),
                }
            }

            Some(incomplete_score)
        })
        .sorted_unstable()
        .collect_vec();

    let index = incomplete_scores.len() / 2;

    let result1 = corrupted_score;
    let result2 = incomplete_scores[index];

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
