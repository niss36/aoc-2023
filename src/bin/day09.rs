use std::{io, num::ParseIntError, str::FromStr};

use aoc::read_lines;
use itertools::Itertools;

#[derive(Debug)]
enum AocError {
    IoError(io::Error),
    ParseIntError(ParseIntError),
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

const INPUT_PATH: &str = "inputs/day09.txt";

fn main() -> Result<(), AocError> {
    let input = read_lines(INPUT_PATH)?;

    println!("Part 1: {:?}", part1(&input)?);
    println!("Part 2: {:?}", part2(&input)?);

    Ok(())
}

struct Sequence(Vec<i64>);

impl FromStr for Sequence {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers = s.split(' ').map(|n| n.parse()).try_collect()?;

        Ok(Self(numbers))
    }
}

impl Sequence {
    fn create_diff_sequence(self) -> Self {
        let numbers = self
            .0
            .into_iter()
            .tuple_windows()
            .map(|(a, b)| b - a)
            .collect();

        Self(numbers)
    }

    fn is_zero(&self) -> bool {
        self.0.iter().all(|n| *n == 0)
    }

    fn extrapolate(self) -> i64 {
        if self.is_zero() {
            return 0;
        }

        let last = *self.0.last().unwrap();
        let diff = self.create_diff_sequence();

        last + diff.extrapolate()
    }

    fn extrapolate_backwards(self) -> i64 {
        if self.is_zero() {
            return 0;
        }

        let first = *self.0.first().unwrap();
        let diff = self.create_diff_sequence();

        first - diff.extrapolate_backwards()
    }
}

fn part1(input: &[String]) -> Result<i64, AocError> {
    let sequences: Vec<Sequence> = input.iter().map(|line| line.parse()).try_collect()?;

    Ok(sequences.into_iter().map(Sequence::extrapolate).sum())
}

fn part2(input: &[String]) -> Result<i64, AocError> {
    let sequences: Vec<Sequence> = input.iter().map(|line| line.parse()).try_collect()?;

    Ok(sequences
        .into_iter()
        .map(Sequence::extrapolate_backwards)
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    use aoc::to_lines;

    // Make sure to remove any extra indentation (otherwise it will be part of the string)
    const EXAMPLE: &str = "\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";

    #[test]
    fn test_part1() {
        let input = to_lines(EXAMPLE);

        assert_eq!(part1(&input).unwrap(), 114);
    }

    #[test]
    fn test_part2() {
        let input = to_lines(EXAMPLE);

        assert_eq!(part2(&input).unwrap(), 2);
    }
}
