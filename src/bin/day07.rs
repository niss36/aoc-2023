use std::{cmp::Ordering, io, num::ParseIntError, str::FromStr};

use aoc::read_lines;
use itertools::Itertools;

#[derive(Debug)]
enum AocError {
    IoError(io::Error),
    ParseIntError(ParseIntError),
    InvalidCard(char),
    InvalidHand(String),
    InvalidBid(String),
}

impl From<io::Error> for AocError {
    fn from(e: io::Error) -> Self {
        Self::IoError(e)
    }
}

impl From<ParseIntError> for AocError {
    fn from(e: ParseIntError) -> Self {
        Self::ParseIntError(e)
    }
}

const INPUT_PATH: &str = "inputs/day07.txt";

fn main() -> Result<(), AocError> {
    let input = read_lines(INPUT_PATH)?;

    println!("Part 1: {:?}", part1(&input)?);
    println!("Part 2: {:?}", part2(&input)?);

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

impl TryFrom<char> for Card {
    type Error = AocError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '2' => Ok(Self::Two),
            '3' => Ok(Self::Three),
            '4' => Ok(Self::Four),
            '5' => Ok(Self::Five),
            '6' => Ok(Self::Six),
            '7' => Ok(Self::Seven),
            '8' => Ok(Self::Eight),
            '9' => Ok(Self::Nine),
            'T' => Ok(Self::T),
            'J' => Ok(Self::J),
            'Q' => Ok(Self::Q),
            'K' => Ok(Self::K),
            'A' => Ok(Self::A),
            _ => Err(AocError::InvalidCard(value)),
        }
    }
}

impl Card {
    fn get_value_1(&self) -> usize {
        match self {
            Card::Two => 2,
            Card::Three => 3,
            Card::Four => 4,
            Card::Five => 5,
            Card::Six => 6,
            Card::Seven => 7,
            Card::Eight => 8,
            Card::Nine => 9,
            Card::T => 10,
            Card::J => 11,
            Card::Q => 12,
            Card::K => 13,
            Card::A => 14,
        }
    }

    fn cmp_1(&self, other: &Self) -> Ordering {
        self.get_value_1().cmp(&other.get_value_1())
    }

    fn get_value_2(&self) -> usize {
        match self {
            Card::J => 1,
            Card::Two => 2,
            Card::Three => 3,
            Card::Four => 4,
            Card::Five => 5,
            Card::Six => 6,
            Card::Seven => 7,
            Card::Eight => 8,
            Card::Nine => 9,
            Card::T => 10,
            Card::Q => 11,
            Card::K => 12,
            Card::A => 13,
        }
    }

    fn cmp_2(&self, other: &Self) -> Ordering {
        self.get_value_2().cmp(&other.get_value_2())
    }
}

fn cmp_cards_1(self_cards: &[Card], other_cards: &[Card]) -> Ordering {
    for (self_card, other_card) in self_cards.iter().zip(other_cards) {
        match self_card.cmp_1(other_card) {
            Ordering::Equal => (),
            order => return order,
        }
    }

    Ordering::Equal
}

