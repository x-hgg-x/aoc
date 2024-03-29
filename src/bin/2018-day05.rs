use aoc::*;

fn react(input: &[u8], output: &mut Vec<u8>, removed_char: Option<u8>) -> usize {
    output.clear();

    for &c in input {
        if let Some(removed_char) = removed_char {
            if c.eq_ignore_ascii_case(&removed_char) {
                continue;
            }
        }

        match output.last() {
            Some(&last) if last.eq_ignore_ascii_case(&c) && last != c => {
                output.pop();
            }
            _ => output.push(c),
        };
    }

    output.len()
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);
    let input = input.trim().as_bytes();

    let mut output = Vec::with_capacity(input.len());

    let result1 = react(input, &mut output, None);
    let result2 = (b'a'..=b'z').map(|c| react(input, &mut output, Some(c))).min().value()?;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
