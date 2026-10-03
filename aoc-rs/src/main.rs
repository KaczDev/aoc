use anyhow::Result;
use aoc_rs::Solution;
use clap::{Parser, ValueEnum};
use glob::glob;
use itertools::Itertools;
use std::time::Instant;

/// Advent of Code solution runner.
#[derive(Parser, Debug)]
#[command(name = "aoc-rs", version, about, long_about = None)]
struct Cli {
    /// Puzzle year, e.g. 2025
    year: u32,

    /// Puzzle day, e.g. 1
    day: u32,

    /// Puzzle part
    part: Part,
}

#[derive(ValueEnum, Clone, Debug)]
enum Part {
    A,
    B,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let solution = aoc_rs::get_solution(cli.year, cli.day)?;
    run(&cli, solution)
}

fn run(cli: &Cli, solution: Box<dyn Solution>) -> Result<()> {
    let files = find_files(cli.year, cli.day);
    if files.is_empty() {
        return Err(anyhow::anyhow!(
            "No files found for {}/{}",
            cli.year,
            cli.day
        ));
    }
    for file in files {
        let now = Instant::now();
        let res = match cli.part {
            Part::A => solution.part_a(&file),
            Part::B => solution.part_b(&file),
        };
        let time_passed = now.elapsed().as_millis();
        match res {
            Ok(res) => {
                println!("{}: {} in {}ms", file, res, time_passed);
            }
            Err(err) => println!("{}", err),
        }
    }
    Ok(())
}

fn find_files(year: u32, day: u32) -> Vec<String> {
    let search_string = format!("./inputs/{year}/day{day}*");
    glob(&search_string)
        .expect("glob search string is invalid")
        .filter_map(Result::ok)
        .map(|buf| buf.into_string().unwrap())
        .collect_vec()
}
