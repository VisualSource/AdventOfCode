use std::{collections::HashMap, iter::Peekable, slice::Iter, str::Lines};

use crate::read_input;

#[derive(Debug, Default)]
struct Range {
    destination_range_start: usize,
    source_range_start: usize,
    range_length: usize,
}

impl Range {
    pub fn in_range(&self, value: &usize) -> bool {
        value >= &self.source_range_start && value <= &(self.source_range_start + self.range_length)
    }
    pub fn transform(&self, value: &usize) -> Option<usize> {
        if !self.in_range(value) {
            return None;
        }

        let old_range = (self.source_range_start + self.range_length) - self.source_range_start;
        let new_range =
            (self.destination_range_start + self.range_length) - self.destination_range_start;

        let new_value = (((value - self.source_range_start) * new_range) / old_range)
            + self.destination_range_start;

        Some(new_value)
    }
}

fn parse_range(line_iter: &mut Peekable<Lines<'_>>) -> Vec<Range> {
    let mut ranges = vec![];
    while let Some(i) = line_iter.next_if(|x| !x.is_empty()) {
        let range: Range = i
            .split(" ")
            .filter_map(|x| x.trim().parse::<usize>().ok())
            .enumerate()
            .fold(Range::default(), |mut acc, (idx, value)| {
                match idx {
                    0 => {
                        acc.destination_range_start = value;
                    }
                    1 => {
                        acc.source_range_start = value;
                    }
                    2 => {
                        acc.range_length = value;
                    }
                    _ => panic!("There should only be three values"),
                }
                acc
            });
        ranges.push(range);
    }

    ranges
}

fn parse_input(
    path: &'static str,
) -> (
    Vec<usize>,
    Vec<Range>,
    Vec<Range>,
    Vec<Range>,
    Vec<Range>,
    Vec<Range>,
    Vec<Range>,
    Vec<Range>,
) {
    let input = read_input(path);

    let mut seed_ids: Vec<usize> = vec![];
    let mut seed_to_soil_map = vec![];
    let mut soil_to_fertilizer = vec![];
    let mut fertilizer_to_water = vec![];
    let mut water_to_light = vec![];
    let mut light_to_temp = vec![];
    let mut temp_to_humidity = vec![];
    let mut humidity_to_location = vec![];

    let mut line_iter = input.lines().into_iter().peekable();

    while let Some(line) = line_iter.next() {
        match line {
            e if e.starts_with("seeds") => {
                seed_ids = e
                    .replace("seeds:", "")
                    .split(" ")
                    .filter_map(|x| x.trim().parse::<usize>().ok())
                    .collect::<Vec<usize>>()
            }
            e if e.starts_with("seed-to-soil") => seed_to_soil_map = parse_range(&mut line_iter),
            e if e.starts_with("soil-to-fertilizer") => {
                soil_to_fertilizer = parse_range(&mut line_iter)
            }
            e if e.starts_with("fertilizer-to-water") => {
                fertilizer_to_water = parse_range(&mut line_iter)
            }
            e if e.starts_with("water-to-light") => water_to_light = parse_range(&mut line_iter),
            e if e.starts_with("light-to-temperature") => {
                light_to_temp = parse_range(&mut line_iter)
            }
            e if e.starts_with("temperature-to-humidity") => {
                temp_to_humidity = parse_range(&mut line_iter)
            }
            e if e.starts_with("humidity-to-location") => {
                humidity_to_location = parse_range(&mut line_iter)
            }
            e if e.is_empty() => {}
            e => panic!("Should not have ended up here '{}'", e.escape_debug()),
        }
    }

    (
        seed_ids,
        seed_to_soil_map,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temp,
        temp_to_humidity,
        humidity_to_location,
    )
}

fn first_transform(mut iter: Iter<'_, Range>, value: &usize) -> Option<usize> {
    while let Some(i) = iter.next() {
        if !i.in_range(&value) {
            continue;
        }

        return i.transform(value);
    }

    None
}

fn part1(path: &'static str) -> usize {
    let (
        seeds,
        seeds_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temp,
        temp_to_humidity,
        humidity_to_location,
    ) = parse_input(path);

    seeds
        .iter()
        .map(|seed| {
            let range = seeds_to_soil.iter();
            first_transform(range, &seed).unwrap_or(*seed)
        })
        .map(|soil| {
            let range = soil_to_fertilizer.iter();
            first_transform(range, &soil).unwrap_or(soil)
        })
        .map(|fertilizer| {
            let range = fertilizer_to_water.iter();
            first_transform(range, &fertilizer).unwrap_or(fertilizer)
        })
        .map(|water| {
            let range = water_to_light.iter();
            first_transform(range, &water).unwrap_or(water)
        })
        .map(|light| {
            let range = light_to_temp.iter();
            first_transform(range, &light).unwrap_or(light)
        })
        .map(|temp| {
            let range = temp_to_humidity.iter();
            first_transform(range, &temp).unwrap_or(temp)
        })
        .map(|humitdity| {
            let range = humidity_to_location.iter();
            first_transform(range, &humitdity).unwrap_or(humitdity)
        })
        .min()
        .unwrap()
}

fn part2(path: &'static str) -> usize {
    let (
        seeds,
        seeds_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temp,
        temp_to_humidity,
        humidity_to_location,
    ) = parse_input(path);

    let mut seen: HashMap<usize, usize> = HashMap::new();

    seeds
        .chunks_exact(2)
        .map(|range| {
            let mut values = vec![];

            for seed in range[0]..(range[0] + range[1]) {
                if seen.contains_key(&seed) {
                    values.push(*seen.get(&seed).unwrap());
                    continue;
                }
                let range = seeds_to_soil.iter();
                let soil = first_transform(range, &seed).unwrap_or(seed);
                let fertilizer = first_transform(soil_to_fertilizer.iter(), &soil).unwrap_or(soil);

                let water =
                    first_transform(fertilizer_to_water.iter(), &fertilizer).unwrap_or(fertilizer);
                let light = first_transform(water_to_light.iter(), &water).unwrap_or(water);
                let temp = first_transform(light_to_temp.iter(), &light).unwrap_or(light);
                let humidity = first_transform(temp_to_humidity.iter(), &temp).unwrap_or(temp);
                let location =
                    first_transform(humidity_to_location.iter(), &humidity).unwrap_or(humidity);

                seen.insert(seed, location);

                values.push(location);
            }
            let min_value = values.iter().min().unwrap();

            min_value.to_owned()
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform() {
        let data = Range {
            destination_range_start: 52,
            source_range_start: 50,
            range_length: 48,
        };

        println!("{:#?}", data.transform(&79))
    }

    #[test]
    fn test_input_parse() {
        let data = parse_input("./data/adv5_test.txt");

        println!("{:#?}", data);
    }

    #[test]
    fn test_one() {
        let output = part1("./data/adv5_test.txt");
        println!("{}", output);
        assert_eq!(output, 35);
    }
    #[test]
    fn test_two() {
        let output = part2("./data/adv5_test.txt");

        assert_eq!(output, 46);
    }

    #[test]
    fn aws_one() {
        let output = part1("./data/adv5.txt");

        println!("{}", output);
    }

    #[test]
    fn aws_two() {
        let output = part2("./data/adv5_test.2txt");

        println!("{}", output);
    }
}
