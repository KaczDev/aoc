use crate::Solution;
use crate::grid::{Grid, Point};
use anyhow::Result;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

type Input<T> = Vec<Vec<T>>;

fn merge_timelines(
    new_beams: &mut HashMap<Point, usize>,
    new_beam_point: Point,
    old_beam_point: Point,
    prev_timelines: usize,
) {
    if new_beams.contains_key(&new_beam_point) {
        *new_beams.get_mut(&new_beam_point).expect("err") += prev_timelines;
    } else {
        new_beams.remove(&old_beam_point);
        new_beams.insert(new_beam_point, prev_timelines);
    }
}

fn read_input(file_name: &str) -> Result<Input<char>> {
    Ok(fs::read_to_string(file_name)?
        .lines()
        .map(|line| line.chars().collect())
        .collect())
}

pub struct Day7;

impl Solution for Day7 {
    fn part_a(&self, file_name: &str) -> Result<String> {
        let mut splits = 0;
        let grid: Grid<char> = read_input(file_name)?;
        let (sr, sc): Point = crate::grid::find_element(&grid, 'S');
        let mut beams: HashSet<Point> = HashSet::new();
        beams.insert((sr + 1, sc));
        for r in sr + 2..grid.len() {
            let mut new_beams: HashSet<Point> = HashSet::new();
            for (_, bc) in beams.into_iter() {
                if grid[r][bc] == '^' {
                    splits += 1;
                    if bc != 0 {
                        new_beams.insert((r, bc - 1));
                    }
                    if bc + 1 < grid[0].len() {
                        new_beams.insert((r, bc + 1));
                    }
                } else {
                    new_beams.insert((r, bc));
                }
            }
            beams = new_beams;
        }

        Ok(format!("{}", splits))
    }

    fn part_b(&self, file_name: &str) -> Result<String> {
        let mut timelines = 0;
        let grid: Grid<char> = read_input(file_name)?;
        let (sr, sc): Point = crate::grid::find_element(&grid, 'S');
        let mut beams: HashMap<Point, usize> = HashMap::new();
        beams.insert((sr + 1, sc), 1);
        for r in sr + 2..grid.len() {
            let mut new_beams: HashMap<Point, usize> = HashMap::new();
            for ((br, bc), prev_timelines) in beams.into_iter() {
                if grid[r][bc] == '^' {
                    if bc != 0 {
                        merge_timelines(&mut new_beams, (r, bc - 1), (br, bc), prev_timelines);
                    }
                    if bc + 1 < grid[0].len() {
                        merge_timelines(&mut new_beams, (r, bc + 1), (br, bc), prev_timelines);
                    }
                } else {
                    merge_timelines(&mut new_beams, (r, bc), (br, bc), prev_timelines);
                }
            }
            beams = new_beams;
        }
        for (_, t) in beams {
            timelines += t;
        }

        Ok(format!("{}", timelines))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() {
        let d7 = Day7;
        let file = "./inputs/day7.test";
        let result = d7.part_a(file).unwrap();
        let expected = String::from("21");
        assert_eq!(result, expected)
    }
    #[test]
    fn test_part_2() {
        let d7 = Day7;
        let file = "./inputs/day7.test";
        let result = d7.part_b(file).unwrap();
        let expected = String::from("40");
        assert_eq!(result, expected)
    }
}
