use std::str::FromStr;

use itertools::Itertools;

advent::solution!(12);

#[derive(Debug, PartialEq, Clone)]
enum State {
    Operational,
    Damaged,
    Unknown,
}

impl FromStr for State {
    type Err = std::fmt::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Self::Operational),
            "#" => Ok(Self::Damaged),
            "?" => Ok(Self::Unknown),
            _ => Err(std::fmt::Error {}),
        }
    }
}

#[derive(Debug)]
struct SpringRow(String, Vec<usize>);
impl SpringRow {
    pub fn new(token: String, num: Vec<usize>) -> Self {
        Self(token, num)
    }

    // 7 -> 11; D
    // 8 -> 0 ;?

    pub fn arrangements(&self) -> usize {
        let mut arrangements = 0;

        let mut data = self.0.clone();

        arrangements
    }
}

impl FromStr for SpringRow {
    type Err = std::fmt::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (spring_tokens, spring_numbs) = s
            .split_whitespace()
            .collect_tuple::<(&str, &str)>()
            .expect("Failed to parse data");
        let spring_number_states = spring_numbs
            .split(",")
            .filter_map(|x| x.trim().parse::<usize>().ok())
            .collect::<Vec<usize>>();

        Ok(SpringRow::new(
            spring_tokens.to_string(),
            spring_number_states,
        ))
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<usize> {
    None
}

#[must_use]
pub fn part_two(input: &str) -> Option<usize> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arrangements() {
        let data = "?#?#?#?#?#?#?#? 1,3,1,6"
            .parse::<SpringRow>()
            .expect("Failed to parse data");

        assert_eq!(data.arrangements(), 1)
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
