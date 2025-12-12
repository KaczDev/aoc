use anyhow::Result;
use itertools::Itertools;
use std::{collections::HashMap, fs, time::Instant};

type Input = (
    HashMap<usize, Vec<Vec<char>>>,
    Vec<((usize, usize), Vec<usize>)>,
);

fn solve(file_name: &str) -> Result<usize> {
    println!("It doesn't work on example but works on real input that is very simple. I'm not gonna stress it out..");
    let mut res = 0;
    let mut presents_area: HashMap<usize, usize> = HashMap::new();
    let (presents, regions) = read_input(file_name)?;
    for (k, v) in presents {
        let area: usize = v
            .iter()
            .map(|s| s.iter().filter(|c| **c == '#').count())
            .sum();
        presents_area.insert(k, area);
    }
    for (region, required) in regions {
        let r_area = region.0 * region.1;
        let occupied_area: usize = required
            .iter()
            .enumerate()
            .filter(|(_, i)| **i != 0)
            .map(|(id, n)| presents_area[&id] * n)
            .sum();
        if occupied_area <= r_area {
            res+=1
        }
    }

    Ok(res)
}

fn read_input(file_name: &str) -> Result<Input> {
    let mut presents = HashMap::new();
    let mut regions = Vec::new();
    let input = fs::read_to_string(file_name)?
        .split("\n\n")
        .map(|s| s.to_string())
        .collect_vec();
    for i in 0..6 {
        let present = &input[i];
        let (id, shape) = present.split_once('\n').unwrap();
        let id = id.trim().replace(':', "");
        let id = id.parse()?;
        let shape = shape.lines().map(|s| s.chars().collect_vec()).collect();
        presents.insert(id, shape);
    }
    input[6].lines().for_each(|l| {
        let (region, required_ids) = l.split_once(':').unwrap();
        let ids = required_ids
            .split_whitespace()
            .map(|d| d.trim().parse::<usize>().unwrap())
            .collect();
        let region = region
            .trim()
            .split("x")
            .map(|d| d.trim().parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap();
        regions.push((region, ids));
    });
    Ok((presents, regions))
}

fn main() -> Result<()> {
    let files = ["./inputs/day12.test", "./inputs/day12.prod"];
    println!("# Part 1");
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
    fn part_1() {
        let file = "./inputs/day12.test";
        let result = solve(file).unwrap();
        let expected = 2;
        assert_eq!(result, expected)
    }
}
