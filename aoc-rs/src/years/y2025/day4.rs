use crate::Solution;
use crate::grid::{self, Grid, will_be_oob};
use anyhow::Result;
use std::fs;

type Input<T> = Vec<Vec<T>>;

fn read_input(file_name: &str) -> Result<Input<char>> {
    Ok(fs::read_to_string(file_name)?
        .lines()
        .map(|line| line.chars().collect())
        .collect())
}

pub struct Day4;

impl Solution for Day4 {
    fn part_a(&self, file_name: &str) -> Result<String> {
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

        Ok(format!("{}", res))
    }

    fn part_b(&self, file_name: &str) -> Result<String> {
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

        Ok(format!("{}", res))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let d4 = Day4;
        let file = "./inputs/day4.test";
        let result = d4.part_a(file).unwrap();
        let expected = String::from("13");
        assert_eq!(result, expected)
    }
    #[test]
    fn test_part2() {
        let d4 = Day4;
        let file = "./inputs/day4.test";
        let result = d4.part_b(file).unwrap();
        let expected = String::from("43");
        assert_eq!(result, expected)
    }
}
