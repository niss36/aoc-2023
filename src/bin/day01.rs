use std::{io, num::ParseIntError};

use aoc::read_lines;
use itertools::Itertools;

#[derive(Debug)]
enum AocError {
    IoError(io::Error),
    ParseIntError(ParseIntError),
    NoDigits,
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

const INPUT_PATH: &str = "inputs/day01.txt";

fn main() -> Result<(), AocError> {
    let input = read_lines(INPUT_PATH)?;

    println!("Part 1: {:?}", part1(&input)?);
    println!("Part 2: {:?}", part2(&input)?);

    Ok(())
}

fn part1(input: &[String]) -> Result<usize, AocError> {
    let first_last_digits = input.iter().map(get_first_and_last_digits);

    let calibration_values: Vec<_> = first_last_digits
        .map(|result| result.and_then(get_number_from_digits))
        .try_collect()?;

    Ok(calibration_values.iter().sum())
}

fn get_first_and_last_digits<S: AsRef<str>>(line: S) -> Result<(char, char), AocError> {
    let line_digits = line
        .as_ref()
        .chars()
        .filter(|c| c.is_numeric())
        .collect_vec();

    let &first_digit = line_digits.first().ok_or(AocError::NoDigits)?;
    let &last_digit = line_digits.last().ok_or(AocError::NoDigits)?;

    Ok((first_digit, last_digit))
}

fn get_number_from_digits((first, last): (char, char)) -> Result<usize, AocError> {
    Ok(format!("{first}{last}").parse()?)
}

fn part2(input: &[String]) -> Result<usize, AocError> {
    let first_last_digits = input.iter().map(get_first_and_last_digits_2);

    let calibration_values: Vec<_> = first_last_digits
        .map(|result| result.and_then(get_number_from_digits))
        .try_collect()?;

    Ok(calibration_values.iter().sum())
}

const DIGITS: [(&str, char); 18] = [
    ("1", '1'),
    ("2", '2'),
    ("3", '3'),
    ("4", '4'),
    ("5", '5'),
    ("6", '6'),
    ("7", '7'),
    ("8", '8'),
    ("9", '9'),
    ("one", '1'),
    ("two", '2'),
    ("three", '3'),
    ("four", '4'),
    ("five", '5'),
    ("six", '6'),
    ("seven", '7'),
    ("eight", '8'),
    ("nine", '9'),
];

fn get_first_and_last_digits_2<S: AsRef<str>>(line: S) -> Result<(char, char), AocError> {
    let line = line.as_ref();

    let first_digits = DIGITS
        .into_iter()
        .filter_map(|(pattern, digit)| line.find(pattern).map(|index| (index, digit)));

    let last_digits = DIGITS
        .into_iter()
        .filter_map(|(pattern, digit)| line.rfind(pattern).map(|index| (index, digit)));

    let (_, first) = first_digits
        .min_by_key(|(index, _)| *index)
        .ok_or(AocError::NoDigits)?;

    let (_, last) = last_digits
        .max_by_key(|(index, _)| *index)
        .ok_or(AocError::NoDigits)?;

    Ok((first, last))
}

#[cfg(test)]
mod tests {
    use super::*;

    use aoc::to_lines;

    // Make sure to remove any extra indentation (otherwise it will be part of the string)
    const EXAMPLE_1: &str = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";
    const EXAMPLE_2: &str = "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";

    #[test]
    fn test_part1() {
        let input = to_lines(EXAMPLE_1);

        assert_eq!(part1(&input).unwrap(), 142);
    }

    #[test]
    fn test_part2() {
        let input = to_lines(EXAMPLE_2);

        assert_eq!(part2(&input).unwrap(), 281);
    }
}
