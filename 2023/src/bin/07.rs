use itertools::Itertools;
use std::{str::FromStr, sync::Once};

advent::solution!(7);

static mut WILDCARDS: bool = false;
static INIT: Once = Once::new();

fn has_widcards() -> bool {
    unsafe { WILDCARDS }
}

fn enable_wildcards() {
    INIT.call_once(|| unsafe {
        WILDCARDS = true;
    });
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Hash)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
}

impl Card {
    fn discriminant(&self) -> u8 {
        unsafe { *<*const _>::from(self).cast::<u8>() }
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let value = match self {
            Self::J => {
                if has_widcards() {
                    0
                } else {
                    Self::J.discriminant() + 1
                }
            }
            e => e.discriminant() + 1,
        };

        let rhs = match other {
            Self::J => {
                if has_widcards() {
                    0
                } else {
                    Self::J.discriminant() + 1
                }
            }
            e => e.discriminant() + 1,
        };

        value.cmp(&rhs)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let value = match self {
            Self::J => {
                if has_widcards() {
                    0
                } else {
                    Self::J.discriminant()
                }
            }
            e => e.discriminant(),
        };

        let rhs = match other {
            Self::J => {
                if has_widcards() {
                    0
                } else {
                    Self::J.discriminant()
                }
            }
            e => e.discriminant(),
        };

        value.partial_cmp(&rhs)
    }
}

impl TryFrom<char> for Card {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Self::A),
            'K' => Ok(Self::K),
            'Q' => Ok(Self::Q),
            'J' => Ok(Self::J),
            'T' => Ok(Self::T),
            '9' => Ok(Self::Nine),
            '8' => Ok(Self::Eight),
            '7' => Ok(Self::Seven),
            '6' => Ok(Self::Six),
            '5' => Ok(Self::Five),
            '4' => Ok(Self::Four),
            '3' => Ok(Self::Three),
            '2' => Ok(Self::Two),
            _ => Err("Failed to parse input"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: Vec<Card>,
    hand_kind: Kind,
}

impl Hand {
    fn parse_without_widcards(cards: &Vec<Card>) -> Kind {
        let card_counts = cards.iter().counts();
        let counts = card_counts.values();

        let len = counts.len();
        let count = counts.fold(1, |acc, e| acc * e);

        // 5 * 1 = 5; Five of a kind
        // 4 * 1 * 1 = 4
        // 3 * 2 * 1 = 6
        // 3 * 1 * 1 * 1 = 3
        // 2 * 2 * 1 * 1 = 4
        // 2 * 1 * 1 * 1 *1 = 2
        // 1 * 1 * 1 * 1 * 1 * 1 = 1

        match count {
            6 => Kind::FullHouse,
            5 => Kind::FiveOfAKind,
            4 => {
                if len == 2 {
                    Kind::FourOfAKind
                } else {
                    Kind::TwoPair
                }
            }
            3 => Kind::ThreeOfAKind,
            2 => Kind::OnePair,
            1 => Kind::HighCard,
            _ => unreachable!("The count of the cards should be between 1 and 6."),
        }
    }

    fn parse_with_widcards(cards: &Vec<Card>) -> Kind {
        let mut cards_counts = cards.iter().counts();
        if cards_counts.contains_key(&Card::J) {
            // max may be empty if we have a FiveOfAKind that is are wilds
            if let Some(max) = cards_counts
                .iter()
                .filter(|x| *x.0 != &Card::J)
                .max_by(|lhs, rhs| lhs.1.cmp(&rhs.1).then_with(|| lhs.0.cmp(rhs.0)))
            {
                let jack = cards_counts.get(&Card::J).expect("Failed to get wilds.");
                cards_counts.insert(max.0, max.1 + jack);

                cards_counts.remove(&Card::J);
            }
        }

        let counts = cards_counts.values();
        let len = counts.len();
        let count = counts.fold(1, |acc, e| acc * e);

        match count {
            6 => Kind::FullHouse,
            5 => Kind::FiveOfAKind,
            4 => {
                if len == 2 {
                    Kind::FourOfAKind
                } else {
                    Kind::TwoPair
                }
            }
            3 => Kind::ThreeOfAKind,
            2 => Kind::OnePair,
            1 => Kind::HighCard,
            _ => unreachable!("The count of the cards should be between 1 and 6."),
        }
    }

    pub fn new(cards: Vec<Card>) -> Self {
        let kind = if has_widcards() {
            Hand::parse_with_widcards(&cards)
        } else {
            Hand::parse_without_widcards(&cards)
        };

        Self {
            cards,
            hand_kind: kind,
        }
    }
}

