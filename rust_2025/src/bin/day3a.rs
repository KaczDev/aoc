use anyhow::Result;
use std::{fs, time::Instant};

type Input<T> = Vec<Vec<T>>;

fn find_largest_digits_ordered(digits: &Vec<u32>, num_of_digits: usize) -> Vec<u32> {
    let res = vec![];
    let last_pos = 0;
    for _ in 0..num_of_digits {

    }

    res
}

fn solve(file_name: &str) -> Result<u32> {
    let input: Input<u32> = read_input(file_name)?;
    // println!("{:?}", input);

    let res = input
        .iter()
        .map(|bank| {
            let mut sum = String::new();
            for ele in find_largest_digits_ordered(bank, 2) {
                sum.push_str(&ele.to_string());
            }
            sum.parse::<u32>().expect("Couldn't aprse result")
        })
        .sum();
    Ok(res)
}

fn read_input(file_name: &str) -> Result<Input<u32>> {
    Ok(fs::read_to_string(file_name)?
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).expect("Unexpected character {c}!"))
                .collect()
        })
        .collect())
}

fn main() -> Result<()> {
    let files = ["./inputs/day3.test", "./inputs/day3.prod"];
    for file in files {
        let now = Instant::now();
        let res = solve(file)?;
        println!("{}: {} in {}ms", file, res, now.elapsed().as_millis());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        let file = "./inputs/day3.test";
        let result = solve(file).unwrap();
        let expected = 357;
        assert_eq!(result, expected)
    }
}
