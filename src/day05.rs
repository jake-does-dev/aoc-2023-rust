use std::cmp::min;
use std::fmt::Debug;

use crate::{DayRunner, FileLoader, Part, TaskType};

#[derive(Debug)]
pub struct Day05;

impl DayRunner for Day05 {
    fn run(part: Part) {
        let result = match part {
            Part::PartOne => part_one(&FileLoader::load("05", &TaskType::Puzzle)),
            Part::PartTwo => part_two(&FileLoader::load("05", &TaskType::Puzzle)),
        };

        Self::report_result(Self, part, result);
    }
}

fn part_one(data: &str) -> u64 {
    let garden = Garden::new(data);
    garden
        .seed_numbers
        .iter()
        .map(|seed_number| garden.find_seed_location(*seed_number))
        .min()
        .unwrap()
}

fn part_two(data: &str) -> u64 {
    let garden = Garden::new(data);
    let mut iter = garden.seed_numbers.iter();

    let mut min_location = u64::MAX;

    while let Some(start_seed) = iter.next() {
        let amount = *iter.next().unwrap();
        let ending_seed = *start_seed + amount;

        let mut current_seed = *start_seed;
        while current_seed < ending_seed {
            let location = garden.find_seed_location(current_seed);
            min_location = min(min_location, location);
            current_seed += 1;
        }
    }

    min_location
}

struct Garden {
    seed_numbers: Vec<u64>,
    mappings: Vec<Vec<GardenRange>>,
}

#[derive(Debug)]
struct GardenRange {
    destination_start: u64,
    source_start: u64,
    amount: u64,
}

impl Garden {
    fn new(data: &str) -> Self {
        let groups = data.split("\n\n").collect::<Vec<&str>>();
        let seed_numbers = groups[0]
            .split_whitespace()
            .filter(|value| *value != "seeds:")
            .map(|value| value.parse::<u64>().unwrap())
            .collect();

        let mappings = groups[1..]
            .iter()
            .map(|group| {
                let lines = group.split('\n').collect::<Vec<&str>>();
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
                        amount: line[2],
                    })
                    .collect::<Vec<GardenRange>>();

                ranges
            })
            .collect::<Vec<Vec<GardenRange>>>();

        Self {
            seed_numbers,
            mappings,
        }
    }

    fn find_seed_location(&self, initial_number: u64) -> u64 {
        let mut number = initial_number;

        for mapping in &self.mappings {
            for range in mapping {
                if range.source_start <= number && number < range.source_start + range.amount {
                    number = (number - range.source_start) + range.destination_start;
                    break;
                }
            }
        }

        number
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example_input() {
        assert_eq!(part_one(&FileLoader::load("05", &TaskType::Example)), 35);
    }

    #[test]
    fn part_one_puzzle_input() {
        assert_eq!(
            part_one(&FileLoader::load("05", &TaskType::Puzzle)),
            340_994_526
        );
    }

    #[test]
    fn part_two_example_input() {
        assert_eq!(part_two(&FileLoader::load("05", &TaskType::Example)), 46);
    }

    #[ignore]
    #[test]
    fn part_two_puzzle_input() {
        assert_eq!(
            part_two(&FileLoader::load("05", &TaskType::Puzzle)),
            52_210_644
        );
    }
}