impl FromStr for Hand {
    type Err = usize;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(
            s.chars()
                .map(|x| x.try_into().expect("Failed to parse."))
                .collect::<Vec<Card>>(),
        ))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.hand_kind.partial_cmp(&other.hand_kind) {
            Some(order) => {
                if order == std::cmp::Ordering::Equal {
                    return self.cards.partial_cmp(&other.cards);
                }
                Some(order)
            }
            None => None,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let result = self.hand_kind.cmp(&other.hand_kind);

        if result == std::cmp::Ordering::Equal {
            return self.cards.cmp(&other.cards);
        }

        result
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Kind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug)]
struct Bid {
    hand: Hand,
    value: usize,
}

impl FromStr for Bid {
    type Err = std::fmt::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((hand_raw, bid)) = s.split_whitespace().collect_tuple::<(&str, &str)>() {
            let hand = hand_raw
                .trim()
                .parse::<Hand>()
                .map_err(|_| std::fmt::Error {})?;
            let value = bid
                .trim()
                .parse::<usize>()
                .map_err(|_| std::fmt::Error {})?;

            Ok(Self { hand, value })
        } else {
            Err(std::fmt::Error {})
        }
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<usize> {
    let result = input
        .lines()
        .filter_map(|e| e.parse::<Bid>().ok())
        .sorted_by(|a, b| a.hand.cmp(&b.hand))
        .enumerate()
        .fold(0, |acc, (idx, bid)| acc + (bid.value * (idx + 1)));

    Some(result)
}

#[must_use]
pub fn part_two(input: &str) -> Option<usize> {
    enable_wildcards();

    let result = input
        .lines()
        .filter_map(|e| e.parse::<Bid>().ok())
        .sorted_by(|a, b| a.hand.cmp(&b.hand))
        .enumerate()
        .fold(0, |acc, (idx, bid)| acc + (bid.value * (idx + 1)));

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wild_order() {
        enable_wildcards();
        let mut hands = vec!["J5J2K", "QK4K4"]
            .iter()
            .map(|x| x.parse::<Hand>().expect("Failed to parse"))
            .collect::<Vec<Hand>>();

        hands.sort();

        println!("{:#?}", hands);
    }

    #[test]
    fn test_wild_parse() {
        enable_wildcards();
        let hand = "QJJQ2".parse::<Hand>().expect("Failed to parse");
        assert_eq!(hand.hand_kind, Kind::FourOfAKind);

        let hand_two = "KTJJT".parse::<Hand>().expect("Failed to parse");
        assert_eq!(hand_two.hand_kind, Kind::FourOfAKind);

        let hand_tree = "43KJJ".parse::<Hand>().expect("Failed to parse");

        assert_eq!(hand_tree.hand_kind, Kind::ThreeOfAKind);
    }

    #[test]
    fn test_ord_of_card() {
        assert!(Card::A > Card::Two);
    }

    #[test]
    fn test_ord_of_kind() {
        assert!(Kind::FiveOfAKind > Kind::HighCard)
    }

    #[test]
    fn test_hand_parse() {
        let five_hand = "AAAAA".parse::<Hand>().expect("Failed to parse output.");

        assert_eq!(five_hand.hand_kind, Kind::FiveOfAKind);

        let four_hand = "AA8AA".parse::<Hand>().expect("Failed to parse output.");
        assert_eq!(four_hand.hand_kind, Kind::FourOfAKind);

        let full_hand = "23332".parse::<Hand>().expect("Failed to parse output.");
        assert_eq!(full_hand.hand_kind, Kind::FullHouse);

        let three_hand = "TTT98".parse::<Hand>().expect("Failed to parse output.");
        assert_eq!(three_hand.hand_kind, Kind::ThreeOfAKind);

        let two_hand = "23432".parse::<Hand>().expect("Failed to parse output.");
        assert_eq!(two_hand.hand_kind, Kind::TwoPair);

        let one_hand = "A23A4".parse::<Hand>().expect("Failed to parse output.");
        assert_eq!(one_hand.hand_kind, Kind::OnePair);

        let high_hand = "23456".parse::<Hand>().expect("Failed to parse output.");
        assert_eq!(high_hand.hand_kind, Kind::HighCard);
    }

    #[test]
    fn test_default_ord_on_enum() {
        let mut test = vec![
            Kind::FullHouse,
            Kind::FiveOfAKind,
            Kind::FourOfAKind,
            Kind::OnePair,
            Kind::HighCard,
            Kind::ThreeOfAKind,
            Kind::TwoPair,
        ];

        let aws = vec![
            Kind::HighCard,
            Kind::OnePair,
            Kind::TwoPair,
            Kind::ThreeOfAKind,
            Kind::FullHouse,
            Kind::FourOfAKind,
            Kind::FiveOfAKind,
        ];

        test.sort();

        assert_eq!(test, aws);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
