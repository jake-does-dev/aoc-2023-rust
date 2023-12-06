use aoc_2023_rust::day01::Day01;
use aoc_2023_rust::day02::Day02;
use aoc_2023_rust::day03::Day03;
use aoc_2023_rust::day04::Day04;
use aoc_2023_rust::day05::Day05;
use aoc_2023_rust::day06::Day06;
use aoc_2023_rust::{DayRunner, Part};

fn main() {
    Day01::run(Part::PartOne);
    Day01::run(Part::PartTwo);
    Day02::run(Part::PartOne);
    Day02::run(Part::PartTwo);
    Day03::run(Part::PartOne);
    Day03::run(Part::PartTwo);
    Day04::run(Part::PartOne);
    Day04::run(Part::PartTwo);
    Day05::run(Part::PartOne);
    // Day05::run(Part::PartTwo);
    Day06::run(Part::PartOne);
    Day06::run(Part::PartTwo);
}
