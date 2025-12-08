use anyhow::Result;
use std::{fs, time::Instant};

type Input<T> = Vec<Vec<T>>;

fn solve(file_name: &str) -> Result<usize> {
    let mut res = 0;
    let input: Input<char> = read_input(file_name)?;
    println!("{:?}", input);

    Ok(res)
}

fn solve_2(file_name: &str) -> Result<usize> {
    let mut res = 0;
    let input: Input<char> = read_input(file_name)?;
    println!("{:?}", input);

    Ok(res)
}

fn read_input(file_name: &str) -> Result<Input<char>> {
    Ok(fs::read_to_string(file_name)?
        .lines()
        .map(|line| line.chars().collect())
        .collect())
}

fn main() -> Result<()> {
    let files = ["./inputs/day12.test", "./inputs/day12.prod"];
    println!("# Part 1");
    for file in files {
        let now = Instant::now();
        let res = solve(file)?;
        println!("{}: {} in {}ms", file, res, now.elapsed().as_millis());
    }
    println!("# Part 2");
    for file in files {
        let now = Instant::now();
        let res = solve_2(file)?;
        println!("{}: {} in {}ms", file, res, now.elapsed().as_millis());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_1() {
        let file = "./inputs/day12.test";
        let result = solve(file).unwrap();
        let expected = 140;
        assert_eq!(result, expected)
    }

    #[test]
    fn part_2() {
        let file = "./inputs/day12.test";
        let result = solve_2(file).unwrap();
        let expected = 140;
        assert_eq!(result, expected)
    }
}
