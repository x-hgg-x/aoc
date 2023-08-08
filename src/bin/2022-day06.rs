use aoc::*;

fn marker<const N: usize>(input: &[u8]) -> Result<usize> {
    let mut buf = [0; N];

    let position = input.windows(N).position(|slice| {
        buf.copy_from_slice(slice);
        buf.sort_unstable();
        buf.windows(2).all(|x| x[0] != x[1])
    });

    Ok(position.value()? + N)
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);
    let input = input.trim().as_bytes();

    let result1 = marker::<4>(input)?;
    let result2 = marker::<14>(input)?;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
