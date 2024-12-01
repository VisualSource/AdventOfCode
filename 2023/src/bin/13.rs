use itertools::Itertools;

advent::solution!(13);

#[must_use]
pub fn part_one(input: &str) -> Option<usize> {
    let mut data: Vec<Vec<&str>> = Vec::new();

    let mut current = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            data.push(current.clone());
            current.clear();
            continue;
        }

        current.push(line);
    }

    if !current.is_empty() {
        data.push(current);
    }

    println!("{:#?}", data);
    let mut output = 0;

    for i in data {
        let width = i.first().expect("Failed to get first").len();
        let height = i.len();

        if width > height {
            let mut row_reflec = None;
            for idx in 0..width {
                let a = i.get(idx).expect("Failed to get index.");
                if let Some(b) = i.get(idx + 1) {
                    if a == b {
                        row_reflec = Some((idx + 1) * 100);
                        break;
                    }
                }
            }

            if let Some(value) = row_reflec {
                println!(":{}", value);
                output += value;
            }
        } else {
            let mut col_reflec = None;

            for idx in 0..width {
                let a: Vec<&str> = i
                    .iter()
                    .map(|x| x.get(idx..=idx).expect("Failed to get index"))
                    .collect();
                let b: Vec<&str> = i
                    .iter()
                    .map(|x| x.get((idx + 1)..=(idx + 1)).expect("Failed to get index"))
                    .collect();

                if a == b {
                    col_reflec = Some(idx + 1);
                    break;
                }
            }

            if let Some(value) = col_reflec {
                println!(":{}", value);
                output += value;
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
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
