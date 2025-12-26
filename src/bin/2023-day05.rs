use aoc::*;

use itertools::Itertools;

fn next_section<'a>(input: &'a str, title: &str) -> Result<(&'a str, &'a str)> {
    input.split(title).next_tuple().value()
}

fn parse_map(map: &str) -> Result<Vec<RangeMap>> {
    map.split_ascii_whitespace()
        .map(|x| Ok(x.parse()?))
        .try_process(|iter| {
            iter.tuples()
                .map(|(destination_start, source_start, length)| RangeMap {
                    source_start,
                    destination_start,
                    length,
                })
                .sorted_unstable_by_key(|map| map.source_start)
                .collect_vec()
        })
}

struct RangeMap {
    source_start: i64,
    destination_start: i64,
    length: i64,
}

struct Garden {
    seeds: Vec<i64>,
    soils: Vec<RangeMap>,
    fertilizers: Vec<RangeMap>,
    waters: Vec<RangeMap>,
    lights: Vec<RangeMap>,
    temperatures: Vec<RangeMap>,
    humidities: Vec<RangeMap>,
    locations: Vec<RangeMap>,
}

impl Garden {
    fn parse(input: &str) -> Result<Self> {
        let input = input.strip_prefix("seeds: ").value()?;

        let (seeds, input) = next_section(input, "seed-to-soil map:")?;
        let (soils, input) = next_section(input, "soil-to-fertilizer map:")?;
        let (fertilizers, input) = next_section(input, "fertilizer-to-water map:")?;
        let (waters, input) = next_section(input, "water-to-light map:")?;
        let (lights, input) = next_section(input, "light-to-temperature map:")?;
        let (temperatures, input) = next_section(input, "temperature-to-humidity map:")?;
        let (humidities, locations) = next_section(input, "humidity-to-location map:")?;

        Ok(Self {
            seeds: (seeds.split_ascii_whitespace().map(|x| x.parse::<i64>())).try_collect()?,
            soils: parse_map(soils)?,
            fertilizers: parse_map(fertilizers)?,
            waters: parse_map(waters)?,
            lights: parse_map(lights)?,
            temperatures: parse_map(temperatures)?,
            humidities: parse_map(humidities)?,
            locations: parse_map(locations)?,
        })
    }

    fn maps(&self) -> [&[RangeMap]; 7] {
        [
            &self.soils,
            &self.fertilizers,
            &self.waters,
            &self.lights,
            &self.temperatures,
            &self.humidities,
            &self.locations,
        ]
    }

    fn compute_lowest_location(&self) -> Result<i64> {
        let mut current_buffer = self.seeds.clone();
        let mut new_buffer = Vec::with_capacity(current_buffer.len());

        for map in self.maps() {
            new_buffer.clear();

            new_buffer.extend(current_buffer.iter().map(|&elem| {
                map.get(map.partition_point(|x| x.source_start + x.length <= elem))
                    .filter(|x| (x.source_start..x.source_start + x.length).contains(&elem))
                    .map(|x| x.destination_start + elem - x.source_start)
                    .unwrap_or(elem)
            }));

            std::mem::swap(&mut new_buffer, &mut current_buffer);
        }

        current_buffer.iter().min().copied().value()
    }

    fn compute_lowest_location_from_range(&self) -> Result<i64> {
        let mut current_buffer = self
            .seeds
            .iter()
            .tuples()
            .map(|(&start, &len)| start..start + len)
            .collect_vec();

        let mut new_buffer = Vec::with_capacity(current_buffer.len());

        for map in self.maps() {
            new_buffer.clear();

            for elem in &current_buffer {
                let idx = map.partition_point(|x| x.source_start + x.length <= elem.start);

                let mut current_range = elem.clone();

                if idx == map.len() {
                    new_buffer.push(current_range);
                    continue;
                }

                for map_elem in &map[idx..] {
                    let start = current_range.start.max(map_elem.source_start);
                    let end = (current_range.end).min(map_elem.source_start + map_elem.length);
                    let intersection = start..end;

                    if intersection.is_empty() {
                        new_buffer.push(current_range);
                        break;
                    }

                    let diff = map_elem.destination_start - map_elem.source_start;
                    new_buffer.push(intersection.start + diff..intersection.end + diff);

                    if current_range.start < start {
                        new_buffer.push(current_range.start..start);
                    }

                    if end < current_range.end {
                        current_range = end..current_range.end;
                    } else {
                        break;
                    }
                }
            }

            new_buffer.sort_unstable_by_key(|x| x.start);

            current_buffer.clear();

            current_buffer.extend(new_buffer.iter().cloned().coalesce(|x1, x2| {
                if x1.end >= x2.start {
                    Ok(x1.start..x2.end)
                } else {
                    Err((x1, x2))
                }
            }));
        }

        current_buffer.iter().map(|x| x.start).min().value()
    }
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let garden = Garden::parse(&input)?;

    let result1 = garden.compute_lowest_location()?;
    let result2 = garden.compute_lowest_location_from_range()?;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
