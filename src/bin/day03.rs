use std::{collections::HashMap, io, num::ParseIntError};

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

const INPUT_PATH: &str = "inputs/day03.txt";

fn main() -> Result<(), AocError> {
    let input = read_lines(INPUT_PATH)?;

    println!("Part 1: {:?}", part1(&input)?);
    println!("Part 2: {:?}", part2(&input)?);

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct EngineSchematicNumber {
    number: usize,
    x_start: usize,
    x_end: usize,
    y: usize,
}

#[derive(Debug, PartialEq, Eq)]
struct EngineSchematic {
    numbers: Vec<EngineSchematicNumber>,
    symbols: HashMap<(usize, usize), char>,
}

fn parse_engine_schematic(input: &[String]) -> Result<EngineSchematic, AocError> {
    let mut numbers = vec![];
    let mut symbols = HashMap::new();

    for (y, line) in input.iter().enumerate() {
        let mut current_number_span: Option<(String, usize)> = None;

        for (x, c) in line.chars().enumerate() {
            current_number_span = match (current_number_span, c) {
                (None, '.') => None,
                (None, n) if n.is_ascii_digit() => Some((String::from(n), x)),
                (None, s) => {
                    symbols.insert((x, y), s);

                    None
                }
                (Some((span, x_start)), '.') => {
                    numbers.push(EngineSchematicNumber {
                        number: span.parse()?,
                        x_start,
                        x_end: x - 1,
                        y,
                    });

                    None
                }
                (Some((mut span, x_start)), n) if n.is_ascii_digit() => {
                    span.push(n);

                    Some((span, x_start))
                }
                (Some((span, x_start)), s) => {
                    symbols.insert((x, y), s);
                    numbers.push(EngineSchematicNumber {
                        number: span.parse()?,
                        x_start,
                        x_end: x - 1,
                        y,
                    });

                    None
                }
            }
        }

        if let Some((span, x_start)) = current_number_span {
            numbers.push(EngineSchematicNumber {
                number: span.parse()?,
                x_start,
                x_end: line.chars().count() - 1,
                y,
            });
        }
    }

    Ok(EngineSchematic { numbers, symbols })
}

fn part1(input: &[String]) -> Result<usize, AocError> {
    let schematic = parse_engine_schematic(input)?;

    let engine_part_numbers = schematic
        .numbers
        .iter()
        .filter(|number| is_adjacent_to_symbol(**number, &schematic.symbols));

    Ok(engine_part_numbers.map(|number| number.number).sum())
}

fn get_neighbours(number: EngineSchematicNumber) -> Vec<(usize, usize)> {
    let mut neighbours = vec![];

    if number.x_start > 0 && number.y > 0 {
        // top-left
        neighbours.push((number.x_start - 1, number.y - 1));
    }

    if number.x_start > 0 {
        // middle-left
        neighbours.push((number.x_start - 1, number.y));
        // bottom-left
        neighbours.push((number.x_start - 1, number.y + 1));
    }

    if number.y > 0 {
        // top & top-right
        neighbours.extend((number.x_start..=(number.x_end + 1)).map(|x| (x, number.y - 1)));
    }

    // bottom & bottom-right
    neighbours.extend((number.x_start..=(number.x_end + 1)).map(|x| (x, number.y + 1)));

    // middle-right
    neighbours.push((number.x_end + 1, number.y));

    neighbours
}

fn is_adjacent_to_symbol(
    number: EngineSchematicNumber,
    symbols: &HashMap<(usize, usize), char>,
) -> bool {
    get_neighbours(number)
        .into_iter()
        .any(|pos| symbols.contains_key(&pos))
}

fn part2(input: &[String]) -> Result<usize, AocError> {
    let schematic = parse_engine_schematic(input)?;

    let potential_gears = schematic.symbols.into_iter().filter(|(_, s)| *s == '*');

    let gear_ratios =
        potential_gears.filter_map(|(gear, _)| get_gear_ratio(gear, &schematic.numbers));

    Ok(gear_ratios.sum())
}

fn get_gear_ratio(gear: (usize, usize), numbers: &[EngineSchematicNumber]) -> Option<usize> {
    let neighbouring_numbers = numbers
        .iter()
        .filter(|number| get_neighbours(**number).into_iter().any(|pos| pos == gear));

    neighbouring_numbers
        .collect_tuple()
        .map(|(number1, number2)| (number1.number * number2.number))
}

#[cfg(test)]
mod tests {
    use super::*;

    use aoc::to_lines;

    #[test]
    fn test_parse_engine_schematic() {
        let input = to_lines("123.123#123\n..123.123.#.123");

        let schematic = parse_engine_schematic(&input).unwrap();
        let expected_schematic = EngineSchematic {
            numbers: vec![
                EngineSchematicNumber {
                    number: 123,
                    x_start: 0,
                    x_end: 2,
                    y: 0,
                },
                EngineSchematicNumber {
                    number: 123,
                    x_start: 4,
                    x_end: 6,
                    y: 0,
                },
                EngineSchematicNumber {
                    number: 123,
                    x_start: 8,
                    x_end: 10,
                    y: 0,
                },
                EngineSchematicNumber {
                    number: 123,
                    x_start: 2,
                    x_end: 4,
                    y: 1,
                },
                EngineSchematicNumber {
                    number: 123,
                    x_start: 6,
                    x_end: 8,
                    y: 1,
                },
                EngineSchematicNumber {
                    number: 123,
                    x_start: 12,
                    x_end: 14,
                    y: 1,
                },
            ],
            symbols: HashMap::from([((7, 0), '#'), ((10, 1), '#')]),
        };

        assert_eq!(schematic, expected_schematic);
    }

    #[test]
    fn test_get_neighbours_corner() {
        let number = EngineSchematicNumber {
            number: 123,
            x_start: 0,
            x_end: 0,
            y: 0,
        };
        let neighbours = get_neighbours(number);

        assert_eq!(neighbours, vec![(0, 1), (1, 1), (1, 0)]);
    }

    #[test]
    fn test_get_neighbours() {
        let number = EngineSchematicNumber {
            number: 123,
            x_start: 1,
            x_end: 3,
            y: 1,
        };
        let neighbours = get_neighbours(number);

        assert_eq!(neighbours.len(), 12);
    }

    // Make sure to remove any extra indentation (otherwise it will be part of the string)
    const EXAMPLE: &str = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";

    #[test]
    fn test_part1() {
        let input = to_lines(EXAMPLE);

        assert_eq!(part1(&input).unwrap(), 4361);
    }

    #[test]
    fn test_part2() {
        let input = to_lines(EXAMPLE);

        assert_eq!(part2(&input).unwrap(), 467835);
    }
}
