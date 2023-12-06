use std::fmt::Debug;
use std::iter::zip;

use crate::{DayRunner, FileLoader, Part, TaskType};

#[derive(Debug)]
pub struct Day06;

impl DayRunner for Day06 {
    fn run(part: Part) {
        let result = match part {
            Part::PartOne => part_one(&FileLoader::load("06", &TaskType::Puzzle)),
            Part::PartTwo => part_two(&FileLoader::load("06", &TaskType::Puzzle)),
        };

        Self::report_result(Self, part, result);
    }
}

fn part_one(data: &str) -> u64 {
    Race::create_individual_races(data)
        .iter()
        .map(Race::number_of_winning_strategies)
        .product::<u64>()
}

fn part_two(data: &str) -> u64 {
    Race::create_merged_race(data).number_of_winning_strategies()
}

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn create_individual_races(data: &str) -> Vec<Self> {
        let mut iter = data.lines();
        let times = Self::get_values(iter.next().unwrap());
        let distances = Self::get_values(iter.next().unwrap());

        zip(times, distances)
            .map(|(time, distance)| Self { time, distance })
            .collect::<Vec<Self>>()
    }

    fn create_merged_race(data: &str) -> Self {
        fn merge_numbers(vector: &[u64]) -> u64 {
            vector
                .iter()
                .fold(String::new(), |total, part| format!("{total}{part}"))
                .parse::<u64>()
                .unwrap()
        }

        let mut iter = data.lines();
        let times = Self::get_values(iter.next().unwrap());
        let distances = Self::get_values(iter.next().unwrap());

        Self {
            time: merge_numbers(&times),
            distance: merge_numbers(&distances),
        }
    }

    fn get_values(line: &str) -> Vec<u64> {
        let split = line.split_whitespace().collect::<Vec<_>>();
        split[1..]
            .iter()
            .map(|value| value.parse::<u64>().unwrap())
            .collect::<Vec<u64>>()
    }

    fn number_of_winning_strategies(&self) -> u64 {
        (1..=self.time)
            .filter(|time| {
                let speed = *time;
                let time_remaining = self.time - speed;
                let distance_travelled = speed * time_remaining;

                distance_travelled > self.distance
            })
            .count()
            .try_into()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example_input() {
        assert_eq!(part_one(&FileLoader::load("06", &TaskType::Example)), 288);
    }

    #[test]
    fn part_one_puzzle_input() {
        assert_eq!(
            part_one(&FileLoader::load("06", &TaskType::Puzzle)),
            633_080
        );
    }

    #[test]
    fn part_two_example_input() {
        assert_eq!(part_two(&FileLoader::load("06", &TaskType::Example)), 71503);
    }

    #[test]
    fn part_two_puzzle_input() {
        assert_eq!(
            part_two(&FileLoader::load("06", &TaskType::Puzzle)),
            20_048_741
        );
    }
}
