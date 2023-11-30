use crate::{DayRunner, Part};

pub struct Day01 {}

impl DayRunner for Day01 {
    fn run(part: Part) {
        let result = match part {
            Part::PartOne => part_one(include_str!("inputs/day01_puzzle.txt")),
            Part::PartTwo => part_two(include_str!("inputs/day01_puzzle.txt")),
        };

        Self::report_result("day01", part, result);
    }
}

fn part_one(_data: &str) -> i64 {
    // let mut _lines = _data.lines();
    1
}

fn part_two(_data: &str) -> i64 {
    // let mut _lines = _data.lines();
    2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example_input() {
        assert_eq!(part_one(include_str!("inputs/day01_example.txt")), 1);
    }

    #[test]
    fn part_one_puzzle_input() {
        assert_eq!(part_one(include_str!("inputs/day01_puzzle.txt")), 1);
    }

    #[test]
    fn part_two_example_input() {
        assert_eq!(part_two(include_str!("inputs/day01_example.txt")), 2);
    }

    #[test]
    fn part_two_puzzle_input() {
        assert_eq!(part_two(include_str!("inputs/day01_puzzle.txt")), 2);
    }
}
