use anyhow::Result;
use rayon::prelude::*;
use std::collections::HashMap;
use std::{collections::HashSet, time::Instant};

type Input = HashMap<String, HashSet<String>>;

fn parse_input_line(s: &str) -> (String, HashSet<String>) {
    let (name, outs) = s
        .split_once(':')
        .expect(&format!("Couldn't parse string ('{}') to Device", s));
    let hs_outs: HashSet<String> = outs.trim().split(' ').map(|d| d.to_string()).collect();
    (name.to_string(), hs_outs)
}

fn walk(cur: &String, input: &Input) -> usize {
    if cur == "out" {
        return 1;
    }
    let mut sum = 0;
    for out in input.get(cur).expect("shouldn't happen") {
        sum += walk(out, input);
    }
    return sum;
}

fn solve(file_name: &str) -> Result<usize> {
    let input: Input = read_input(file_name)?;
    let res = walk(&String::from("you"), &input);

    Ok(res)
}

fn walk_2(
    cur: &String,
    input: &Input,
    mut has_dac: bool,
    mut has_fft: bool,
    cache: &mut HashMap<(String, bool, bool), usize>,
) -> usize {
    let key = (cur.clone(), has_dac, has_fft);
    if let Some(&cached) = cache.get(&key) {
        return cached;
    }

    if cur == "out" {
        return if has_dac && has_fft { 1 } else { 0 };
    }

    if cur == "fft" {
        has_fft = true;
    }
    if cur == "dac" {
        has_dac = true;
    }

    let mut sum = 0;
    for out in input.get(cur).expect("shouldn't happen") {
        sum += walk_2(out, input, has_dac, has_fft, cache);
    }
    cache.insert(key, sum);
    sum
}
fn solve_2(file_name: &str) -> Result<usize> {
    let input: Input = read_input(file_name)?;
    let starting_branches: Vec<&String> = input
        .get("svr")
        .expect("Input doesn't have 'svr' in it!")
        .iter()
        .collect();

    let all_paths: usize = starting_branches
        .par_iter()
        .map(|branch| {
            // cache: (node, has_dac, has_fft) -> path count
            let mut cache: HashMap<(String, bool, bool), usize> = HashMap::new();
            walk_2(branch, &input, false, false, &mut cache)
        })
        .sum();
    Ok(all_paths)
}

fn read_input(file_name: &str) -> Result<Input> {
    let mut input = HashMap::new();
    aoc_utils::reader::read_lines(file_name)?
        .map_while(Result::ok)
        .for_each(|line| {
            let (device_name, outs) = parse_input_line(&line);
            input.insert(device_name, outs);
        });
    Ok(input)
}

fn main() -> Result<()> {
    let files = ["./inputs/day11.test", "./inputs/day11.prod"];
    println!("# Part 1");
    for file in files {
        let now = Instant::now();
        let res = solve(file)?;
        println!("{}: {} in {}ms", file, res, now.elapsed().as_millis());
    }
    let files = ["./inputs/day11.test2", "./inputs/day11.prod"];
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
        let file = "./inputs/day11.test";
        let result = solve(file).unwrap();
        let expected = 5;
        assert_eq!(result, expected)
    }

    #[test]
    fn part_2() {
        let file = "./inputs/day11.test2";
        let result = solve_2(file).unwrap();
        let expected = 2;
        assert_eq!(result, expected)
    }
}
