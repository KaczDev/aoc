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
    part: Option<Part>,
}

#[derive(ValueEnum, Clone, Debug, Copy)]
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
        match cli.part {
            Some(part) => match part {
                Part::A => wrapper(|input| solution.part_a(input), part, &file),
                Part::B => wrapper(|input| solution.part_b(input), part, &file),
            },
            None => {
                wrapper(|input| solution.part_a(input), Part::A, &file);
                wrapper(|input| solution.part_b(input), Part::B, &file);
            }
        };
    }
    Ok(())
}

fn wrapper<F>(sol: F, part: Part, file_name: &str)
where
    F: FnOnce(&str) -> Result<String>,
{
    let now = Instant::now();
    let res = sol(file_name);
    match res {
        Ok(res) => {
            let time_passed = now.elapsed().as_millis();
            println!("Part {:?} - {}: {} in {}ms", part, file_name, res, time_passed);
        }
        Err(err) => println!("{}", err),
    };
}

fn find_files(year: u32, day: u32) -> Vec<String> {
    let search_string = format!("./inputs/{year}/day{day}[!0-9]*");
    glob(&search_string)
        .expect("glob search string is invalid")
        .filter_map(Result::ok)
        .map(|buf| buf.into_string().unwrap())
        .collect_vec()
}
