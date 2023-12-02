use std::{io, num::ParseIntError, str::FromStr};

use aoc::read_lines;
use itertools::Itertools;

#[derive(Debug)]
enum AocError {
    IoError(io::Error),
    ParseIntError(ParseIntError),
    InvalidDrawnCubes(String),
    InvalidGame(String),
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

const INPUT_PATH: &str = "inputs/day02.txt";

fn main() -> Result<(), AocError> {
    let input = read_lines(INPUT_PATH)?;

    println!("Part 1: {:?}", part1(&input)?);
    println!("Part 2: {:?}", part2(&input)?);

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct DrawnCubes {
    red: usize,
    green: usize,
    blue: usize,
}

impl FromStr for DrawnCubes {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for part in s.split(", ") {
            if let Some((amount, colour)) = part.split_whitespace().collect_tuple() {
                let amount: usize = amount.parse()?;
                match colour {
                    "red" => red = amount,
                    "green" => green = amount,
                    "blue" => blue = amount,
                    _ => return Err(AocError::InvalidDrawnCubes(s.to_owned())),
                }
            } else {
                return Err(AocError::InvalidDrawnCubes(s.to_owned()));
            }
        }

        Ok(Self { red, green, blue })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Game {
    id: usize,
    draws: Vec<DrawnCubes>,
}

impl FromStr for Game {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (prefix, draws) = s
            .split(": ")
            .collect_tuple()
            .ok_or(AocError::InvalidGame(s.to_owned()))?;

        let id = prefix
            .strip_prefix("Game ")
            .ok_or(AocError::InvalidGame(s.to_owned()))?
            .parse()?;
        let draws = draws.split("; ").map(|draw| draw.parse()).try_collect()?;

        Ok(Self { id, draws })
    }
}

fn part1(input: &[String]) -> Result<usize, AocError> {
    let games: Vec<Game> = input.iter().map(|line| line.parse()).try_collect()?;

    let possible_games = games
        .iter()
        .filter(|game| is_game_possible(game, 12, 13, 14));

    Ok(possible_games.map(|game| game.id).sum())
}

fn is_game_possible(game: &Game, red: usize, green: usize, blue: usize) -> bool {
    game.draws
        .iter()
        .all(|draw| draw.red <= red && draw.green <= green && draw.blue <= blue)
}

fn part2(input: &[String]) -> Result<usize, AocError> {
    let games: Vec<Game> = input.iter().map(|line| line.parse()).try_collect()?;

    Ok(games
        .iter()
        .map(get_minimum_draw)
        .map(|draw| draw.red * draw.green * draw.blue)
        .sum())
}

fn get_minimum_draw(game: &Game) -> DrawnCubes {
    game.draws
        .iter()
        .fold(Default::default(), |acc, draw| DrawnCubes {
            red: acc.red.max(draw.red),
            green: acc.green.max(draw.green),
            blue: acc.blue.max(draw.blue),
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    use aoc::to_lines;

    #[test]
    fn test_parse_game() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game: Game = input.parse().unwrap();
        let expected_game = Game {
            id: 1,
            draws: vec![
                DrawnCubes {
                    red: 4,
                    green: 0,
                    blue: 3,
                },
                DrawnCubes {
                    red: 1,
                    green: 2,
                    blue: 6,
                },
                DrawnCubes {
                    red: 0,
                    green: 2,
                    blue: 0,
                },
            ],
        };

        assert_eq!(game, expected_game);
    }

    // Make sure to remove any extra indentation (otherwise it will be part of the string)
    const EXAMPLE: &str = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green    
";

    #[test]
    fn test_part1() {
        let input = to_lines(EXAMPLE);

        assert_eq!(part1(&input).unwrap(), 8);
    }

    #[test]
    fn test_part2() {
        let input = to_lines(EXAMPLE);

        assert_eq!(part2(&input).unwrap(), 2286);
    }
}
