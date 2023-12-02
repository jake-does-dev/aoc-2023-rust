use crate::{DayRunner, Part};
use std::collections::HashMap;

pub struct Day02 {}

impl DayRunner for Day02 {
    fn run(part: Part) {
        let result = match part {
            Part::PartOne => part_one(include_str!("inputs/day02_puzzle.txt")),
            Part::PartTwo => part_two(include_str!("inputs/day02_puzzle.txt")),
        };

        Self::report_result("day01", part, result);
    }
}

fn part_one(data: &str) -> u32 {
    let red_max = 12;
    let green_max = 13;
    let blue_max = 14;

    data.lines()
        .map(Game::new)
        .map(MaxPullsGame::from)
        .filter(|game| {
            game.max_pulls.get(&CubeColor::Red).unwrap() <= &red_max
                && game.max_pulls.get(&CubeColor::Green).unwrap() <= &green_max
                && game.max_pulls.get(&CubeColor::Blue).unwrap() <= &blue_max
        })
        .map(|game| game.game_number)
        .sum()
}

fn part_two(data: &str) -> u32 {
    data.lines()
        .map(Game::new)
        .map(MaxPullsGame::from)
        .map(|game| {
            game.max_pulls.get(&CubeColor::Red).unwrap()
                * game.max_pulls.get(&CubeColor::Green).unwrap()
                * game.max_pulls.get(&CubeColor::Blue).unwrap()
        })
        .sum()
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
enum CubeColor {
    Red,
    Green,
    Blue,
}

struct Game {
    game_number: u32,
    pulls: Vec<HashMap<CubeColor, u32>>,
}

impl Game {
    fn new(game_line: &str) -> Self {
        let (game_info_str, cube_pulls_str) = game_line.split_once(':').unwrap();

        let (_game_string, game_number) = game_info_str.split_once(' ').unwrap();
        let game_number = game_number.parse::<u32>().unwrap();

        let pulls: Vec<HashMap<CubeColor, u32>> = cube_pulls_str
            .split(';')
            .map(str::trim)
            .map(|pulls| {
                let mut color_count: HashMap<CubeColor, u32> = HashMap::new();

                pulls.split(',').map(str::trim).for_each(|pull| {
                    let (count, color) = pull.split_once(' ').unwrap();
                    let count = count.parse::<u32>().unwrap();

                    match color {
                        "red" => color_count.entry(CubeColor::Red).or_insert(count),
                        "blue" => color_count.entry(CubeColor::Blue).or_insert(count),
                        "green" => color_count.entry(CubeColor::Green).or_insert(count),
                        _ => panic!("Unhandled color identified!"),
                    };
                });

                color_count
            })
            .collect();

        Self { game_number, pulls }
    }
}

struct MaxPullsGame {
    game_number: u32,
    max_pulls: HashMap<CubeColor, u32>,
}

#[allow(clippy::needless_pass_by_value)]
impl MaxPullsGame {
    fn from(base_game: Game) -> Self {
        let mut max_pulls: HashMap<CubeColor, u32> = HashMap::from([
            (CubeColor::Red, 0),
            (CubeColor::Green, 0),
            (CubeColor::Blue, 0),
        ]);

        base_game.pulls.iter().for_each(|pull| {
            for (key, value) in pull {
                if max_pulls.get(key).unwrap() < value {
                    max_pulls.insert(*key, *value);
                }
            }
        });

        Self {
            game_number: base_game.game_number,
            max_pulls,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example_input() {
        assert_eq!(part_one(include_str!("inputs/day02_example.txt"),), 8);
    }

    #[test]
    fn part_one_puzzle_input() {
        assert_eq!(part_one(include_str!("inputs/day02_puzzle.txt"),), 2268);
    }

    #[test]
    fn part_two_example_input() {
        assert_eq!(part_two(include_str!("inputs/day02_example.txt"),), 2286);
    }

    #[test]
    fn part_two_puzzle_input() {
        assert_eq!(part_two(include_str!("inputs/day02_puzzle.txt"),), 63542);
    }
}
