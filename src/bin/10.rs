use std::collections::{HashSet, VecDeque};
use std::str::FromStr;

use itertools::Itertools;

advent::solution!(10);

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    West,
    North,
    East,
    South,
}

impl Direction {
    pub fn transform(&self, value: usize) -> usize {
        match self {
            Self::North | Self::East => value - 1,
            Self::West | Self::South => value + 1,
        }
    }
}

#[derive(Debug, PartialEq)]
enum Tile {
    VerticalPipe,
    HorizontalPipe,
    BendNorthEast,
    BendNorthWest,
    BendSouthWest,
    BendSouthEast,
    Ground,
    Start,
}

impl Tile {
    pub fn transform(&self, item: &(usize, usize, Direction)) -> Option<(usize, usize, Direction)> {
        match self {
            Self::Start | Self::Ground => None,
            Self::VerticalPipe => Some((item.0, item.2.transform(item.1), item.2)),
            Self::HorizontalPipe => Some((item.2.transform(item.0), item.1, item.2)),
            Self::BendNorthEast => {
                if item.2 == Direction::West || item.2 == Direction::North {
                    return None;
                }

                let result = if item.2 == Direction::South {
                    (item.0 + 1, item.1, Direction::West)
                } else {
                    (item.0, item.1 - 1, Direction::North)
                };

                Some(result)
            }
            Self::BendNorthWest => {
                if item.2 == Direction::North || item.2 == Direction::East {
                    return None;
                }

                let result = if item.2 == Direction::West {
                    (item.0, item.1 - 1, Direction::North)
                } else {
                    (item.0 - 1, item.1, Direction::East)
                };
                Some(result)
            }
            Self::BendSouthWest => {
                if item.2 == Direction::East || item.2 == Direction::South {
                    return None;
                }

                let result = if item.2 == Direction::West {
                    (item.0, item.1 + 1, Direction::South)
                } else {
                    (item.0 - 1, item.1, Direction::East)
                };
                Some(result)
            }
            Self::BendSouthEast => {
                if item.2 == Direction::South || item.2 == Direction::West {
                    return None;
                }

                let result = if item.2 == Direction::North {
                    (item.0 + 1, item.1, Direction::West)
                } else {
                    (item.0, item.1 + 1, Direction::South)
                };
                Some(result)
            }
        }
    }
}

impl FromStr for Tile {
    type Err = std::fmt::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "|" => Ok(Self::VerticalPipe),
            "-" => Ok(Self::HorizontalPipe),
            "L" => Ok(Self::BendNorthEast),
            "J" => Ok(Self::BendNorthWest),
            "7" => Ok(Self::BendSouthWest),
            "F" => Ok(Self::BendSouthEast),
            "." => Ok(Self::Ground),
            "S" => Ok(Self::Start),
            _ => Err(std::fmt::Error {}),
        }
    }
}

fn get_connections(
    graph: &Vec<Vec<Tile>>,
    start: &(usize, usize),
) -> Vec<(usize, usize, Direction)> {
    let mut connections = vec![];

    {
        // left
        if let Some(x) = start.0.checked_sub(1) {
            let y = start.1;

            let tile = graph
                .get(y)
                .expect("Failed to get value")
                .get(x)
                .expect("Failed to get value.");

            if tile == &Tile::BendSouthEast
                || tile == &Tile::BendNorthEast
                || tile == &Tile::HorizontalPipe
            {
                connections.push((x, y, Direction::East));
            }
        }
    }

    {
        // right
        let x = start.0 + 1;
        let y = start.1;

        let tile = graph
            .get(y)
            .expect("Failed to get value")
            .get(x)
            .expect("Failed to get value.");

        if tile == &Tile::BendNorthWest
            || tile == &Tile::BendSouthWest
            || tile == &Tile::HorizontalPipe
        {
            connections.push((x, y, Direction::West));
        }
    }

    {
        // up
        let x = start.0;
        if let Some(y) = start.1.checked_sub(1) {
            let tile = graph
                .get(y)
                .expect("Failed to get value")
                .get(x)
                .expect("Failed to get value.");

            if tile == &Tile::VerticalPipe
                || tile == &Tile::BendSouthWest
                || tile == &Tile::BendSouthEast
            {
                connections.push((x, y, Direction::North));
            }
        }
    }

    {
        // down
        let x = start.0;
        let y = start.1 + 1;

        let tile = graph
            .get(y)
            .expect("Failed to get value")
            .get(x)
            .expect("Failed to get value.");

        if tile == &Tile::VerticalPipe
            || tile == &Tile::BendNorthEast
            || tile == &Tile::BendNorthWest
        {
            connections.push((x, y, Direction::South));
        }
    }

    connections
}

#[must_use]
pub fn part_one(input: &str) -> Option<usize> {
    let mut grid = Vec::new();

    for line in input.lines() {
        grid.push(
            line.split("")
                .filter_map(|x| x.parse::<Tile>().ok())
                .collect::<Vec<Tile>>(),
        );
    }

    let mut start_position = (0, 0);

    'outer: for (idx_y, i) in grid.iter().enumerate() {
        for (idx_x, j) in i.iter().enumerate() {
            if j == &Tile::Start {
                start_position = (idx_x, idx_y);
                break 'outer;
            }
        }
    }

    let connections = get_connections(&grid, &start_position);

    if connections.is_empty() {
        return None;
    }

    let mut queue = VecDeque::<Vec<(usize, usize, Direction)>>::new();
    queue.push_back(connections);

    let mut steps = 1;
    while !queue.is_empty() {
        let items = queue.pop_front().expect("Failed to get item.");

        let mut next = Vec::new();

        for item in items {
            let tile = grid.get(item.1).expect("").get(item.0).expect("");
            if let Some(output) = tile.transform(&item) {
                next.push(output);
            }
        }

        steps += 1;

        if next.iter().map(|x| (x.0, x.1)).all_equal() {
            break;
        }

        if !next.is_empty() {
            queue.push_back(next);
        }
    }

    Some(steps)
}

#[must_use]
pub fn part_two(input: &str) -> Option<usize> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_one_a() {
        let result = part_one(&advent::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two_a() {
        let result = part_two(&advent::template::read_file_part("examples", DAY, 3));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two_b() {
        let result = part_two(&advent::template::read_file_part("examples", DAY, 4));
        assert_eq!(result, Some(8));
    }
}
