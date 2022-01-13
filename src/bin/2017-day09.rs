use aoc::*;

fn main() -> Result<()> {
    let input = setup(file!())?;

    let mut group_score = 1u64;
    let mut cancelled = false;
    let mut garbage = false;
    let mut total_score = 0;
    let mut garbage_count = 0usize;

    for &c in &input {
        if cancelled {
            cancelled = false;
            continue;
        }

        match c {
            b'!' => cancelled = true,
            b'<' if !garbage => garbage = true,
            b'>' if garbage => garbage = false,
            b'{' if !garbage => {
                total_score += group_score;
                group_score += 1;
            }
            b'}' if !garbage => group_score -= 1,
            _ if garbage => garbage_count += 1,
            _ => (),
        }
    }

    let result1 = total_score;
    let result2 = garbage_count;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
