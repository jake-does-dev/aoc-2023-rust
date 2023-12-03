use std::fmt::Debug;

use regex::{Captures, Regex};

use crate::{DayRunner, FileLoader, Part, TaskType};

#[derive(Debug)]
pub struct Day03;

impl DayRunner for Day03 {
    fn run(part: Part) {
        let result = match part {
            Part::PartOne => part_one(&FileLoader::load("03", &TaskType::Puzzle)),
            Part::PartTwo => part_two(&FileLoader::load("03", &TaskType::Puzzle)),
        };

        Self::report_result(Self, part, result);
    }
}

struct Number {
    row_index: usize,
    column_start_index: usize,
    column_end_index: usize,
    value: u32,
    visited: bool,
}

impl Number {
    fn visit(&mut self) {
        self.visited = true;
    }
}

#[derive(Debug)]
struct Symbol {
    row_index: usize,
    column_index: usize,
}

impl Symbol {
    fn visit<'a>(&'a self, numbers: &'a mut [Number]) -> Vec<&mut Number> {
        let (row_index, column_index) = (self.row_index, self.column_index);
        let mut visited_numbers: Vec<&mut Number> = vec![];

        let neighbour_indices = [
            (row_index - 1, column_index - 1),
            (row_index - 1, column_index),
            (row_index - 1, column_index + 1),
            (row_index, column_index - 1),
            (row_index, column_index + 1),
            (row_index + 1, column_index - 1),
            (row_index + 1, column_index),
            (row_index + 1, column_index + 1),
        ];

        for number in numbers.iter_mut() {
            for (adjacent_row_index, adjacent_column_index) in neighbour_indices {
                if number.row_index == adjacent_row_index
                    && (number.column_start_index..=number.column_end_index)
                        .contains(&adjacent_column_index)
                {
                    number.visit();
                    visited_numbers.push(number);
                    break;
                }
            }
        }

        visited_numbers
    }
}

fn part_one(data: &str) -> u32 {
    let mut numbers = extract_numbers(data, &Regex::new(r"(\d+)+").unwrap());
    let symbols = extract_symbols(data, &Regex::new(r"[^\d.\n]").unwrap());

    for symbol in &symbols {
        symbol.visit(&mut numbers);
    }

    numbers
        .iter()
        .filter(|number| number.visited)
        .map(|number| number.value)
        .sum()
}

#[allow(clippy::trivial_regex)]
fn part_two(data: &str) -> u32 {
    let mut numbers = extract_numbers(data, &Regex::new(r"(\d+)+").unwrap());
    let symbols = extract_symbols(data, &Regex::new(r"[*]").unwrap());

    let mut gear_part_ratios: Vec<u32> = vec![];

    for symbol in &symbols {
        let visited_numbers = symbol.visit(&mut numbers);
        if visited_numbers.len() == 2 {
            gear_part_ratios.push(
                visited_numbers.first().unwrap().value * visited_numbers.last().unwrap().value,
            );
        }
    }

    gear_part_ratios.iter().sum()
}

fn extract_numbers(data: &str, numbers_pattern: &Regex) -> Vec<Number> {
    data.lines()
        .enumerate()
        .flat_map(|(row_index, line)| {
            numbers_pattern
                .captures_iter(line)
                .map(|captures| transform_matching_numbers(row_index, &captures))
                .collect::<Vec<Number>>()
        })
        .collect::<Vec<Number>>()
}

fn transform_matching_numbers(row_index: usize, captures: &Captures) -> Number {
    let number = captures.get(0).unwrap();
    let (column_start_index, column_end_index, value) = (
        number.start(),
        number.end(),
        number.as_str().parse::<u32>().unwrap(),
    );
    Number {
        row_index,
        column_start_index,
        column_end_index: column_end_index - 1, //the Regex crate has the end index as exclusive
        value,
        visited: false,
    }
}

fn extract_symbols(data: &str, symbols_pattern: &Regex) -> Vec<Symbol> {
    data.lines()
        .enumerate()
        .flat_map(|(row_index, line)| {
            symbols_pattern
                .captures_iter(line)
                .map(|captures| transform_matching_symbols(row_index, &captures))
                .collect::<Vec<Symbol>>()
        })
        .collect::<Vec<Symbol>>()
}

fn transform_matching_symbols(row_index: usize, captures: &Captures) -> Symbol {
    let symbol = captures.get(0).unwrap();
    let column_index = symbol.start();
    Symbol {
        row_index,
        column_index,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day03_part_one_example_input() {
        assert_eq!(part_one(&FileLoader::load("03", &TaskType::Example)), 4361);
    }

    #[test]
    fn day03_part_one_puzzle_input() {
        assert_eq!(
            part_one(&FileLoader::load("03", &TaskType::Puzzle)),
            556_057
        );
    }

    #[test]
    fn day03_part_two_example_input() {
        assert_eq!(
            part_two(&FileLoader::load("03", &TaskType::Example)),
            467_835
        );
    }

    #[test]
    fn day03_part_two_puzzle_input() {
        assert_eq!(
            part_two(&FileLoader::load("03", &TaskType::Puzzle)),
            82_824_352
        );
    }
}
