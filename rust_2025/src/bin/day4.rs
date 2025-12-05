use anyhow::Result;
use aoc_utils::grid::{self, will_be_oob, Grid};
use std::{fs, time::Instant};

type Input<T> = Vec<Vec<T>>;

fn solve_1(file_name: &str) -> Result<usize> {
    let mut res = 0;
    let input: Grid<char> = read_input(file_name)?;
    for (r, row) in input.iter().enumerate() {
        for (c, char) in row.iter().enumerate() {
            if *char == '@' {
                let mut count = 0;
                for dir in grid::Direction::all_diagonals() {
                    if count >= 4 {
                        break;
                    }
                    if !will_be_oob(&input, (r, c), dir) {
                        let p = dir.move_point((r, c));
                        if *grid::get_element(&input, p) == '@' {
                            count += 1;
                        }
                    }
                }
                if count < 4 {
                    res += 1;
                }
            }
        }
    }

    Ok(res)
}

fn solve_2(file_name: &str) -> Result<usize> {
    let mut res = 0;
    let mut input: Grid<char> = read_input(file_name)?;
    loop {
        let mut removed = 0;

        for (r, row) in input.clone().iter().enumerate() {
            for (c, char) in row.iter().enumerate() {
                if *char == '@' {
                    let mut count = 0;
                    for dir in grid::Direction::all_diagonals() {
                        if count >= 4 {
                            break;
                        }
                        if !will_be_oob(&input, (r, c), dir) {
                            let p = dir.move_point((r, c));
                            if *grid::get_element(&input, p) == '@' {
                                count += 1;
                            }
                        }
                    }
                    if count < 4 {
                        input[r][c] = '.';
                        removed += 1;
                    }
                }
            }
        }
        res += removed;
        if removed == 0 {
            break;
        }
    }

    Ok(res)
}

fn read_input(file_name: &str) -> Result<Input<char>> {
    Ok(fs::read_to_string(file_name)?
        .lines()
        .map(|line| line.chars().collect())
        .collect())
}

fn main() -> Result<()> {
    let files = ["./inputs/day4.test", "./inputs/day4.prod"];
    println!("# Part 1");
    for file in files {
        let now = Instant::now();
        let res = solve_1(file)?;
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
    fn test_part1() {
        let file = "./inputs/day4.test";
        let result = solve_1(file).unwrap();
        let expected = 13;
        assert_eq!(result, expected)
    }
    #[test]
    fn test_part2() {
        let file = "./inputs/day4.test";
        let result = solve_2(file).unwrap();
        let expected = 43;
        assert_eq!(result, expected)
    }
}
