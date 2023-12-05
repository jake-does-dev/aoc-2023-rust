use std::cmp::min;
use std::fmt::Debug;

use crate::{DayRunner, FileLoader, Part, TaskType};

#[derive(Debug)]
pub struct Day05;

impl DayRunner for Day05 {
    fn run(part: Part) {
        let result = match part {
            Part::PartOne => run(
                &FileLoader::load("05", &TaskType::Puzzle),
                SeedInputType::Individual,
            ),
            Part::PartTwo => run(
                &FileLoader::load("05", &TaskType::Puzzle),
                SeedInputType::Range,
            ),
        };

        Self::report_result(Self, part, result);
    }
}

fn run(data: &str, seed_input_type: SeedInputType) -> u64 {
    let groups = data.split("\n\n").collect::<Vec<&str>>();
    match seed_input_type {
        SeedInputType::Individual => min_location(IndividualSeeds::new(groups[0]), groups),
        SeedInputType::Range => min_location(RangeSeeds::new(groups[0]), groups),
    }
}

fn min_location(mut seeds: impl SeedRetriever, groups: Vec<&str>) -> u64 {
    let mut min_location = u64::MAX;
    let garden_mappings: Vec<GardenMapping> = vec![
        GardenMapping::new(GardenMappingType::SeedToSoil, groups[1]),
        GardenMapping::new(GardenMappingType::SoilToFertilizer, groups[2]),
        GardenMapping::new(GardenMappingType::FertilizerToWater, groups[3]),
        GardenMapping::new(GardenMappingType::WaterToLight, groups[4]),
        GardenMapping::new(GardenMappingType::LightToTemperature, groups[5]),
        GardenMapping::new(GardenMappingType::TemperatureToHumidity, groups[6]),
        GardenMapping::new(GardenMappingType::HumidityToLocation, groups[7]),
    ];

    while let Some(seed_number) = seeds.next() {
        let location = garden_mappings
            .iter()
            .fold(seed_number, |source, garden_mapping| {
                garden_mapping.to_destination(source)
            });

        min_location = min(location, min_location);
    }

    min_location
}

enum SeedInputType {
    Individual,
    Range,
}

trait SeedRetriever {
    fn next(&mut self) -> Option<u64>;
}

#[derive(Debug)]
struct IndividualSeeds {
    seed_numbers: Vec<u64>,
    index: usize,
}

impl IndividualSeeds {
    fn new(data: &str) -> Self {
        Self {
            seed_numbers: split_seed_line_to_numbers(data),
            index: 0,
        }
    }
}

impl SeedRetriever for IndividualSeeds {
    fn next(&mut self) -> Option<u64> {
        match self.seed_numbers.get(self.index) {
            Some(seed_number) => {
                self.index += 1;
                Some(*seed_number)
            }
            None => None,
        }
    }
}

struct RangeSeeds {
    seed_ranges: Vec<Vec<u64>>,
    index: usize,
    value_offset: u64,
}

impl RangeSeeds {
    fn new(data: &str) -> Self {
        let numbers = split_seed_line_to_numbers(data);
        let seed_ranges: Vec<Vec<u64>> = numbers
            .chunks(2)
            .map(|chunk| {
                let start = *chunk.first().unwrap();
                let end = *chunk.first().unwrap() + *chunk.last().unwrap();
                vec![start, end]
            })
            .collect();

        Self {
            seed_ranges,
            index: 0,
            value_offset: 0,
        }
    }
}

impl SeedRetriever for RangeSeeds {
    fn next(&mut self) -> Option<u64> {
        if self.index > self.seed_ranges.len() - 1 {
            None
        } else {
            let current_range = self.seed_ranges.get(self.index);
            match current_range {
                None => {
                    self.index += 1;
                    self.value_offset = 0;
                    let next_range = self.seed_ranges.get(self.index).unwrap();
                    let start = *next_range.first().unwrap();
                    Some(start)
                }
                Some(seed_range) => {
                    let start = *seed_range.first().unwrap();
                    let end = *seed_range.last().unwrap();
                    let next_seed_value = start + self.value_offset;

                    return if next_seed_value < end {
                        self.value_offset += 1;
                        Some(next_seed_value)
                    } else if next_seed_value == end {
                        self.index += 1;
                        Some(end)
                    } else {
                        self.value_offset = 0;
                        let next_range = self.seed_ranges.get(self.index).unwrap();
                        let start = *next_range.first().unwrap();
                        Some(start)
                    };
                }
            }
        }
    }
}

fn split_seed_line_to_numbers(data: &str) -> Vec<u64> {
    data.split_whitespace()
        .filter(|value| *value != "seeds:")
        .map(|value| value.parse::<u64>().unwrap())
        .collect()
}

#[derive(Debug, Eq, PartialEq, Hash)]
enum GardenMappingType {
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

#[derive(Debug)]
struct GardenRange {
    destination_start: u64,
    source_start: u64,
    length: u64,
}

#[derive(Debug)]
struct GardenMapping {
    _mapping_type: GardenMappingType,
    ranges: Vec<GardenRange>,
}

impl GardenMapping {
    fn new(mapping_type: GardenMappingType, data: &str) -> Self {
        let lines = data.split('\n').collect::<Vec<&str>>();
        let ranges = lines[1..]
            .iter()
            .map(|line| line.split_whitespace().collect::<Vec<&str>>())
            .map(|line| {
                line.iter()
                    .map(|value| value.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>()
            })
            .map(|line| GardenRange {
                destination_start: line[0],
                source_start: line[1],
                length: line[2],
            })
            .collect::<Vec<GardenRange>>();

        Self {
            _mapping_type: mapping_type,
            ranges,
        }
    }

    fn to_destination(&self, source_number: u64) -> u64 {
        let target_range = self
            .ranges
            .iter()
            .filter(|range| {
                range.source_start <= source_number
                    && source_number < range.source_start + range.length
            })
            .collect::<Vec<&GardenRange>>();

        let destination = match &target_range.len() {
            0 => source_number, // no mapping, so source = destination
            1 => {
                let target_range = target_range.first().unwrap();
                source_number - target_range.source_start + target_range.destination_start
            }
            _too_many => panic!(
                "There should be at most one range resolved, but got multiple: {target_range:#?}"
            ),
        };

        destination
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example_input() {
        assert_eq!(
            run(
                &FileLoader::load("05", &TaskType::Example),
                SeedInputType::Individual
            ),
            35
        );
    }

    #[test]
    fn part_one_puzzle_input() {
        assert_eq!(
            run(
                &FileLoader::load("05", &TaskType::Puzzle),
                SeedInputType::Individual
            ),
            340_994_526
        );
    }

    // #[ignore]
    #[test]
    fn part_two_example_input() {
        assert_eq!(
            run(
                &FileLoader::load("05", &TaskType::Example),
                SeedInputType::Range
            ),
            46
        );
    }

    // #[ignore]
    #[test]
    fn part_two_puzzle_input() {
        assert_eq!(
            run(
                &FileLoader::load("05", &TaskType::Puzzle),
                SeedInputType::Range
            ),
            5_489_610
        );
    }
}
