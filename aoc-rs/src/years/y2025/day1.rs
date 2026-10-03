use crate::Solution;
use anyhow::{Result, anyhow};
use std::{fs, str::FromStr};

type Input<T> = Vec<T>;

#[derive(Debug, Copy, Clone)]
enum Direction {
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return match s {
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(anyhow!("Didn't expect this character to be here - {s}")),
        };
    }
}

#[derive(Debug, Copy, Clone)]
struct Rotation {
    direction: Direction,
    dist: usize,
}
impl FromStr for Rotation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (di, ds) = s.split_at(1);
        let direction = Direction::from_str(di)?;
        let dist = ds.parse::<usize>()?;
        Ok(Rotation { direction, dist })
    }
}
const MAX: usize = 100;

fn rotate_a(mut cur: usize, rot: &mut Rotation) -> usize {
    if rot.dist >= MAX {
        rot.dist = rot.dist % MAX;
    }
    match rot.direction {
        Direction::Left => match cur.checked_sub(rot.dist) {
            Some(new) => cur = new,
            None => cur = MAX - (rot.dist - cur),
        },
        Direction::Right => {
            cur += rot.dist;
            if cur >= MAX {
                cur -= MAX;
            }
        }
    }
    cur
}

fn rotate_b(mut cur: usize, rot: &mut Rotation) -> (usize, usize) {
    let mut ticks = 0;
    if rot.dist >= MAX {
        ticks = rot.dist / MAX;
        rot.dist = rot.dist % MAX;
    }
    match rot.direction {
        Direction::Left => match cur.checked_sub(rot.dist) {
            Some(new) => cur = new,
            None => {
                if cur != 0 {
                    ticks += 1;
                }
                cur = MAX - (rot.dist - cur);
            }
        },
        Direction::Right => {
            cur += rot.dist;
            if cur >= MAX {
                cur -= MAX;
                if cur != 0 {
                    ticks += 1;
                }
            }
        }
    }
    (cur, ticks)
}

fn read_input(file_name: &str) -> Result<Input<Rotation>> {
    Ok(fs::read_to_string(file_name)?
        .lines()
        .map(|line| Rotation::from_str(line).expect(&format!("Couldn't parse line {}", line)))
        .collect())
}

pub struct Day1;

impl Solution for Day1 {
    fn part_a(&self, file_name: &str) -> Result<String> {
        let mut cur: usize = 50;
        let mut res: usize = 0;
        let mut input: Input<Rotation> = read_input(file_name)?;
        for rot in &mut input {
            cur = rotate_a(cur, rot);
            if cur == 0 {
                res += 1;
            }
        }
        Ok(format!("{}", res))
    }

    fn part_b(&self, file_name: &str) -> Result<String> {
        let mut cur: usize = 50;
        let mut res: usize = 0;
        let mut input: Input<Rotation> = read_input(file_name)?;
        for rot in &mut input {
            let (newcur, ticks) = rotate_b(cur, rot);
            cur = newcur;
            res += ticks;
            if cur == 0 {
                res += 1;
            }
        }
        Ok(format!("{}", res))
    }
}

#[cfg(test)]
mod tests {
    pub use crate::solution::Solution;
    use crate::years::y2025::day1::Day1;

    #[test]
    fn part_1() {
        let d1 = Day1;
        let file = "./inputs/day1.test";
        let result = d1.part_a(file).unwrap();
        let expected = String::from("3");
        assert_eq!(result, expected)
    }
    #[test]
    fn part_2() {
        let d1 = Day1;
        let file = "./inputs/day1.test";
        let result = d1.part_b(file).unwrap();
        let expected = String::from("6");
        assert_eq!(result, expected)
    }
}