fn cmp_cards_2(self_cards: &[Card], other_cards: &[Card]) -> Ordering {
    for (self_card, other_card) in self_cards.iter().zip(other_cards) {
        match self_card.cmp_2(other_card) {
            Ordering::Equal => (),
            order => return order,
        }
    }

    Ordering::Equal
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Hand(Vec<Card>);

impl FromStr for Hand {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards: Vec<_> = s.chars().map(|c| c.try_into()).try_collect()?;
        if cards.len() != 5 {
            return Err(AocError::InvalidHand(s.to_owned()));
        }

        Ok(Hand(cards))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn get_hand_type_from_counts(counts: std::collections::HashMap<&Card, usize>) -> HandType {
    let counts = counts.into_values().filter(|&c| c > 0).sorted();

    match counts.as_slice() {
        [5] => HandType::FiveOfAKind,
        [1, 4] => HandType::FourOfAKind,
        [2, 3] => HandType::FullHouse,
        [1, 1, 3] => HandType::ThreeOfAKind,
        [1, 2, 2] => HandType::TwoPair,
        [1, 1, 1, 2] => HandType::OnePair,
        [1, 1, 1, 1, 1] => HandType::HighCard,
        other => panic!("Unexpected hand type '{other:?}'"),
    }
}

impl Hand {
    fn get_hand_type_1(&self) -> HandType {
        get_hand_type_from_counts(self.0.iter().counts())
    }

    fn cmp_1(&self, other: &Self) -> Ordering {
        match self.get_hand_type_1().cmp(&other.get_hand_type_1()) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => cmp_cards_1(&self.0, &other.0),
            Ordering::Greater => Ordering::Greater,
        }
    }

    fn get_hand_type_2(&self) -> HandType {
        let counts = self.0.iter().counts();
        if let Some(jokers) = counts.get(&Card::J) {
            return (0..*jokers)
                .map(|_| counts.keys())
                .multi_cartesian_product()
                .map(|v| {
                    let mut counts = counts.clone();
                    for card in v {
                        counts.entry(&Card::J).and_modify(|c| *c -= 1);
                        counts.entry(card).and_modify(|c| *c += 1);
                    }

                    counts
                })
                .map(get_hand_type_from_counts)
                .max()
                .unwrap();
        }

        get_hand_type_from_counts(counts)
    }

    fn cmp_2(&self, other: &Self) -> Ordering {
        match self.get_hand_type_2().cmp(&other.get_hand_type_2()) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => cmp_cards_2(&self.0, &other.0),
            Ordering::Greater => Ordering::Greater,
        }
    }
}

fn parse_hand_and_bid(line: &str) -> Result<(Hand, usize), AocError> {
    let (hand, bid) = line
        .split(' ')
        .collect_tuple()
        .ok_or(AocError::InvalidBid(line.to_owned()))?;

    Ok((hand.parse()?, bid.parse()?))
}

fn parse_hands_and_bids(input: &[String]) -> Result<Vec<(Hand, usize)>, AocError> {
    input.iter().map(|line| parse_hand_and_bid(line)).collect()
}

fn get_total_winnings<F: Fn(&Hand, &Hand) -> Ordering>(
    mut hands_and_bids: Vec<(Hand, usize)>,
    compare: F,
) -> usize {
    hands_and_bids.sort_unstable_by(|(a, _), (b, _)| compare(a, b));

    hands_and_bids
        .iter()
        .enumerate()
        .map(|(i, &(_, bid))| (i + 1) * bid)
        .sum()
}

fn part1(input: &[String]) -> Result<usize, AocError> {
    let hands_and_bids = parse_hands_and_bids(input)?;

    Ok(get_total_winnings(hands_and_bids, Hand::cmp_1))
}

fn part2(input: &[String]) -> Result<usize, AocError> {
    let hands_and_bids = parse_hands_and_bids(input)?;

    Ok(get_total_winnings(hands_and_bids, Hand::cmp_2))
}

#[cfg(test)]
mod tests {
    use super::*;

    use aoc::to_lines;

    #[test]
    fn test_get_hand_type_1() {
        let hand: Hand = "QQQJA".parse().unwrap();
        assert_eq!(hand.get_hand_type_1(), HandType::ThreeOfAKind);
    }

    #[test]
    fn test_hand_cmp_1() {
        let hand0: Hand = "33332".parse().unwrap();
        let hand1: Hand = "2AAAA".parse().unwrap();

        assert_eq!(hand0.cmp_1(&hand1), Ordering::Greater);

        let hand0: Hand = "77888".parse().unwrap();
        let hand1: Hand = "77788".parse().unwrap();

        assert_eq!(hand0.cmp_1(&hand1), Ordering::Greater);
    }

    #[test]
    fn test_get_hand_type_2() {
        let hand: Hand = "QJJQ2".parse().unwrap();
        assert_eq!(hand.get_hand_type_2(), HandType::FourOfAKind);
    }

    #[test]
    fn test_hand_cmp_2() {
        let hand0: Hand = "QQQQ2".parse().unwrap();
        let hand1: Hand = "JKKK2".parse().unwrap();

        assert_eq!(hand0.cmp_2(&hand1), Ordering::Greater);
    }

    // Make sure to remove any extra indentation (otherwise it will be part of the string)
    const EXAMPLE: &str = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";

    #[test]
    fn test_part1() {
        let input = to_lines(EXAMPLE);

        assert_eq!(part1(&input).unwrap(), 6440);
    }

    #[test]
    fn test_part2() {
        let input = to_lines(EXAMPLE);

        assert_eq!(part2(&input).unwrap(), 5905);
    }
}
