use aoc::*;

use itertools::Itertools;
use smallvec::SmallVec;

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let sorted_required_fields = ["byr", "ecl", "eyr", "hcl", "hgt", "iyr", "pid"];

    let mut count1 = 0usize;
    let mut count2 = 0usize;

    let iter = input.split("\n\n").filter_map(|passport| {
        let mut fields: SmallVec<[_; 7]> = passport
            .split_ascii_whitespace()
            .filter_map(|x| x.split(':').next_tuple())
            .filter(|&(name, _)| name != "cid")
            .collect();

        if fields.len() != sorted_required_fields.len() {
            return None;
        }

        fields.sort_unstable();

        fields
            .iter()
            .zip(&sorted_required_fields)
            .all(|(&(name, _), &required_name)| name == required_name)
            .then_some(fields)
    });

    for fields in iter {
        let [
            (_, byr),
            (_, ecl),
            (_, eyr),
            (_, hcl),
            (_, hgt),
            (_, iyr),
            (_, pid),
        ] = fields.as_slice().try_into()?;

        let mut check2 = match *hgt.as_bytes() {
            [b'1', h1, h0, b'c', b'm'] => (50..=93).contains(&(10 * (h1 - b'0') + (h0 - b'0'))),
            [h1, h0, b'i', b'n'] => (59..=76).contains(&(10 * (h1 - b'0') + (h0 - b'0'))),
            _ => false,
        };

        check2 = check2 && byr.len() == 4 && (1920..=2002).contains(&byr.parse::<u16>()?);
        check2 = check2 && eyr.len() == 4 && (2020..=2030).contains(&eyr.parse::<u16>()?);
        check2 = check2 && iyr.len() == 4 && (2010..=2020).contains(&iyr.parse::<u16>()?);
        check2 = check2 && pid.len() == 9 && pid.bytes().all(|x| x.is_ascii_digit());

        check2 = check2
            && matches!(
                ecl.as_bytes(),
                b"amb" | b"blu" | b"brn" | b"gry" | b"grn" | b"hzl" | b"oth"
            );

        check2 = check2
            && matches!(hcl.as_bytes(), [b'#', tail @ ..] if tail.len() == 6 && tail.iter().all(|&x| matches!(x, b'0'..=b'9' | b'a'..=b'f')));

        if check2 {
            count2 += 1;
        }
        count1 += 1;
    }

    let result1 = count1;
    let result2 = count2;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
