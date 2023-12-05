use std::collections::HashMap;
use std::fmt::Debug;

use crate::{DayRunner, FileLoader, Part, TaskType};

#[derive(Debug)]
pub struct Day04;

impl DayRunner for Day04 {
    fn run(part: Part) {
        let result = match part {
            Part::PartOne => part_one(&FileLoader::load("04", &TaskType::Puzzle)),
            Part::PartTwo => part_two(&FileLoader::load("04", &TaskType::Puzzle)),
        };

        Self::report_result(Self, part, result);
    }
}

#[derive(Debug)]
struct Card {
    id: usize,
    numbers: Vec<u32>,
    drawn_numbers: Vec<u32>,
}

impl Card {
    fn new(line: &str) -> Self {
        fn number_line_as_vec(number_line: &str) -> Vec<u32> {
            let numbers = number_line.split_whitespace().collect::<Vec<_>>();

            numbers
                .iter()
                .map(|number| number.parse::<u32>().unwrap())
                .collect()
        }

        let (card_id_and_numbers, drawn_numbers) = line.trim().split_once('|').unwrap();
        let drawn_numbers: Vec<u32> = number_line_as_vec(drawn_numbers);

        let (card_id, numbers) = card_id_and_numbers.trim().split_once(':').unwrap();

        let id = card_id.split_whitespace().last().unwrap();
        let id = id.parse::<usize>().unwrap();

        let numbers = number_line_as_vec(numbers);

        Self {
            id,
            numbers,
            drawn_numbers,
        }
    }

    fn points(&self) -> u32 {
        match self.winning_numbers().len() {
            0 => 0,
            count => 2_u32.pow((count - 1).try_into().unwrap()),
        }
    }

    fn winning_numbers(&self) -> Vec<u32> {
        self.numbers
            .iter()
            .filter(|number| self.drawn_numbers.contains(number))
            .copied()
            .collect()
    }
}

fn part_one(data: &str) -> u32 {
    data.lines().map(Card::new).map(|card| card.points()).sum()
}

fn part_two(data: &str) -> u32 {
    let cards = data.lines().map(Card::new).collect::<Vec<Card>>();
    let mut card_id_to_count = cards.iter().fold(HashMap::new(), |mut map, card| {
        map.entry(card.id).or_insert(1u32);
        map
    });

    for card in &cards {
        match card.winning_numbers().len() {
            0 => {}
            number_of_winners => {
                (card.id + 1..card.id + 1 + number_of_winners).for_each(|card_id| {
                    *card_id_to_count.get_mut(&card_id).unwrap() += card_id_to_count[&card.id];
                });
            }
        }
    }

    card_id_to_count.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example_input() {
        assert_eq!(part_one(&FileLoader::load("04", &TaskType::Example)), 13);
    }

    #[test]
    fn part_one_puzzle_input() {
        assert_eq!(part_one(&FileLoader::load("04", &TaskType::Puzzle)), 20855);
    }

    #[test]
    fn part_two_example_input() {
        assert_eq!(part_two(&FileLoader::load("04", &TaskType::Example)), 30);
    }

    #[test]
    fn part_two_puzzle_input() {
        assert_eq!(
            part_two(&FileLoader::load("04", &TaskType::Puzzle)),
            5_489_600
        );
    }
}
