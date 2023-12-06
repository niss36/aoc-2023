use std::{io, iter::zip, num::ParseIntError};

use aoc::read_lines;
use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Debug)]
enum AocError {
    IoError(io::Error),
    ParseIntError(ParseIntError),
    InvalidRaces,
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

const INPUT_PATH: &str = "inputs/day06.txt";

fn main() -> Result<(), AocError> {
    let input = read_lines(INPUT_PATH)?;

    println!("Part 1: {:?}", part1(&input)?);
    println!("Part 2: {:?}", part2(&input)?);

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Race {
    time_allowed: usize,
    distance_record: usize,
}

impl Race {
    fn get_distance_for_time_holding_button(&self, time_held: usize) -> usize {
        let speed = time_held;
        let time = self.time_allowed.saturating_sub(time_held);

        speed * time
    }

    fn get_number_of_ways_to_win(&self) -> usize {
        (1..self.time_allowed)
            .map(|time_held| self.get_distance_for_time_holding_button(time_held))
            .filter(|distance| distance > &self.distance_record)
            .count()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Races(Vec<Race>);

impl TryFrom<&[String]> for Races {
    type Error = AocError;

    fn try_from(value: &[String]) -> Result<Self, Self::Error> {
        let [times, distances] = value else {
            return Err(AocError::InvalidRaces);
        };

        static WHITESPACE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\s+").unwrap());

        let times = times
            .strip_prefix("Time:")
            .map(|t| t.trim())
            .ok_or(AocError::InvalidRaces)?;
        let times: Vec<usize> = WHITESPACE_REGEX
            .split(times)
            .map(|time| time.parse())
            .try_collect()?;

        let distances = distances
            .strip_prefix("Distance:")
            .map(|d| d.trim())
            .ok_or(AocError::InvalidRaces)?;
        let distances: Vec<usize> = WHITESPACE_REGEX
            .split(distances)
            .map(|distance| distance.parse())
            .try_collect()?;

        let races = zip(times, distances)
            .map(|(time, distance)| Race {
                time_allowed: time,
                distance_record: distance,
            })
            .collect();

        Ok(Self(races))
    }
}

fn part1(input: &[String]) -> Result<usize, AocError> {
    let races: Races = input.try_into()?;

    Ok(races
        .0
        .iter()
        .map(|race| race.get_number_of_ways_to_win())
        .product())
}

fn parse_race_2(input: &[String]) -> Result<Race, AocError> {
    let [time_line, distance_line] = input else {
        return Err(AocError::InvalidRaces);
    };

    let time = time_line
        .strip_prefix("Time:")
        .map(|t| t.replace(' ', ""))
        .ok_or(AocError::InvalidRaces)?;

    let distance = distance_line
        .strip_prefix("Distance:")
        .map(|t| t.replace(' ', ""))
        .ok_or(AocError::InvalidRaces)?;

    Ok(Race {
        time_allowed: time.parse()?,
        distance_record: distance.parse()?,
    })
}

fn part2(input: &[String]) -> Result<usize, AocError> {
    let race = parse_race_2(input)?;

    Ok(race.get_number_of_ways_to_win())
}

#[cfg(test)]
mod tests {
    use super::*;

    use aoc::to_lines;

    // Make sure to remove any extra indentation (otherwise it will be part of the string)
    const EXAMPLE: &str = "\
Time:      7  15   30
Distance:  9  40  200
";

    #[test]
    fn test_parse_races() {
        let input = to_lines(EXAMPLE);
        let races: Races = input.as_slice().try_into().unwrap();
        let expected_races = Races(vec![
            Race {
                time_allowed: 7,
                distance_record: 9,
            },
            Race {
                time_allowed: 15,
                distance_record: 40,
            },
            Race {
                time_allowed: 30,
                distance_record: 200,
            },
        ]);

        assert_eq!(races, expected_races);
    }

    #[test]
    fn test_part1() {
        let input = to_lines(EXAMPLE);

        assert_eq!(part1(&input).unwrap(), 288);
    }

    #[test]
    fn test_part2() {
        let input = to_lines(EXAMPLE);

        assert_eq!(part2(&input).unwrap(), 71503);
    }
}
