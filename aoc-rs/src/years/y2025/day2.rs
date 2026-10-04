use crate::Solution;
use anyhow::Result;
use itertools::Itertools;
use std::{fs, str::FromStr};

type Input<T> = Vec<T>;

#[derive(Debug)]
struct Interval {
    start: usize,
    end: usize,
}
impl FromStr for Interval {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once("-").expect("Couldn't split interval {s}");
        let start = start.parse::<usize>()?;
        let end = end.parse::<usize>()?;
        Ok(Interval { start, end })
    }
}

fn read_input(file_name: &str) -> Result<Input<Interval>> {
    Ok(fs::read_to_string(file_name)?
        .split(',')
        .map(|interval| Interval::from_str(interval.trim()).expect("Couldn't parse {interval}"))
        .collect())
}

pub struct Day2;

impl Solution for Day2 {
    fn part_a(&self, file_name: &str) -> Result<String> {
        let input: Input<Interval> = read_input(file_name)?;
        let mut res = 0;
        input.iter().for_each(|interval| {
            for n in interval.start..=interval.end {
                let ns = n.to_string();
                if ns.len() % 2 != 0 {
                    continue;
                }
                let mid = ns.len() / 2;
                let (s1, s2) = ns.split_at(mid);
                if s1 == s2 {
                    res += n;
                }
            }
        });
        Ok(format!("{}", res))
    }

    fn part_b(&self, file_name: &str) -> Result<String> {
        let input: Input<Interval> = read_input(file_name)?;
        let mut res = 0;
        input.iter().for_each(|interval| {
            for n in interval.start..=interval.end {
                let ns = n.to_string();
                let mut valid = true;
                for seq_size in 1..=ns.len() / 2 {
                    //we're looking for invalid sequences
                    if ns.len() % seq_size != 0 {
                        continue;
                    }
                    //take first window and check if other windows match
                    let chunks = ns
                        .chars()
                        .chunks(seq_size)
                        .into_iter()
                        .map(|chunk| chunk.collect::<String>())
                        .collect_vec();
                    let first_chunk = &chunks[0];
                    for i in 1..chunks.len() {
                        if chunks[i] != *first_chunk {
                            valid = true;
                            break;
                        }
                        valid = false
                    }
                    if !valid {
                        break;
                    }
                }
                if !valid {
                    res += n;
                }
            }
        });
        Ok(format!("{}", res))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_1() {
        let d2 = Day2;
        let file = "./inputs/day2.test";
        let result = d2.part_a(file).unwrap();
        let expected = String::from("1227775554");
        assert_eq!(result, expected)
    }
    #[test]
    fn part_2() {
        let d2 = Day2;
        let file = "./inputs/day2.test";
        let result = d2.part_b(file).unwrap();
        let expected = String::from("4174379265");
        assert_eq!(result, expected)
    }
}
