pub mod grid;
pub mod reader;
pub mod solution;
pub mod years;

use anyhow::{Result, bail};
pub use solution::Solution;

/// Returns the `Solution` implementation for the given year & day,
/// or `None` if that day hasn't been solved yet.
pub fn get_solution(year: u32, day: u32) -> Result<Box<dyn Solution>> {
    let solution = match year {
        2025 => get_2025_solution(day),
        _ => bail!("Year {year} isn't wired up to the CLI yet. Only 2025 is implemented."),
    };
    solution.ok_or_else(|| anyhow::anyhow!("Day {day} of {year} hasn't been solved yet"))
}

fn get_2025_solution(day: u32) -> Option<Box<dyn Solution>> {
    match day {
        1 => Some(Box::new(years::y2025::day1::Day1)),
        2 => Some(Box::new(years::y2025::day2::Day2)),
        3 => Some(Box::new(years::y2025::day3::Day3)),
        4 => Some(Box::new(years::y2025::day4::Day4)),
        5 => Some(Box::new(years::y2025::day5::Day5)),
        6 => Some(Box::new(years::y2025::day6::Day6)),
        7 => Some(Box::new(years::y2025::day7::Day7)),
        8 => Some(Box::new(years::y2025::day8::Day8)),
        9 => Some(Box::new(years::y2025::day9::Day9)),
        10 => Some(Box::new(years::y2025::day10::Day10)),
        11 => Some(Box::new(years::y2025::day11::Day11)),
        12 => Some(Box::new(years::y2025::day12::Day12)),
        _ => None,
    }
}
