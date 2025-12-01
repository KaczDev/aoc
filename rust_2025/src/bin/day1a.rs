use anyhow::{anyhow, Result};
use std::{fs, str::FromStr, time::Instant};

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
fn rotate(mut cur: usize, rot: &mut Rotation) -> usize {
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

fn solve(file_name: &str) -> Result<usize> {
    let mut cur: usize = 50;
    let mut res: usize = 0;
    let mut input: Input<Rotation> = read_input(file_name)?;
    for rot in &mut input {
        cur = rotate(cur, rot);
        if cur == 0 {
            res += 1;
        }
    }
    Ok(res)
}

fn read_input(file_name: &str) -> Result<Input<Rotation>> {
    Ok(fs::read_to_string(file_name)?
        .lines()
        .map(|line| Rotation::from_str(line).expect(&format!("Couldn't parse line {}", line)))
        .collect())
}

fn main() -> Result<()> {
    let files = ["./inputs/day1.test", "./inputs/day1.prod"];
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
    fn test() {
        let file = "./inputs/day1.test";
        let result = solve(file).unwrap();
        let expected = 3;
        assert_eq!(result, expected)
    }
}
