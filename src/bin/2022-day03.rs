use aoc::*;

use itertools::Itertools;

fn fill_sort_dedup(buf: &mut Vec<u8>, data: &[u8]) {
    buf.clear();
    buf.extend_from_slice(data);
    buf.sort_unstable();
    buf.dedup();
}

fn compute_priority(c: u8) -> u64 {
    let c = c as u64;
    (c & 0x1f) + 26 * (c & 0x20 == 0) as u64
}

fn compute_intersection_priority(intersection: &mut Vec<u8>, buf: &mut Vec<u8>, init_data: &[u8], chunks: &[&[u8]]) -> u64 {
    fill_sort_dedup(intersection, init_data);

    for &data in chunks {
        fill_sort_dedup(buf, data);

        let mut buf_iter = buf.iter();
        intersection.retain(|&x| buf_iter.take_while_ref(|&&item| item <= x).any(|&item| item == x));
    }

    intersection.iter().map(|&c| compute_priority(c)).sum()
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let lines = input.lines().collect_vec();

    let mut intersection = Vec::new();
    let mut buffer = Vec::new();

    let result1 = lines
        .iter()
        .map(|line| {
            let (c1, c2) = line.as_bytes().split_at(line.len() / 2);
            compute_intersection_priority(&mut intersection, &mut buffer, c1, &[c2])
        })
        .sum::<u64>();

    let result2 = lines
        .chunks_exact(3)
        .map(|x| {
            let chunks = [x[0].as_bytes(), x[1].as_bytes(), x[2].as_bytes()];
            compute_intersection_priority(&mut intersection, &mut buffer, chunks[0], &chunks[1..])
        })
        .sum::<u64>();

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
