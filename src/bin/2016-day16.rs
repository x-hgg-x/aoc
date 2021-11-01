use std::fs;

use itertools::Itertools;

fn compute_checksum(input: &[u8], disk_size: usize) -> String {
    let mut disk = Vec::with_capacity(disk_size);
    disk.extend(input.iter().map(|&x| x - b'0'));

    while disk.len() < disk_size {
        let len = disk.len();

        disk.push(0);
        disk.extend_from_within(..len);

        let right = &mut disk[(len + 1)..];
        right.reverse();
        for x in right {
            *x = (*x == 0) as u8;
        }
    }
    disk.truncate(disk_size);

    let mut buf = disk;
    let mut checksum = buf.chunks_exact(2).map(|x| (x[0] == x[1]) as u8).collect_vec();

    while checksum.len() % 2 == 0 {
        std::mem::swap(&mut buf, &mut checksum);
        checksum.clear();
        checksum.extend(buf.chunks_exact(2).map(|x| (x[0] == x[1]) as u8));
    }

    for x in &mut checksum {
        *x += b'0';
    }
    String::from_utf8_lossy(&checksum).into_owned()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("inputs/2016-day16.txt")?;
    let input = input.trim().as_bytes();

    let result1 = compute_checksum(input, 272);
    let result2 = compute_checksum(input, 35651584);

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
