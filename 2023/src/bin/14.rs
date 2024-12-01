use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

advent::solution!(14);

fn transform_north(data: &mut Vec<String>, tracker: &mut HashMap<usize, VecDeque<usize>>) {
    let data_len = data.len();

    fn set_line(
        col: usize,
        row: usize,
        data: &mut Vec<String>,
        tracker: &mut HashMap<usize, VecDeque<usize>>,
    ) {
        if let Some(failling) = tracker.get_mut(&col) {
            while !failling.is_empty() {
                let item = failling.pop_front();
                if let Some(offset) = item {
                    data.get_mut(row + offset)
                        .expect("Failed to get row")
                        .replace_range(col..=col, "O");
                }
            }
        }
    }

    for idx in 0..data_len {
        let row = (data_len - 1) - idx;
        let line = data.get(row).expect("Failed to get line.").clone();

        for (col, char) in line.chars().enumerate() {
            match char {
                'O' => {
                    let is_falling = if let Some(next_idx) = row.checked_sub(1) {
                        let next_line = data.get(next_idx).expect("Failed to get next line");
                        if let Some(next_char) = next_line.chars().nth(col) {
                            next_char != '#'
                        } else {
                            false
                        }
                    } else {
                        false
                    };

                    if is_falling {
                        if !tracker.contains_key(&col) {
                            tracker.insert(col, VecDeque::new());
                        }

                        let falling = tracker.get_mut(&col).expect("Failed to get col");
                        falling.push_back(falling.len());
                        data.get_mut(row)
                            .expect("Failed to get row")
                            .replace_range(col..=col, ".");
                    } else {
                        set_line(col, row + 1, data, tracker)
                    }
                }
                '#' => set_line(col, row + 1, data, tracker),
                '.' => {
                    if idx + 1 >= data_len {
                        set_line(col, row, data, tracker);
                    }
                }
                _ => unreachable!("Reached invalid token."),
            }
        }
    }
}

fn transform_west(data: &mut Vec<String>, tracker: &mut HashMap<usize, VecDeque<usize>>) {
    let data_len = data.len();

    fn set_line(
        col: usize,
        idx: usize,
        data: &mut Vec<String>,
        tracker: &mut HashMap<usize, VecDeque<usize>>,
    ) {
        if let Some(failling) = tracker.get_mut(&idx) {
            let line = data.get_mut(idx).expect("Failed to get line.");

            while !failling.is_empty() {
                let item = failling.pop_front();
                if let Some(offset) = item {
                    let idx = col + offset;
                    line.replace_range(idx..=idx, "O");
                }
            }
        }
    }

    for idx in 0..data_len {
        let line = data.get(idx).expect("Failed to get line.").clone();

        let chars = line.chars().collect::<Vec<char>>();

        let mut col = line.len() - 1;
        loop {
            let char = chars.get(col).expect("Failed to get char");

            match char {
                'O' => {
                    let does_slide = if let Some(next_idx) = col.checked_sub(1) {
                        let next_char = chars.get(next_idx).expect("Failed to get next char");
                        next_char != &'#'
                    } else {
                        false
                    };

                    if does_slide {
                        if !tracker.contains_key(&idx) {
                            tracker.insert(idx, VecDeque::new());
                        }

                        let item = tracker.get_mut(&idx).expect("Failed to get col");
                        item.push_back(item.len());

                        data.get_mut(idx)
                            .expect("Failed to get row")
                            .replace_range(col..=col, ".");
                    } else {
                        set_line(col + 1, idx, data, tracker)
                    }
                }
                '#' => set_line(col + 1, idx, data, tracker),
                '.' => {
                    if col.checked_sub(1).is_none() {
                        set_line(col, idx, data, tracker)
                    }
                }
                _ => unreachable!("Invaild char"),
            }

            if let Some(value) = col.checked_sub(1) {
                col = value;
            } else {
                break;
            }
        }
    }
}

fn transform_south(data: &mut Vec<String>, tracker: &mut HashMap<usize, VecDeque<usize>>) {
    let data_len = data.len();

    fn set_line(
        col: usize,
        idx: usize,
        data: &mut Vec<String>,
        tracker: &mut HashMap<usize, VecDeque<usize>>,
    ) {
        if let Some(failling) = tracker.get_mut(&col) {
            while !failling.is_empty() {
                let item = failling.pop_front();
                if let Some(offset) = item {
                    let item = idx - offset;

                    data.get_mut(item)
                        .expect("Failed to get row")
                        .replace_range(col..=col, "O");
                }
            }
        }
    }

    for row in 0..data_len {
        let line = data.get(row).expect("Failed to get line.").clone();

        for (col, char) in line.chars().enumerate() {
            match char {
                'O' => {
                    let is_falling = if row + 1 < data_len {
                        let next_line = data.get(row + 1).expect("Failed to get next line");
                        if let Some(next_char) = next_line.chars().nth(col) {
                            next_char != '#'
                        } else {
                            false
                        }
                    } else {
                        false
                    };

                    if is_falling {
                        if !tracker.contains_key(&col) {
                            tracker.insert(col, VecDeque::new());
                        }

                        let falling = tracker.get_mut(&col).expect("Failed to get col");
                        falling.push_back(falling.len());
                        data.get_mut(row)
                            .expect("Failed to get row")
                            .replace_range(col..=col, ".");
                    } else {
                        if let Some(value) = row.checked_sub(1) {
                            set_line(col, value, data, tracker)
                        }
                    }
                }
                '#' => {
                    if let Some(value) = row.checked_sub(1) {
                        set_line(col, value, data, tracker)
                    }
                }
                '.' => {
                    if row + 1 >= data_len {
                        set_line(col, row, data, tracker)
                    }
                }
                _ => unreachable!("Reached invalid token."),
            }
        }
    }
}

