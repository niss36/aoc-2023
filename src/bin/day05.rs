use std::{io, num::ParseIntError, str::FromStr};

use aoc::read_lines;
use itertools::Itertools;

#[derive(Debug)]
enum AocError {
    IoError(io::Error),
    ParseIntError(ParseIntError),
    InvalidAlmanacMap(String),
    InvalidAlmanac,
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

const INPUT_PATH: &str = "inputs/day05.txt";

fn main() -> Result<(), AocError> {
    let input = read_lines(INPUT_PATH)?;

    println!("Part 1: {:?}", part1(&input)?);
    println!("Part 2: {:?}", part2(&input)?);

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct AlmanacMap {
    destination_range_start: usize,
    source_range_start: usize,
    range_length: usize,
}

impl AlmanacMap {
    fn apply(&self, value: usize) -> Option<usize> {
        if value < self.source_range_start || value >= self.source_range_start + self.range_length {
            return None;
        }

        Some(value - self.source_range_start + self.destination_range_start)
    }
}

fn apply_all(maps: &[AlmanacMap], value: usize) -> usize {
    maps.iter()
        .filter_map(|map| map.apply(value))
        .next()
        .unwrap_or(value)
}

impl FromStr for AlmanacMap {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (destination_range_start, source_range_start, range_length) = s
            .split(' ')
            .collect_tuple()
            .ok_or_else(|| AocError::InvalidAlmanacMap(s.to_owned()))?;

        Ok(Self {
            destination_range_start: destination_range_start.parse()?,
            source_range_start: source_range_start.parse()?,
            range_length: range_length.parse()?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Almanac {
    seeds: Vec<usize>,
    seed_to_soil_maps: Vec<AlmanacMap>,
    soil_to_fertilizer_maps: Vec<AlmanacMap>,
    fertilizer_to_water_maps: Vec<AlmanacMap>,
    water_to_light_maps: Vec<AlmanacMap>,
    light_to_temperature_maps: Vec<AlmanacMap>,
    temperature_to_humidity_maps: Vec<AlmanacMap>,
    humidity_to_location_maps: Vec<AlmanacMap>,
}

impl Almanac {
    fn convert_seed(&self, seed: usize) -> usize {
        let soil = apply_all(&self.seed_to_soil_maps, seed);
        let fertilizer = apply_all(&self.soil_to_fertilizer_maps, soil);
        let water = apply_all(&self.fertilizer_to_water_maps, fertilizer);
        let light = apply_all(&self.water_to_light_maps, water);
        let temperature = apply_all(&self.light_to_temperature_maps, light);
        let humidity = apply_all(&self.temperature_to_humidity_maps, temperature);

        apply_all(&self.humidity_to_location_maps, humidity)
    }

    fn convert_all_seeds(&self) -> impl Iterator<Item = usize> + '_ {
        self.seeds.iter().map(|&seed| self.convert_seed(seed))
    }

    fn convert_all_seeds_2(&self) -> impl Iterator<Item = usize> + '_ {
        let all_seeds = self
            .seeds
            .iter()
            .tuples()
            .flat_map(|(&start, &length)| start..start + length);

        all_seeds.map(|seed| self.convert_seed(seed))
    }
}

impl TryFrom<&[String]> for Almanac {
    type Error = AocError;

    fn try_from(value: &[String]) -> Result<Self, Self::Error> {
        let mut lines = value.iter();

        let seeds = lines
            .next()
            .and_then(|s| s.strip_prefix("seeds: "))
            .ok_or(AocError::InvalidAlmanac)?;
        let seeds = seeds.split(' ').map(|seed| seed.parse()).try_collect()?;

        if !lines.next().is_some_and(String::is_empty) {
            return Err(AocError::InvalidAlmanac);
        }

        fn parse_maps<'a>(
            header: &str,
            lines: &mut impl Iterator<Item = &'a String>,
        ) -> Result<Vec<AlmanacMap>, AocError> {
            if !lines.next().is_some_and(|s| s == header) {
                return Err(AocError::InvalidAlmanac);
            }

            let mut maps = vec![];

            for line in lines {
                if line.is_empty() {
                    break;
                }
                maps.push(line.parse()?);
            }

            Ok(maps)
        }

        let seed_to_soil_maps = parse_maps("seed-to-soil map:", &mut lines)?;
        let soil_to_fertilizer_maps = parse_maps("soil-to-fertilizer map:", &mut lines)?;
        let fertilizer_to_water_maps = parse_maps("fertilizer-to-water map:", &mut lines)?;
        let water_to_light_maps = parse_maps("water-to-light map:", &mut lines)?;
        let light_to_temperature_maps = parse_maps("light-to-temperature map:", &mut lines)?;
        let temperature_to_humidity_maps = parse_maps("temperature-to-humidity map:", &mut lines)?;
        let humidity_to_location_maps = parse_maps("humidity-to-location map:", &mut lines)?;

        Ok(Self {
            seeds,
            seed_to_soil_maps,
            soil_to_fertilizer_maps,
            fertilizer_to_water_maps,
            water_to_light_maps,
            light_to_temperature_maps,
            temperature_to_humidity_maps,
            humidity_to_location_maps,
        })
    }
}

fn part1(input: &[String]) -> Result<usize, AocError> {
    let almanac: Almanac = input.try_into()?;

    almanac
        .convert_all_seeds()
        .min()
        .ok_or(AocError::InvalidAlmanac)
}

fn part2(input: &[String]) -> Result<usize, AocError> {
    let almanac: Almanac = input.try_into()?;

    almanac
        .convert_all_seeds_2()
        .min()
        .ok_or(AocError::InvalidAlmanac)
}

#[cfg(test)]
mod tests {
    use super::*;

    use aoc::to_lines;

    #[test]
    fn test_parse_almanac() {
        let input = to_lines(
            "\
seeds: 1 2 3

seed-to-soil map:
3 4 5
5 6 7

soil-to-fertilizer map:
7 8 9

fertilizer-to-water map:
9 0 1

water-to-light map:
1 2 3

light-to-temperature map:
3 4 5

temperature-to-humidity map:
5 6 7

humidity-to-location map:
7 8 9
",
        );
        let almanac: Almanac = input.as_slice().try_into().unwrap();
        let expected_almanac = Almanac {
            seeds: vec![1, 2, 3],
            seed_to_soil_maps: vec![
                AlmanacMap {
                    destination_range_start: 3,
                    source_range_start: 4,
                    range_length: 5,
                },
                AlmanacMap {
                    destination_range_start: 5,
                    source_range_start: 6,
                    range_length: 7,
                },
            ],
            soil_to_fertilizer_maps: vec![AlmanacMap {
                destination_range_start: 7,
                source_range_start: 8,
                range_length: 9,
            }],
            fertilizer_to_water_maps: vec![AlmanacMap {
                destination_range_start: 9,
                source_range_start: 0,
                range_length: 1,
            }],
            water_to_light_maps: vec![AlmanacMap {
                destination_range_start: 1,
                source_range_start: 2,
                range_length: 3,
            }],
            light_to_temperature_maps: vec![AlmanacMap {
                destination_range_start: 3,
                source_range_start: 4,
                range_length: 5,
            }],
            temperature_to_humidity_maps: vec![AlmanacMap {
                destination_range_start: 5,
                source_range_start: 6,
                range_length: 7,
            }],
            humidity_to_location_maps: vec![AlmanacMap {
                destination_range_start: 7,
                source_range_start: 8,
                range_length: 9,
            }],
        };

        assert_eq!(almanac, expected_almanac);
    }

    #[test]
    fn test_almanac_map_apply() {
        let map = AlmanacMap {
            destination_range_start: 50,
            source_range_start: 98,
            range_length: 2,
        };

        assert_eq!(map.apply(0), None);
        assert_eq!(map.apply(98), Some(50));
        assert_eq!(map.apply(99), Some(51));
        assert_eq!(map.apply(100), None);
    }

    #[test]
    fn test_apply_all() {
        let maps = vec![
            AlmanacMap {
                destination_range_start: 50,
                source_range_start: 98,
                range_length: 2,
            },
            AlmanacMap {
                destination_range_start: 52,
                source_range_start: 50,
                range_length: 48,
            },
        ];

        assert_eq!(apply_all(&maps, 79), 81);
        assert_eq!(apply_all(&maps, 14), 14);
        assert_eq!(apply_all(&maps, 55), 57);
        assert_eq!(apply_all(&maps, 13), 13);
    }

    // Make sure to remove any extra indentation (otherwise it will be part of the string)
    const EXAMPLE: &str = "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";

    #[test]
    fn test_part1() {
        let input = to_lines(EXAMPLE);

        assert_eq!(part1(&input).unwrap(), 35);
    }

    #[test]
    fn test_part2() {
        let input = to_lines(EXAMPLE);

        assert_eq!(part2(&input).unwrap(), 46);
    }
}
