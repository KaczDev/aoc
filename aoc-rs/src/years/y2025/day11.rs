use crate::Solution;
use anyhow::Result;
use rayon::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;

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

fn read_input(file_name: &str) -> Result<Input> {
    let mut input = HashMap::new();
    crate::reader::read_lines(file_name)?
        .map_while(Result::ok)
        .for_each(|line| {
            let (device_name, outs) = parse_input_line(&line);
            input.insert(device_name, outs);
        });
    Ok(input)
}

pub struct Day11;

impl Solution for Day11 {
    fn part_a(&self, file_name: &str) -> Result<String> {
        let input: Input = read_input(file_name)?;
        let res = walk(&String::from("you"), &input);

        Ok(format!("{}", res))
    }

    fn part_b(&self, file_name: &str) -> Result<String> {
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
        Ok(format!("{}", all_paths))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_1() {
        let d11 = Day11;
        let file = "./inpures/day11.test";
        let result = d11.part_a(file).unwrap();
        let expected = String::from("5");
        assert_eq!(result, expected)
    }

    #[test]
    fn part_2() {
        let d11 = Day11;
        let file = "./inputs/day11.test2";
        let result = d11.part_b(file).unwrap();
        let expected = String::from("2");
        assert_eq!(result, expected)
    }
}
