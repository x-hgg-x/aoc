use regex::{CaptureLocations, Regex};

use std::fs;

fn get_location_data<'a>(input: &'a str, locations: &CaptureLocations, i: usize) -> &'a str {
    locations.get(i).map(|(start, end)| &input[start..end]).unwrap()
}

fn file_length_v1(data: &str, re: &Regex, locations: &mut CaptureLocations) -> usize {
    let mut size = 0;
    let mut offset = 0;
    while let Some(m) = re.captures_read_at(locations, data, offset) {
        let chunk_size: usize = get_location_data(data, locations, 1).parse().unwrap();
        let repetitions: usize = get_location_data(data, locations, 2).parse().unwrap();

        size += m.start() - offset + chunk_size * repetitions;
        offset = m.end() + chunk_size;
    }
    size += data.len() - offset;

    size
}

fn file_length_v2(data: &str, re: &Regex, locations: &mut CaptureLocations) -> usize {
    let mut size = 0;
    let mut offset = 0;
    while let Some(m) = re.captures_read_at(locations, data, offset) {
        let chunk_size: usize = get_location_data(data, locations, 1).parse().unwrap();
        let repetitions: usize = get_location_data(data, locations, 2).parse().unwrap();

        size += m.start() - offset;
        size += file_length_v2(&data[m.end()..m.end() + chunk_size], re, locations) * repetitions;

        offset = m.end() + chunk_size;
    }
    size += data.len() - offset;

    size
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("inputs/2016-day09.txt")?;
    let input = input.trim();

    let re = Regex::new(r#"\((\d+)x(\d+)\)"#)?;

    let mut locations = re.capture_locations();

    let result1 = file_length_v1(input, &re, &mut locations);
    let result2 = file_length_v2(input, &re, &mut locations);

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
