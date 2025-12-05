use anyhow::Result;
use std::{fs, time::Instant};

type Input<T> = Vec<Vec<T>>;

fn max_joltage(bank: &Vec<usize>, num_of_digits: usize) -> usize {
    if num_of_digits == 0 {
        return 0;
    }
    // let (left, _right) = bank.split_at(bank.len() - num_of_digits as usize + 1);
    // let d = left.iter().max().expect("Couldn't find max in {left}");
    let d = bank
        .iter()
        .take(bank.len() - num_of_digits + 1)
        .max()
        .expect("Couldn't find max in {left}");
    let idx = bank.iter().position(|n| n == d).unwrap();
    let (_left, right) = bank.split_at(idx + 1);

    d * (10_usize.pow(num_of_digits as u32 - 1)) + max_joltage(&right.to_vec(), num_of_digits - 1)
}

fn solve(file_name: &str, digits: usize) -> Result<usize> {
    let input: Input<usize> = read_input(file_name)?;
    let res = input.iter().map(|bank| max_joltage(bank, digits)).sum();
    Ok(res)
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

fn main() -> Result<()> {
    let files = ["./inputs/day3.test", "./inputs/day3.prod"];
    println!("Part 1");
    for file in files {
        let now = Instant::now();
        let res = solve(file, 2)?;
        println!("{}: {} in {}ms", file, res, now.elapsed().as_millis());
    }
    println!("Part 2");
    for file in files {
        let now = Instant::now();
        let res = solve(file, 12)?;
        println!("{}: {} in {}ms", file, res, now.elapsed().as_millis());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let file = "./inputs/day3.test";
        let result = solve(file,2).unwrap();
        let expected = 357;
        assert_eq!(result, expected)
    }
    #[test]
    fn test_part2() {
        let file = "./inputs/day3.test";
        let result = solve(file,12).unwrap();
        let expected = 3121910778619;
        assert_eq!(result, expected)
    }
}
