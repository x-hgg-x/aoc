use itertools::{iproduct, Itertools};
use regex::bytes::Regex;

use std::fs;
use std::iter::once;

fn has_abba(bytes_list: &[&[u8]]) -> bool {
    bytes_list.iter().any(|bytes| {
        bytes
            .windows(4)
            .any(|x| x[0] != x[1] && x[0] == x[3] && x[1] == x[2])
    })
}

fn get_aba<'a>(bytes_list: &'a [&[u8]]) -> impl Iterator<Item = &'a [u8]> + Clone {
    bytes_list
        .iter()
        .flat_map(|bytes| bytes.windows(3).filter(|x| x[0] == x[2] && x[0] != x[1]))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("inputs/2016-day07.txt")?;

    let re = Regex::new(r#"\[\w+\]"#).unwrap();

    let ips = input
        .lines()
        .map(|line| {
            let bytes = line.as_bytes();

            let ranges = re.find_iter(bytes).map(|x| x.range()).collect_vec();

            let hypernets = ranges
                .iter()
                .map(|range| &bytes[range.start + 1..range.end - 1])
                .collect_vec();

            let supernets = once(0..0)
                .chain(ranges)
                .chain(once(bytes.len()..bytes.len()))
                .tuple_windows()
                .map(|(x, y)| (x.end..y.start))
                .filter(|x| !x.is_empty())
                .map(|range| &bytes[range])
                .collect_vec();

            (hypernets, supernets)
        })
        .collect_vec();

    let result1 = ips
        .iter()
        .filter(|(hypernets, supernets)| !has_abba(&hypernets) && has_abba(&supernets))
        .count();

    let result2 = ips
        .iter()
        .filter(|(hypernets, supernets)| {
            iproduct!(get_aba(&hypernets), get_aba(&supernets))
                .any(|(bab, aba)| aba[0] == bab[1] && aba[1] == bab[0])
        })
        .count();

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
