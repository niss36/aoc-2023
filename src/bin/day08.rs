use std::{collections::HashMap, io};

use aoc::read_lines;
use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Debug)]
enum AocError {
    IoError(io::Error),
    InvalidMove(char),
    InvalidNetworkEntry(String),
    InvalidMap(String),
}

impl From<io::Error> for AocError {
    fn from(e: io::Error) -> Self {
        Self::IoError(e)
    }
}

const INPUT_PATH: &str = "inputs/day08.txt";

fn main() -> Result<(), AocError> {
    let input = read_lines(INPUT_PATH)?;

    println!("Part 1: {:?}", part1(&input)?);
    println!("Part 2: {:?}", part2(&input)?);

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Move {
    Left,
    Right,
}

impl TryFrom<char> for Move {
    type Error = AocError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            _ => Err(AocError::InvalidMove(value)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Map {
    moves: Vec<Move>,
    network: HashMap<String, (String, String)>,
}

fn parse_network_entry(line: &str) -> Result<(String, (String, String)), AocError> {
    static ENTRY_REGEX: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"^(\w+) = \((\w+), (\w+)\)$").unwrap());

    let (_, [key, left, right]) = ENTRY_REGEX
        .captures(line)
        .ok_or_else(|| AocError::InvalidNetworkEntry(line.to_owned()))?
        .extract();

    Ok((key.to_owned(), (left.to_owned(), right.to_owned())))
}

impl TryFrom<&[String]> for Map {
    type Error = AocError;

    fn try_from(value: &[String]) -> Result<Self, Self::Error> {
        match value {
            [moves, space, network @ ..] if space.is_empty() => {
                let moves = moves.chars().map(|c| c.try_into()).try_collect()?;
                let network = network
                    .iter()
                    .map(|s| parse_network_entry(s))
                    .try_collect()?;

                Ok(Self { moves, network })
            }
            _ => Err(AocError::InvalidMap(value.join("\n"))),
        }
    }
}

impl Map {
    fn get_move_at(&self, steps: usize) -> Move {
        self.moves[steps % self.moves.len()]
    }

    fn next_position(&self, current_move: Move, current_position: &str) -> &str {
        let (left, right) = &self.network[current_position];

        match current_move {
            Move::Left => left,
            Move::Right => right,
        }
    }
}

fn steps_to_end(map: &Map, starting_pos: &str) -> usize {
    let mut pos = starting_pos;
    let mut steps = 0;

    while !pos.ends_with('Z') {
        pos = map.next_position(map.get_move_at(steps), pos);
        steps += 1;
    }

    steps
}

fn steps_to_end_2<S: AsRef<str>, Positions: IntoIterator<Item = S>>(
    map: &Map,
    starting_positions: Positions,
) -> usize {
    starting_positions
        .into_iter()
        .map(|pos| steps_to_end(map, pos.as_ref()))
        .fold(1, num::integer::lcm)
}

fn part1(input: &[String]) -> Result<usize, AocError> {
    let map: Map = input.try_into()?;

    Ok(steps_to_end(&map, "AAA"))
}

fn part2(input: &[String]) -> Result<usize, AocError> {
    let map: Map = input.try_into()?;

    let starting_positions = map.network.keys().filter(|key| key.ends_with('A'));

    Ok(steps_to_end_2(&map, starting_positions))
}

#[cfg(test)]
mod tests {
    use super::*;

    use aoc::to_lines;

    // Make sure to remove any extra indentation (otherwise it will be part of the string)
    const EXAMPLE: &str = "\
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";

    #[test]
    fn test_parse_map() {
        let input = to_lines(EXAMPLE);

        let map: Map = (input.as_slice()).try_into().unwrap();
        let expected_map = Map {
            moves: vec![Move::Left, Move::Left, Move::Right],
            network: HashMap::from([
                ("AAA".to_owned(), ("BBB".to_owned(), "BBB".to_owned())),
                ("BBB".to_owned(), ("AAA".to_owned(), "ZZZ".to_owned())),
                ("ZZZ".to_owned(), ("ZZZ".to_owned(), "ZZZ".to_owned())),
            ]),
        };

        assert_eq!(map, expected_map)
    }

    #[test]
    fn test_part1() {
        let input = to_lines(EXAMPLE);

        assert_eq!(part1(&input).unwrap(), 6);
    }

    const EXAMPLE_2: &str = "\
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";

    #[test]
    fn test_part2() {
        let input = to_lines(EXAMPLE_2);

        assert_eq!(part2(&input).unwrap(), 6);
    }
}
