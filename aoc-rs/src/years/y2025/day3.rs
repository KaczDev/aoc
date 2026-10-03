use crate::Solution;
use anyhow::Result;
use std::fs;

type Input<T> = Vec<Vec<T>>;

fn max_joltage(bank: &Vec<usize>, num_of_digits: usize) -> usize {
    if num_of_digits == 0 {
        return 0;
    }
    let d = bank
        .iter()
        .take(bank.len() - num_of_digits + 1)
        .max()
        .expect("Couldn't find max in {left}");
    let idx = bank.iter().position(|n| n == d).unwrap();
    let (_left, right) = bank.split_at(idx + 1);

    d * (10_usize.pow(num_of_digits as u32 - 1)) + max_joltage(&right.to_vec(), num_of_digits - 1)
}

fn read_input(file_name: &str) -> Result<Input<usize>> {
    Ok(fs::read_to_string(file_name)?
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).expect("Unexpected character {c}!") as usize)
                .collect()
        })
        .collect())
}

pub struct Day3;

impl Solution for Day3 {
    fn part_a(&self, file_name: &str) -> Result<String> {
        let digits = 2;
        let input: Input<usize> = read_input(file_name)?;
        let res: usize = input.iter().map(|bank| max_joltage(bank, digits)).sum();
        Ok(format!("{}", res))
    }

    fn part_b(&self, file_name: &str) -> Result<String> {
        let digits = 12;
        let input: Input<usize> = read_input(file_name)?;
        let res: usize = input.iter().map(|bank| max_joltage(bank, digits)).sum();
        Ok(format!("{}", res))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let d3 = Day3;
        let file = "./inputs/day3.test";
        let result = d3.part_a(file).unwrap();
        let expected = String::from("357");
        assert_eq!(result, expected)
    }
    #[test]
    fn test_part2() {
        let d3 = Day3;
        let file = "./inputs/day3.test";
        let result = d3.part_b(file).unwrap();
        let expected = String::from("3121910778619");
        assert_eq!(result, expected)
    }
}
