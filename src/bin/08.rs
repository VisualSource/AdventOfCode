use std::{collections::HashMap, str::FromStr};

use itertools::Itertools;

advent::solution!(8);

#[derive(Debug)]
enum Action {
    R,
    L,
}

impl FromStr for Action {
    type Err = std::fmt::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Self::L),
            "R" => Ok(Self::R),
            _ => Err(std::fmt::Error {}),
        }
    }
}

#[derive(Debug)]
struct Instruction(String, String);

impl Instruction {
    pub fn get_path(&self, action: &Action) -> String {
        match action {
            Action::R => self.1.to_owned(),
            Action::L => self.0.to_owned(),
        }
    }
}

impl FromStr for Instruction {
    type Err = std::fmt::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((left, right)) = s
            .replace("(", "")
            .replace(")", "")
            .split(",")
            .collect_tuple::<(&str, &str)>()
        {
            return Ok(Self(left.trim().to_string(), right.trim().to_string()));
        }

        return Err(std::fmt::Error {});
    }
}

fn parse_input(input: &str) -> (Vec<Action>, HashMap<String, Instruction>) {
    let mut actions: Vec<Action> = Vec::new();
    let mut map: HashMap<String, Instruction> = std::collections::HashMap::new();

    let mut parse_actions = true;
    for line in input.lines() {
        if parse_actions {
            if line.is_empty() {
                parse_actions = false;
                continue;
            }

            actions.extend(line.split("").filter_map(|x| x.parse::<Action>().ok()))
        } else {
            if let Some((key, values)) = line.split("=").collect_tuple::<(&str, &str)>() {
                map.insert(
                    key.trim().to_string(),
                    values
                        .parse::<Instruction>()
                        .expect("Failed to parse input."),
                );
            }
        }
    }

    (actions, map)
}

fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

fn gcd(a: usize, b: usize) -> usize {
    let mut c = a;
    let mut d = b;

    while c != d {
        if c > d {
            c -= d;
        } else if d > c {
            d -= c;
        } else if c == d {
            break;
        }
    }

    c
}

#[must_use]
pub fn part_one(input: &str) -> Option<usize> {
    let (actions, map) = parse_input(input);

    let mut actions_iter = actions.iter().cycle();

    let mut current = "AAA".to_string();
    let mut steps = 0;

    while let Some(value) = actions_iter.next() {
        let instruction = map.get(&current).expect("Failed to get next action");

        current = instruction.get_path(value);

        steps += 1;

        if current.as_str() == "ZZZ" {
            break;
        }
    }

    Some(steps)
}

#[must_use]
pub fn part_two(input: &str) -> Option<usize> {
    let (actions, map) = parse_input(input);

    let mut actions_iter = actions.iter().cycle();
    let mut currnet_positions = map
        .keys()
        .filter_map(|x| {
            if x.ends_with("A") {
                return Some(x.to_owned());
            }
            None
        })
        .collect::<Vec<String>>();
    let mut end_positions = map
        .keys()
        .filter_map(|x| {
            if x.ends_with("Z") {
                return Some(x.to_owned());
            }

            None
        })
        .collect::<Vec<String>>();

    let mut values = vec![];

    let mut steps: usize = 0;
    while let Some(value) = actions_iter.next() {
        for current in currnet_positions.iter_mut() {
            if !current.ends_with("Z") {
                let instruction = map.get(current).expect("Failed to get next action");
                *current = instruction.get_path(value);
            }
        }

        steps += 1;

        currnet_positions.iter().for_each(|x| {
            if end_positions.contains(x) {
                let idx = end_positions
                    .iter()
                    .position(|e| e == x)
                    .expect("Failed to get index");
                end_positions.remove(idx);

                values.push(steps);
            }
        });

        if end_positions.is_empty() {
            break;
        }
    }

    if values.is_empty() {
        None
    } else {
        // lcm of values
        values
            .iter()
            .map(|x| x.to_owned())
            .reduce(|acc, x| lcm(acc, x))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_ext() {
        let result = part_one(&advent::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent::template::read_file_part("examples", DAY, 3));
        assert_eq!(result, Some(6));
    }
}
