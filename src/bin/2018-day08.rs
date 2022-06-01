use aoc::*;

use itertools::Itertools;

fn parse_tree(mut data: &[usize]) -> Result<(usize, usize, &[usize])> {
    let (header, remaining) = data.split_at(2);
    let [children_count, metadata_size] = <[_; 2]>::try_from(header)?;
    data = remaining;

    let mut metadata_sum = 0;
    let mut values = Vec::with_capacity(children_count);

    for _ in 0..children_count {
        let (child_metadata_sum, value, remaining) = parse_tree(data)?;
        metadata_sum += child_metadata_sum;
        values.push(value);
        data = remaining;
    }

    let (current_metadata, remaining) = data.split_at(metadata_size);
    let current_metadata_sum = current_metadata.iter().sum::<usize>();
    metadata_sum += current_metadata_sum;

    let current_value = if children_count == 0 { current_metadata_sum } else { current_metadata.iter().flat_map(|&index| values.get(index - 1)).sum() };

    Ok((metadata_sum, current_value, remaining))
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let data: Vec<usize> = input.split_ascii_whitespace().map(|x| x.parse()).try_collect()?;

    let (result1, result2, _) = parse_tree(&data)?;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
