use regex::Regex;
use std::fmt::{Debug, Formatter};

use crate::{DayRunner, Part};

pub struct Day03;

impl DayRunner for Day03 {
    fn run(part: Part) {
        let result = match part {
            Part::PartOne => part_one(include_str!("inputs/day03_puzzle.txt")),
            Part::PartTwo => part_two(include_str!("inputs/day03_puzzle.txt")),
        };

        Self::report_result("day01", part, result);
    }
}

fn numbers_pattern() -> Regex {
    Regex::new(r"(\d+)+").unwrap()
}

fn symbols_pattern() -> Regex {
    Regex::new(r"[^\d.\n]").unwrap()
}

// #[derive(Debug)]
struct Number {
    row_index: usize,
    column_start_index: usize,
    column_end_index: usize,
    value: u32,
    visited: bool,
}

impl Debug for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} @ ({},{}-{})",
            self.value, self.row_index, self.column_start_index, self.column_end_index
        )
    }
}

impl Number {
    const fn new(
        row_index: usize,
        column_start_index: usize,
        column_end_index: usize,
        value: u32,
    ) -> Self {
        Self {
            row_index,
            column_start_index,
            column_end_index,
            value,
            visited: false,
        }
    }

    fn visit(&mut self) {
        self.visited = true;
    }
}

#[derive(Debug)]
struct Symbol {
    row_index: usize,
    column_index: usize,
    value: String,
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

        // let smallest_row_index = if number.row_index == 0 {
        //     0
        // } else {
        //     number.row_index - 1
        // };
        // let largest_row_index = number.row_index + 1;
        // let smallest_column_index = if number.column_start_index == 0 {
        //     0
        // } else {
        //     number.column_start_index - 1
        // };
        // let largest_column_index = number.column_end_index + 1;
        //
        // if self.row_index >= smallest_row_index
        //     && self.row_index <= largest_row_index
        //     && self.column_index >= smallest_column_index
        //     && self.column_index <= largest_column_index
        // {
        //     number.visit();
        // }
    }
}

fn part_one(data: &str) -> u32 {
    let lines: Vec<&str> = data.lines().collect();

    let mut numbers: Vec<Number> = vec![];
    let mut symbols: Vec<Symbol> = vec![];

    lines.iter().enumerate().for_each(|(row_index, line)| {
        numbers_pattern().captures_iter(line).for_each(|captures| {
            let number = captures.get(0).unwrap();
            let (column_start_index, column_end_index, value) = (
                number.start(),
                number.end(),
                number.as_str().parse::<u32>().unwrap(),
            );
            numbers.push(Number::new(
                row_index,
                column_start_index,
                column_end_index,
                value,
            ));
        });

        symbols_pattern().captures_iter(line).for_each(|captures| {
            let symbol = captures.get(0).unwrap();
            let (column_index, value) = (symbol.start(), symbol.as_str().to_string());
            symbols.push(Symbol {
                row_index,
                column_index,
                value,
            });
        });
    });

    symbols.iter().for_each(|symbol| {
        let visited_numbers = symbol.visit(&mut numbers);
        println!(
            "The symbol {} at position ({},{}) has visited the following numbers: {:?}",
            symbol.value, symbol.row_index, symbol.column_index, visited_numbers
        );
    });

    // let not_visited: Vec<&Number> = numbers.iter().filter(|number| !number.visited).collect();

    // println!("not_visited: {not_visited:#?}");

    numbers
        .iter()
        .filter(|number| number.visited)
        .map(|number| {
            // println!("visited number: {number:?}");
            number.value
        })
        .sum()
}

fn part_two(_data: &str) -> u32 {
    2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example_input() {
        assert_eq!(part_one(include_str!("inputs/day03_example.txt"),), 4361);
    }

    #[test]
    fn part_one_puzzle_input() {
        assert_eq!(part_one(include_str!("inputs/day03_puzzle.txt"),), 2);
    }

    #[test]
    #[ignore]
    fn part_two_example_input() {
        assert_eq!(part_two(include_str!("inputs/day03_example.txt"),), 2286);
    }

    #[test]
    #[ignore]
    fn part_two_puzzle_input() {
        assert_eq!(part_two(include_str!("inputs/day03_puzzle.txt"),), 63542);
    }
}
