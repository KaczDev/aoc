use anyhow::Result;
use itertools::Itertools;
use std::{cmp, fs, ops::RangeInclusive, time::Instant};

type Input<T> = Vec<T>;

fn merge_intervals(intervals: &mut Vec<RangeInclusive<usize>>) -> Vec<RangeInclusive<usize>> {
    intervals.sort_by(|i1, i2| i1.start().cmp(i2.start()));
    let mut merged_intervals: Vec<RangeInclusive<usize>> = vec![];
    merged_intervals.push(intervals[0].clone());
    for i in 1..intervals.len() {
        let i1_end = *merged_intervals[merged_intervals.len() - 1].end();
        let i2_start = *intervals[i].start();
        let i2_end = *intervals[i].end();
        if i2_start <= i1_end {
            let start = *merged_intervals[merged_intervals.len() - 1].start();
            let new_end = cmp::max(i1_end, i2_end);
            merged_intervals.remove(merged_intervals.len() - 1);
            merged_intervals.push(RangeInclusive::new(start, new_end));
        } else {
            merged_intervals.push(intervals[i].clone());
        }
    }
    merged_intervals
}

fn solve_1(file_name: &str) -> Result<usize> {
    let input: Input<String> = read_input(file_name)?;
    let mut intervals = parse_intervals(&input[0]);
    let intervals = merge_intervals(&mut intervals);
    let res = input[1]
        .lines()
        .map(|id| id.parse::<usize>().expect("Couldn't parse {id}"))
        .filter(|id| {
            for itrv in &intervals {
                if itrv.contains(id) {
                    return true;
                }
            }
            false
        })
        .count();

    Ok(res)
}
fn solve_2(file_name: &str) -> Result<usize> {
    let input: Input<String> = read_input(file_name)?;
    let mut intervals = parse_intervals(&input[0]);
    let merged = merge_intervals(&mut intervals);
    let res = merged
        .into_iter()
        .map(|itrv| {
            let start = itrv.start();
            let end = itrv.end();
            end - start + 1
        })
        .sum();

    Ok(res)
}

fn parse_intervals(input: &String) -> Vec<RangeInclusive<usize>> {
    let res = input
        .lines()
        .map(|interval| {
            let range = interval.split('-').collect_vec();
            let start = range[0]
                .parse::<usize>()
                .expect("Couldn't parse {range[0]}");
            let end = range[1]
                .parse::<usize>()
                .expect("Couldn't parse {range[1]}");
            RangeInclusive::new(start, end)
        })
        .collect_vec();

    res
}

fn read_input(file_name: &str) -> Result<Input<String>> {
    Ok(fs::read_to_string(file_name)?
        .split("\n\n")
        .map(|s| s.to_string())
        .collect_vec())
}

fn main() -> Result<()> {
    let files = ["./inputs/day5.test", "./inputs/day5.prod"];
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
        let file = "./inputs/day5.test";
        let result = solve_1(file).unwrap();
        let expected = 3;
        assert_eq!(result, expected)
    }
    #[test]
    fn test_part2() {
        let file = "./inputs/day5.test";
        let result = solve_2(file).unwrap();
        let expected = 14;
        assert_eq!(result, expected)
    }
}
