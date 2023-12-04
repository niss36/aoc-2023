use std::{
    collections::{HashMap, HashSet},
    io,
    num::ParseIntError,
    str::FromStr,
};

use aoc::read_lines;
use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Debug)]
enum AocError {
    IoError(io::Error),
    ParseIntError(ParseIntError),
    InvalidScratchCard(String),
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

const INPUT_PATH: &str = "inputs/day04.txt";

fn main() -> Result<(), AocError> {
    let input = read_lines(INPUT_PATH)?;

    println!("Part 1: {:?}", part1(&input)?);
    println!("Part 2: {:?}", part2(&input)?);

    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ScratchCard {
    id: usize,
    left_numbers: HashSet<usize>,
    right_numbers: HashSet<usize>,
}

impl FromStr for ScratchCard {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static CARD_REGEX: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"^Card\s+(\d+):\s+([^|]*) \|\s+([^|]*)$").unwrap());

        let (_, [id, left, right]) = CARD_REGEX
            .captures(s)
            .map(|caps| caps.extract())
            .ok_or(AocError::InvalidScratchCard(s.to_owned()))?;

        let id = id.parse()?;

        static WHITESPACE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\s+").unwrap());

        let left_numbers = WHITESPACE_REGEX
            .split(left)
            .map(|n| n.parse())
            .try_collect()?;
        let right_numbers = WHITESPACE_REGEX
            .split(right)
            .map(|n| n.parse())
            .try_collect()?;

        Ok(Self {
            id,
            left_numbers,
            right_numbers,
        })
    }
}

impl ScratchCard {
    fn count_matches(&self) -> usize {
        self.left_numbers.intersection(&self.right_numbers).count()
    }

    fn get_points(&self) -> usize {
        let number_matches = self.count_matches();

        if number_matches > 0 {
            2usize.pow((number_matches - 1) as u32)
        } else {
            0
        }
    }
}

fn part1(input: &[String]) -> Result<usize, AocError> {
    let cards: Vec<ScratchCard> = input.iter().map(|line| line.parse()).try_collect()?;

    let points = cards.iter().map(ScratchCard::get_points).sum();

    Ok(points)
}

fn part2(input: &[String]) -> Result<usize, AocError> {
    let cards: Vec<ScratchCard> = input.iter().map(|line| line.parse()).try_collect()?;

    let mut copies: HashMap<usize, usize> = HashMap::new();

    let mut total_cards = 0;

    for card in cards {
        let multiplier = 1 + copies.get(&card.id).copied().unwrap_or_default();
        total_cards += multiplier;

        let matches = card.count_matches();

        for i in 1..=matches {
            copies
                .entry(card.id + i)
                .and_modify(|v| *v += multiplier)
                .or_insert(multiplier);
        }
    }

    Ok(total_cards)
}

#[cfg(test)]
mod tests {
    use super::*;

    use aoc::to_lines;

    #[test]
    fn test_parse_scratch_card() {
        let input = "Card 123:  1 23 |  4 56";
        let scratch_card: ScratchCard = input.parse().unwrap();
        let expected_scratch_card = ScratchCard {
            id: 123,
            left_numbers: HashSet::from([1, 23]),
            right_numbers: HashSet::from([4, 56]),
        };

        assert_eq!(scratch_card, expected_scratch_card);
    }

    // Make sure to remove any extra indentation (otherwise it will be part of the string)
    const EXAMPLE: &str = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";

    #[test]
    fn test_part1() {
        let input = to_lines(EXAMPLE);

        assert_eq!(part1(&input).unwrap(), 13);
    }

    #[test]
    fn test_part2() {
        let input = to_lines(EXAMPLE);

        assert_eq!(part2(&input).unwrap(), 30);
    }
}
