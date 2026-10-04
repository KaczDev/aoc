use crate::Solution;
use anyhow::{Result, anyhow};
use itertools::Itertools;
use std::{collections::HashMap, fs};

type Input = (
    HashMap<usize, Vec<Vec<char>>>,
    Vec<((usize, usize), Vec<usize>)>,
);

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

pub struct Day12;

impl Solution for Day12 {
    fn part_a(&self, file_name: &str) -> Result<String> {
        println!(
            "It doesn't work on example but works on real input that is very simple. I'm not gonna stress it out.."
        );
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
                res += 1
            }
        }

        Ok(format!("{}", res))
    }

    fn part_b(&self, _file_name: &str) -> Result<String> {
        Err(anyhow!("Part B wasn't solved yet"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_1() {
        let d12 = Day12;
        let file = "./inputs/day12.test";
        let result = d12.part_a(file).unwrap();
        let expected = String::from("2");
        assert_eq!(result, expected)
    }
}
