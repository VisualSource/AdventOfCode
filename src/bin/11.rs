use std::collections::HashSet;

use itertools::Itertools;

advent::solution!(11);

#[must_use]
pub fn part_one(input: &str) -> Option<usize> {
    let mut gallexy = input
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut empty_cols = vec![];
    let mut empty_rows = vec![];

    let len = gallexy.len();

    for idx in 0..len {
        let col = gallexy
            .iter()
            .all(|x| x.get(idx).expect("Failed to get idex.") == &'.');

        if col {
            empty_cols.push(idx);
        }

        let row = gallexy
            .get(idx)
            .expect("Failed to get row")
            .iter()
            .all_equal();

        if row {
            empty_rows.push(idx);
        }
    }

    for (idx, row) in empty_rows.iter().enumerate() {
        gallexy.insert(idx + row, vec!['.'].repeat(len));
    }

    for (idx, col) in empty_cols.iter().enumerate() {
        gallexy.iter_mut().for_each(|x| {
            x.insert(idx + col, '.');
        });
    }
    let mut gallexy_count = 0;
    let gallexy_indexs: Vec<(usize, usize, usize)> = gallexy
        .iter()
        .enumerate()
        .map(|(idx_y, row)| {
            let mut items = vec![];

            for (idx_x, e) in row.iter().enumerate() {
                if e != &'#' {
                    continue;
                }
                gallexy_count += 1;
                items.push((idx_x, idx_y, gallexy_count))
            }

            items
        })
        .flatten()
        .collect();

    //println!("Gallexys: {}", gallexy_count);

    /*gallexy.iter().for_each(|x| {
        x.iter().for_each(|e| print!("{e}"));
        println!();
    });*/

    let mut seen = HashSet::<String>::new();
    let mut output = 0;
    for idx in 0..gallexy_count {
        let (x, y, id) = gallexy_indexs
            .get(idx)
            .expect("Failed to get current gallexy.");

        //println!("Current idx: {} Item: ({},{},{})", idx, x, y, id);

        for row in &gallexy_indexs {
            if &row.2 == id {
                // can't link to self.
                continue;
            }
            let mut keys = vec![*id, row.2];
            keys.sort();

            // 0-1;1-0
            let key = format!("{0}-{1};{1}-{0}", keys[0], keys[1]);

            if !seen.contains(&key) {
                let x_a = row.0.checked_sub(*x).unwrap_or_else(|| x - row.0);
                let y_a = row.1.checked_sub(*y).unwrap_or_else(|| y - row.1);

                let min = x_a + y_a;

                if min > 0 {
                    /*println!(
                        "{} : ({},{}) - ({},{}) = ({} {}) => {}",
                        key, row.0, row.1, x, y, x_a, y_a, min
                    );*/
                    output += min;
                    seen.insert(key);
                }
            }
        }
    }

    Some(output)
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
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
