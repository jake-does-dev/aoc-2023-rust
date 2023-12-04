use crate::{DayRunner, FileLoader, Part, TaskType};

#[derive(Debug)]
pub struct Day01;

impl DayRunner for Day01 {
    fn run(part: Part) {
        let result = match part {
            Part::PartOne => run(
                &FileLoader::load_with_infix("01", "part1", &TaskType::Puzzle),
                &WordReplacement::Disabled,
            ),
            Part::PartTwo => run(
                &FileLoader::load_with_infix("01", "part2", &TaskType::Puzzle),
                &WordReplacement::Enabled,
            ),
        };

        Self::report_result(Self, part, result);
    }
}

enum WordReplacement {
    Enabled,
    Disabled,
}

fn run(data: &str, word_replacement: &WordReplacement) -> u16 {
    data.lines()
        .map(|line| match word_replacement {
            WordReplacement::Enabled => replace_words_with_digits(line),
            WordReplacement::Disabled => line.to_string(),
        })
        .map(|line| {
            let digits = line
                .chars()
                .filter(char::is_ascii_digit)
                .collect::<Vec<char>>();

            format!(
                "{}{}",
                digits.first().expect("First digit should exist"),
                digits.last().expect("Last digit should exist")
            )
            .parse::<u16>()
            .expect("The only two digits retrieved should be valid")
        })
        .sum::<u16>()
}

/// Instead of having to parse words separately, instead consider how we can represent words
/// with digits inside the string.
///
/// To respect double borrowing of letters in some lines, we can replace the written form of
/// a number with the beginning character, the digit representation, and the final character.
///
/// In particular, consider `eightwo`.
/// Performing the replacement rules:
///   `eightwo` -> `eight2o` -> `e8t2o`
/// and work on the digits, as in `part_one`.
///
/// Note that we cannot perform a one-sided replacement, as some combinations of letter
/// borrowing are lost.
///
/// If you just kept the ending written representation, then the replacement rules
///   `eight` -> `8t` and `nine` -> `9e`
/// will fail when replacing for the string `nineight` (when replacing in increasing value),
/// as the first contentful replacement yields `nin8t`, and you've lost the `eight`.
///
/// Similarly, if you just kept the beginning written representation, then the replacement rules
///   `three` -> `t3` and `eight` -> `e8`
/// will fail when replacing for the string `threeight` (when replacing in increasing value),
/// as the first contentful replacement yields `t3ight`, and you've lost the `eight`.
fn replace_words_with_digits(line: &str) -> String {
    line.replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day01_part_one_example_input() {
        assert_eq!(
            run(
                &FileLoader::load_with_infix("01", "part1", &TaskType::Example),
                &WordReplacement::Disabled
            ),
            142
        );
    }

    #[test]
    fn day01_part_one_puzzle_input() {
        assert_eq!(
            run(
                &FileLoader::load_with_infix("01", "part1", &TaskType::Puzzle),
                &WordReplacement::Disabled
            ),
            54632
        );
    }

    #[test]
    fn day01_part_two_example_input() {
        assert_eq!(
            run(
                &FileLoader::load_with_infix("01", "part2", &TaskType::Example),
                &WordReplacement::Enabled
            ),
            281
        );
    }

    #[test]
    fn day01_part_two_puzzle_input() {
        assert_eq!(
            run(
                &FileLoader::load_with_infix("01", "part2", &TaskType::Puzzle),
                &WordReplacement::Enabled
            ),
            54019
        );
    }
}
