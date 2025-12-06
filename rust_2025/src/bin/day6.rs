use anyhow::Result;
use std::{fs, time::Instant};

type Input<T> = Vec<Vec<T>>;

fn solve_1(file_name: &str) -> Result<usize> {
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

    Ok(res)
}

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

fn solve_2(file_name: &str) -> Result<usize> {
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

    Ok(res)
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

fn main() -> Result<()> {
    let files = ["./inputs/day6.test", "./inputs/day6.prod"];
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
    fn test_part_1() {
        let file = "./inputs/day6.test";
        let result = solve_1(file).unwrap();
        let expected = 4277556;
        assert_eq!(result, expected)
    }
    #[test]
    fn test_part_2() {
        let file = "./inputs/day6.test";
        let result = solve_2(file).unwrap();
        let expected = 3263827;
        assert_eq!(result, expected)
    }
}
