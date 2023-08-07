use aoc::*;

use itertools::Itertools;
use smallvec::SmallVec;

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let mut questions_union = SmallVec::<[_; 26]>::new();
    let mut questions_intersection = SmallVec::<[_; 26]>::new();
    let mut buffer = SmallVec::<[_; 26]>::new();

    let mut count1 = 0usize;
    let mut count2 = 0usize;

    for group in input.split("\n\n") {
        let mut lines = group.lines();
        let first_line = lines.next().map(|line| line.as_bytes()).unwrap_or_default();

        questions_union.clear();
        questions_union.extend(first_line.iter().copied());

        questions_intersection.clear();
        questions_intersection.extend(first_line.iter().copied());
        questions_intersection.sort_unstable();
        questions_intersection.dedup();

        for line in lines {
            questions_union.extend(line.bytes());

            buffer.clear();
            buffer.extend(line.bytes());
            buffer.sort_unstable();
            buffer.dedup();

            let mut buffer_iter = buffer.iter();
            questions_intersection.retain(|&mut x| buffer_iter.take_while_ref(|&&item| item <= x).any(|&item| item == x));
        }

        questions_union.sort_unstable();
        questions_union.dedup();

        count1 += questions_union.len();
        count2 += questions_intersection.len();
    }

    let result1 = count1;
    let result2 = count2;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
