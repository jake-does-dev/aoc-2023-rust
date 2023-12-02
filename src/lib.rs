use std::fmt::Display;

pub mod day01;
pub mod day02;

#[derive(Debug)]
pub enum Part {
    PartOne,
    PartTwo,
}

pub trait DayRunner {
    fn run(part: Part);

    fn report_result(day: &str, part: Part, result: impl Display) {
        println!("{day} {part:?} yields result: {result}");
    }
}
