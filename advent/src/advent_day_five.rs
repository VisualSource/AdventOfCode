use range_ext::intersect::{Intersect, IntersectionExt};
use std::{
    collections::{BTreeSet, HashMap, VecDeque},
    iter::Peekable,
    slice::Iter,
    str::Lines,
};

use crate::read_input;

#[derive(Debug, Default)]
struct Range {
    pub destination_range_start: usize,
    pub source_range_start: usize,
    pub range_length: usize,
}

impl Range {
    pub fn new(d: usize, s: usize, l: usize) -> Self {
        Self {
            destination_range_start: d,
            source_range_start: s,
            range_length: l,
        }
    }

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

fn convert_range(source: &Vec<Range>, start: usize, len: usize) -> Vec<(usize, usize)> {
    let mut slices = BTreeSet::new();

    let range_max = start + len;

    for entry in source {
        let source_max = entry.source_range_start + entry.range_length;

        if range_max < entry.source_range_start || start > source_max {
            continue;
        }

        if entry.source_range_start > start {
            slices.insert(entry.source_range_start);
        }

        if source_max < range_max {
            slices.insert(source_max);
        }
    }

    slices.insert(range_max);

    let mut output = Vec::new();
    let mut current = start;

    for position in slices {
        output.push((
            source
                .iter()
                .map(|entry| entry.transform(&current))
                .find_map(|e| e)
                .unwrap_or(current),
            position - current,
        ));

        current = position;
    }

    output
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

// @see https://github.com/andypymont/advent2023-rust/blob/main/src/bin/05.rs
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

    let mut current = seeds
        .chunks_exact(2)
        .map(|range| (range[0], range[1]))
        .collect::<Vec<(usize, usize)>>();
    let mut future = Vec::new();

    for map in vec![
        seeds_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temp,
        temp_to_humidity,
        humidity_to_location,
    ] {
        for range in current {
            future.extend(convert_range(&map, range.0, range.1))
        }

        current = future;
        future = Vec::new();
    }

    current
        .iter()
        .map(|range| range.0)
        .min()
        .expect("Failed to get min")
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
        let output = part2("./data/adv5.txt");

        println!("{}", output);
    }
}
