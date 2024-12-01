use std::collections::{HashMap, VecDeque};

advent::solution!(4);

#[derive(Debug, PartialEq, Eq)]
struct Card {
    game: usize,
    winning: Vec<usize>,
    given: Vec<usize>,
}

impl Card {
    pub fn points(&self) -> usize {
        self.given
            .iter()
            .filter(|e| self.winning.contains(e))
            .enumerate()
            .fold(0, |acc, (idx, _)| if idx == 0 { 1 } else { acc * 2 })
    }

    pub fn winnings(&self) -> Vec<usize> {
        self.given
            .iter()
            .filter(|e| self.winning.contains(e))
            .enumerate()
            .map(|(idx, _)| idx + 1 + self.game)
            .collect()
    }
}

fn parse_input(input: &str) -> Vec<Card> {
    let mut cards = vec![];
    for line in input.lines() {
        let data = line.split(":").collect::<Vec<&str>>();

        let id = data
            .get(0)
            .unwrap()
            .replace("Card ", "")
            .trim()
            .parse::<usize>()
            .unwrap();

        let content = data.get(1).unwrap().split("|").collect::<Vec<&str>>();

        let winning = content
            .get(0)
            .unwrap()
            .split(" ")
            .filter_map(|e| e.trim().parse::<usize>().ok())
            .collect::<Vec<usize>>();
        let given = content
            .get(1)
            .unwrap()
            .split(" ")
            .filter_map(|e| e.trim().parse::<usize>().ok())
            .collect::<Vec<usize>>();

        cards.push(Card {
            game: id,
            winning,
            given,
        });
    }

    cards
}

#[must_use]
fn part_one(path: &str) -> Option<usize> {
    let input = parse_input(path);

    Some(input.iter().map(|e| e.points()).fold(0, |acc, v| acc + v))
}

#[must_use]
fn part_two(path: &str) -> Option<usize> {
    let input = parse_input(path);

    let mut output = input.len();

    let mut seen = HashMap::<usize, Vec<usize>>::new();

    let mut data = VecDeque::from((1..input.len()).collect::<Vec<usize>>());

    while !data.is_empty() {
        let idx = data.pop_front().unwrap() - 1;

        if seen.contains_key(&idx) {
            let list = seen.get(&idx).unwrap();
            output += list.len();

            data.extend(list.iter());
            continue;
        }

        let card = input.get(idx).unwrap();

        let wins = card.winnings();

        output += wins.len();

        data.extend(wins.iter());

        seen.insert(idx, wins);
    }

    Some(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_winning() {
        let card = Card {
            game: 1,
            winning: vec![41, 48, 83, 86, 17],
            given: vec![83, 86, 6, 31, 17, 9, 48, 53],
        };

        println!("{:#?}", card.winnings());
    }
}
