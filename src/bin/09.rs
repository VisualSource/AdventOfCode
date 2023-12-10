use std::str::FromStr;

use itertools::Itertools;

advent::solution!(9);

#[derive(Debug)]
struct NumberSet(Vec<isize>);

impl NumberSet {
    pub fn first(&self, value: isize) -> isize {
        let input = self.0.first().expect("Failed to get first");
        input - value
    }

    pub fn last(&self, value: isize) -> isize {
        let input = self.0.last().expect("Failed get last");

        value + input
    }

    pub fn is_zero(&self) -> bool {
        self.0.iter().all(|x| *x == 0)
    }
    pub fn extrapolate(&self) -> NumberSet {
        let mut output = Vec::new();

        for idx in 0..self.0.len() {
            let a = self.0.get(idx).expect("Failed to get index.");
            if let Some(b) = self.0.get(idx + 1) {
                output.push(b - a);
            } else {
                break;
            }
        }

        NumberSet(output)
    }
}

impl FromStr for NumberSet {
    type Err = std::fmt::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.split_whitespace()
                .filter_map(|e| e.parse::<isize>().ok())
                .collect(),
        ))
    }
}
fn parse_input(input: &str) -> Vec<Vec<NumberSet>> {
    let sets = input
        .lines()
        .filter_map(|e| e.parse::<NumberSet>().ok())
        .collect::<Vec<NumberSet>>();

    let mut data: Vec<Vec<NumberSet>> = Vec::new();

    for x in sets {
        let mut set = vec![x];

        let mut last = set.last().expect("Failed to get last");

        while !last.is_zero() {
            set.push(last.extrapolate());

            last = set.last().expect("Failed to get last");
        }

        data.push(set);
    }

    data
}

#[must_use]
pub fn part_one(input: &str) -> Option<isize> {
    let data = parse_input(input);

    Some(
        data.iter()
            .map(|x| x.iter().fold(0, |acc, e| e.last(acc)))
            .fold(0, |acc, x| acc + x),
    )
}

#[must_use]
pub fn part_two(input: &str) -> Option<isize> {
    let data = parse_input(input);

    let mut output = Vec::new();

    for x in data {
        let mut prev = 0;
        for i in x.iter().rev() {
            prev = i.first(prev);
        }

        output.push(prev);
    }

    Some(output.iter().fold(0, |acc, e| acc + e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two_a() {
        let result = part_two(&advent::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(5));
    }
}