fn transform_east(data: &mut Vec<String>, tracker: &mut HashMap<usize, VecDeque<usize>>) {
    let data_len = data.len();

    fn set_line(
        col: usize,
        row: usize,
        data: &mut Vec<String>,
        tracker: &mut HashMap<usize, VecDeque<usize>>,
    ) {
        if let Some(failling) = tracker.get_mut(&row) {
            let line = data.get_mut(row).expect("Failed to get line.");

            while !failling.is_empty() {
                let item = failling.pop_front();
                if let Some(offset) = item {
                    let idx = col - offset;
                    line.replace_range(idx..=idx, "O");
                }
            }
        }
    }

    for row in 0..data_len {
        let line = data.get(row).expect("Failed to get line.").clone();

        let mut chars = line.chars().enumerate().into_iter().peekable();

        while let Some((col, char)) = chars.next() {
            match char {
                'O' => {
                    let does_slide = if let Some(next_char) = chars.peek() {
                        next_char.1 != '#'
                    } else {
                        false
                    };

                    if does_slide {
                        if !tracker.contains_key(&row) {
                            tracker.insert(row, VecDeque::new());
                        }

                        let item = tracker.get_mut(&row).expect("Failed to get col");
                        item.push_back(item.len());

                        data.get_mut(row)
                            .expect("Failed to get row")
                            .replace_range(col..=col, ".");
                    } else {
                        if let Some(value) = col.checked_sub(1) {
                            set_line(value, row, data, tracker);
                        }
                    }
                }
                '.' => {
                    if chars.peek().is_none() {
                        set_line(col, row, data, tracker);
                    }
                }
                '#' => {
                    if let Some(value) = col.checked_sub(1) {
                        set_line(value, row, data, tracker)
                    }
                }
                _ => unreachable!("Should not be here."),
            }
        }
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<usize> {
    let mut data = input
        .lines()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    let data_len = data.len();

    let mut tracker_north = HashMap::<usize, VecDeque<usize>>::new();
    transform_north(&mut data, &mut tracker_north);

    let output = data
        .iter()
        .enumerate()
        .map(|(idx, line)| {
            let mutliplayer = data_len - idx;

            let count = line.chars().filter(|x| x == &'O').count();

            count * mutliplayer
        })
        .fold(0, |acc, e| acc + e);

    Some(output)
}

#[must_use]
pub fn part_two(input: &str) -> Option<usize> {
    part_two_core(input, 1_000_000_000)
}

pub fn part_two_core(input: &str, cycles: usize) -> Option<usize> {
    let mut data = input
        .lines()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    let mut tracker_north = HashMap::<usize, VecDeque<usize>>::new();
    let mut tracker_west = HashMap::<usize, VecDeque<usize>>::new();
    let mut tracker_south = HashMap::<usize, VecDeque<usize>>::new();
    let mut tracker_east = HashMap::<usize, VecDeque<usize>>::new();

    for _ in 0..cycles {
        transform_north(&mut data, &mut tracker_north);
        transform_west(&mut data, &mut tracker_west);
        transform_south(&mut data, &mut tracker_south);
        transform_east(&mut data, &mut tracker_east);
    }

    let output = data
        .iter()
        .rev()
        .enumerate()
        .map(|(idx, line)| {
            let count = line.chars().filter(|x| x == &'O').count();

            count * (idx + 1)
        })
        .fold(0, |acc, e| acc + e);

    Some(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_west() {
        let input = advent::template::read_file("examples", DAY);

        let mut data = input
            .lines()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        let mut tracker = HashMap::<usize, VecDeque<usize>>::new();
        transform_west(&mut data, &mut tracker);

        println!("{:#?}", data);
    }

    #[test]
    fn test_south() {
        let input = advent::template::read_file("examples", DAY);

        let mut data = input
            .lines()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        let mut tracker = HashMap::<usize, VecDeque<usize>>::new();
        transform_south(&mut data, &mut tracker);

        println!("{:#?}", data);
    }

    #[test]
    fn test_east() {
        let input = advent::template::read_file("examples", DAY);

        let mut data = input
            .lines()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        let mut tracker = HashMap::<usize, VecDeque<usize>>::new();
        transform_east(&mut data, &mut tracker);

        println!("{:#?}", data);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two_core(&advent::template::read_file("examples", DAY), 3);
        assert_eq!(result, Some(69));
    }
}
