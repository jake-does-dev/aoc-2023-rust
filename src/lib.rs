use std::fmt::Display;
use std::fs;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;

#[derive(Debug)]
pub enum Part {
    PartOne,
    PartTwo,
}

pub trait DayRunner {
    fn run(part: Part);

    fn report_result(day_impl: impl DayRunner + std::fmt::Debug, part: Part, result: impl Display) {
        println!("{day_impl:?} {part:?} yields result: {result}");
    }
}

pub enum TaskType {
    Example,
    Puzzle,
}
pub struct FileLoader;
impl FileLoader {
    fn load(day: &str, task_type: &TaskType) -> String {
        Self::load_data(day, "", task_type)
    }

    fn load_with_infix(day: &str, infix: &str, task_type: &TaskType) -> String {
        Self::load_data(day, infix, task_type)
    }

    fn load_data(day: &str, infix: &str, task_type: &TaskType) -> String {
        let file_name_ending = match task_type {
            TaskType::Example => "example",
            TaskType::Puzzle => "puzzle",
        };

        let file_path = match infix {
            "" => format!("src/inputs/day{day}_{file_name_ending}.txt"),
            infix_value => {
                format!("src/inputs/day{day}_{infix_value}_{file_name_ending}.txt")
            }
        };

        fs::read_to_string(file_path).unwrap()
    }
}
