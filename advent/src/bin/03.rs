use std::{collections::HashSet, usize};

advent::solution!(3);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Cord {
    pub col: usize,
    pub row: usize,
}

#[derive(Debug)]
struct Number {
    pub value: usize,
    start_row: usize,
    start_col: usize,
    end_row: usize,
    end_col: usize,
}

impl Number {
    pub fn new(
        value: usize,
        start_col: usize,
        start_row: usize,
        end_col: usize,
        end_row: usize,
    ) -> Self {
        Self {
            value,
            start_col,
            end_col,
            start_row,
            end_row,
        }
    }

    pub fn ranges(&self, symbles: &HashSet<Cord>) -> bool {
        let upper_col = self.start_col.checked_sub(1).unwrap_or(self.start_col);
        let lower_col = self.end_col + 1;

        let upper_row = self.start_row.checked_sub(1).unwrap_or(self.start_row);
        let lower_row = self.end_row + 1;

        symbles.iter().any(|x| {
            x.col >= upper_col && x.col <= lower_col && x.row >= upper_row && x.row <= lower_row
        })
    }
    // Gear
    // COL 0-2
    // ROW 2-4

    // Num
    // COL 0-0
    // ROW 0-2
    pub fn in_range(&self, cord: &Cord) -> bool {
        let upper_col = self.start_col.checked_sub(1).unwrap_or(self.start_col);
        let lower_col = self.end_col + 1;

        let upper_row = self.start_row.checked_sub(1).unwrap_or(self.start_row);
        let lower_row = self.end_row + 1;

        cord.col >= upper_col
            && cord.col <= lower_col
            && cord.row >= upper_row
            && cord.row <= lower_row
    }
}

#[must_use]
fn part_two(input: &str) -> Option<usize> {
    let mut numbers: Vec<Number> = vec![];

    let mut gears: HashSet<Cord> = HashSet::new();

    for (col, line) in input.lines().enumerate() {
        let mut iter = line.chars().into_iter().peekable();

        let mut row: usize = 0;
        while let Some(char) = iter.next() {
            match char {
                e if e.is_numeric() => {
                    let mut num = format!("{e}");

                    while let Some(el) = iter.next_if(|x| x.is_numeric()) {
                        num.push(el);
                    }

                    let value_size = num.len() - 1;
                    let value = num.parse::<usize>().unwrap();

                    numbers.push(Number::new(value, col, row, col, row + value_size));

                    row += value_size;
                }
                '*' => {
                    /* Save symble locations for later */
                    gears.insert(Cord { col, row });
                }
                _ => { /* Ignore . */ }
            }

            row += 1;
        }
    }

    Some(gears.iter().fold(0, |acc, gear| {
        let values = numbers
            .iter()
            .filter(|num| num.in_range(gear))
            .collect::<Vec<&Number>>();
        if values.len() != 2 {
            acc
        } else {
            acc + values.iter().fold(1, |acc, x| acc * x.value)
        }
    }))
}

#[must_use]
fn part_one(input: &str) -> Option<usize> {
    let mut cached_symbles: HashSet<Cord> = HashSet::new();
    let mut numbers: Vec<Number> = vec![];

    for (col, line) in input.lines().enumerate() {
        let mut iter = line.chars().into_iter().peekable();

        let mut row: usize = 0;
        while let Some(char) = iter.next() {
            match char {
                e if e.is_numeric() => {
                    let mut num = format!("{e}");

                    while let Some(el) = iter.next_if(|x| x.is_numeric()) {
                        num.push(el);
                    }

                    let value_size = num.len() - 1;
                    let value = num.parse::<usize>().unwrap();

                    numbers.push(Number::new(value, col, row, col, row + value_size));

                    row += value_size;
                }
                '.' => { /* Ignore . */ }
                _ => {
                    /* Save symble locations for later */
                    cached_symbles.insert(Cord { col, row });
                }
            }

            row += 1;
        }
    }

    Some(
        numbers
            .iter()
            .filter(|x| x.ranges(&cached_symbles))
            .fold(0, |acc, e| acc + e.value),
    )
}

#[cfg(test)]
mod tests {}
