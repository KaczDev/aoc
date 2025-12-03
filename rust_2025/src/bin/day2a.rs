use anyhow::Result;
use std::{fs, str::FromStr, time::Instant};

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

fn solve(file_name: &str) -> Result<usize> {
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
    Ok(res)
}

fn read_input(file_name: &str) -> Result<Input<Interval>> {
    Ok(fs::read_to_string(file_name)?
        .split(',')
        .map(|interval| Interval::from_str(interval.trim()).expect("Couldn't parse {interval}"))
        .collect())
}

fn main() -> Result<()> {
    let files = ["./inputs/day2.test", "./inputs/day2.prod"];
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
        let file = "./inputs/day2.test";
        let result = solve(file).unwrap();
        let expected = 1227775554;
        assert_eq!(result, expected)
    }
}
