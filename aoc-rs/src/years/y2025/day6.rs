use crate::Solution;
use anyhow::Result;
use std::{fs};

type Input<T> = Vec<Vec<T>>;


fn add_number(sign: &str, cul: &mut usize, n: &String) {
    let n = n
        .trim()
        .parse::<usize>()
        .expect(format!("Couldn't parse '{}'", n).as_str());

    match sign {
        "+" => *cul += n,
        "*" => *cul *= n,
        _ => panic!("Unkown sign '{}'", sign),
    }
}

fn read_input_1(file_name: &str) -> Result<Input<String>> {
    Ok(fs::read_to_string(file_name)?
        .lines()
        .map(|line| {
            line.trim()
                .split(' ')
                .map(|s| s.to_string())
                .filter(|s| !s.is_empty())
                .collect()
        })
        .collect())
}


fn replace_separator_column(input: &mut Input<String>) {
    // find empty columns
    let mut cols = vec![];
    for c in 0..input[0].len() {
        let mut is_empty_column = true;
        for r in 0..input.len() {
            if !input[r][c].trim().is_empty() {
                is_empty_column = false;
                break;
            }
        }
        if is_empty_column {
            cols.push(c);
        }
    }
    //replace empty columns with X
    for c in cols {
        for r in 0..input.len() {
            input[r][c] = "X".to_string();
        }
    }
}
fn read_input_2(file_name: &str) -> Result<Input<String>> {
    Ok(fs::read_to_string(file_name)?
        .lines()
        .map(|line| line.chars().map(|c| c.to_string()).collect())
        .collect())
}

pub struct Day6;

impl Solution for Day6 {
    fn part_a(&self, file_name: &str) -> Result<String> {
        let mut res = 0;
        let input: Input<String> = read_input_1(file_name)?;
        let signs = input[input.len() - 1].clone();
        for c in 0..signs.len() {
            let mut ans = 0;
            let sign = &signs[c];
            if sign.as_str() == "*" {
                ans = 1;
            }
            for r in 0..input.len() - 1 {
                let n = &input[r][c];
                add_number(sign, &mut ans, n);
            }
            res += ans;
        }

        Ok(format!("{}", res))
    }

    fn part_b(&self, file_name: &str) -> Result<String> {
        let mut res = 0;
        let mut input: Input<String> = read_input_2(file_name)?;
        // 1. Replace the separating spaces with 'X' so we can have clear "blocks"
        // of the math problems
        replace_separator_column(&mut input);
        let signs = input[input.len() - 1].clone();
        // 2. Sign is always starting the number
        // 3. it's + or * so order of numbers doesnt matter, can be just reading from right
        //
        let mut c = 0;
        while c < signs.len() {
            let mut ans = 0;
            let sign = &signs[c];
            if sign.as_str() == "*" {
                ans = 1;
            }
            if sign.as_str() == "X" {
                continue;
            }
            while c < signs.len() && signs[c] != "X" {
                // 4. keep reading the numbers from top to bottom until X is met (end of the block)
                //      4a. if character is empty '' just skip to next line
                let mut new_nr = String::new();
                for r in 0..input.len() - 1 {
                    if input[r][c].is_empty() {
                        continue;
                    }
                    new_nr.push_str(&input[r][c]);
                }
                add_number(sign, &mut ans, &new_nr);
                c += 1;
            }
            c += 1;
            res += ans;
        }

        Ok(format!("{}", res))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() {
        let d6=Day6;
        let file = "./inputs/day6.test";
        let result = d6.part_a(file).unwrap();
        let expected = String::from("4277556");
        assert_eq!(result, expected)
    }
    #[test]
    fn test_part_2() {
        let d6=Day6;
        let file = "./inputs/day6.test";
        let result = d6.part_b(file).unwrap();
        let expected = String::from("3263827");
        assert_eq!(result, expected)
    }
}
