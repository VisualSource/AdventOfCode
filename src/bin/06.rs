use std::ops::Sub;

advent::solution!(6);

#[derive(Debug)]
struct Race {
    time: usize,
    distance: usize,
}

impl Race {
    fn calc_options(&self) -> usize {
        let mut options = 0;

        for int in 0..=self.time {
            //1: 1 - 7 = 6 => 1 * 6 = 6
            //2: 2 - 7 = 5 => 5 * 2 = 10,
            let dis = int * self.time.checked_sub(int).unwrap_or(0);
            if dis > self.distance {
                options += 1;
            }
        }

        options
    }
}

fn parse_input(input: &str) -> Vec<Race> {
    let mut times: Vec<usize> = Vec::new();
    let mut distances: Vec<usize> = Vec::new();

    for line in input.lines() {
        match line {
            e if e.starts_with("Time:") => {
                times = e
                    .replace("Time:", "")
                    .split_whitespace()
                    .filter_map(|x| x.parse::<usize>().ok())
                    .collect();
            }
            e if e.starts_with("Distance:") => {
                distances = e
                    .replace("Distance:", "")
                    .split_whitespace()
                    .filter_map(|x| x.parse::<usize>().ok())
                    .collect();
            }
            _ => panic!("Unknown input."),
        }
    }

    let mut races = vec![];

    for i in 0..times.len() {
        let time = *times.get(i).expect("Failed to get input.");
        let distance = *distances.get(i).expect("Failed to get input.");

        races.push(Race { time, distance });
    }

    races
}

#[must_use]
pub fn part_one(input: &str) -> Option<usize> {
    let data = parse_input(input);

    Some(
        data.iter()
            .map(|x| x.calc_options())
            .fold(1, |acc, e| acc * e),
    )
}

#[must_use]
pub fn part_two(input: &str) -> Option<usize> {
    let mut lines = input.lines();

    let time = lines
        .next()
        .expect("Failed to read input.")
        .replace("Time:", "")
        .replace(" ", "")
        .parse::<usize>()
        .expect("Failed to parse input");
    let distance = lines
        .next()
        .expect("Failed to read input.")
        .replace("Distance:", "")
        .replace(" ", "")
        .parse::<usize>()
        .expect("Failed to parse input.");

    let race = Race { time, distance };

    Some(race.calc_options())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn option_clac() {
        let race = Race {
            time: 7,
            distance: 9,
        };
        assert_eq!(race.calc_options(), 4);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
