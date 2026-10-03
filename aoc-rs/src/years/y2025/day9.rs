use crate::Solution;
use anyhow::Result;
use itertools::Itertools;
use std::{cmp, collections::HashSet, fmt::Display, fs, str::FromStr};

type Input<T> = Vec<T>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Corner {
    col: isize,
    row: isize,
}

impl Corner {
    fn area(&self, r2: &Corner) -> isize {
        let r1 = self;
        let a = (r2.col - r1.col).abs() + 1;
        let b = (r2.row - r1.row).abs() + 1;
        a * b
    }
}
impl Display for Corner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Position({},{})", self.col, self.row)
    }
}

impl FromStr for Corner {
    type Err = anyhow::Error;

    // expects s to be "x,y"
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let s = s.split(",").collect_vec();
        let x = s[0]
            .parse::<isize>()
            .expect(&format!("Couldn't parse {}", s[0]));
        let y = s[1]
            .parse::<isize>()
            .expect(&format!("Couldn't parse {}", s[1]));
        Ok(Self { col: x, row: y })
    }
}

fn is_valid_rectangle(polygon: &HashSet<Corner>, r1: &Corner, r2: &Corner) -> bool {
    let min_row = cmp::min(r1.row, r2.row);
    let max_row = cmp::max(r1.row, r2.row);
    let min_col = cmp::min(r1.col, r2.col);
    let max_col = cmp::max(r1.col, r2.col);
    for p in polygon {
        if p.row > min_row && p.row < max_row && p.col > min_col && p.col < max_col {
            return false;
        }
    }
    true
}

fn read_input(file_name: &str) -> Result<Input<Corner>> {
    Ok(fs::read_to_string(file_name)?
        .lines()
        .map(|line| line.parse::<Corner>().expect("culdnt parse"))
        .collect())
}

pub struct Day9;

impl Solution for Day9 {
    fn part_a(&self, file_name: &str) -> Result<String> {
        let input: Input<Corner> = read_input(file_name)?;
        let mut max_area = isize::MIN;
        input.iter().for_each(|r1| {
            input.iter().for_each(|r2| {
                let area = r1.area(&r2);
                if area > max_area {
                    max_area = area;
                }
            });
        });
        Ok(format!("{}", max_area))
    }

    fn part_b(&self, file_name: &str) -> Result<String> {
        let input: Input<Corner> = read_input(file_name)?;
        // polygon's perimeter
        let mut polygon: HashSet<Corner> = HashSet::new();
        input.iter().circular_tuple_windows().for_each(|(r1, r2)| {
            let min_row = cmp::min(r1.row, r2.row);
            let max_row = cmp::max(r1.row, r2.row);
            let min_col = cmp::min(r1.col, r2.col);
            let max_col = cmp::max(r1.col, r2.col);
            if r1.row == r2.row {
                for col in min_col..=max_col {
                    polygon.insert(Corner {
                        row: r1.row,
                        col: col,
                    });
                }
            }
            if r1.col == r2.col {
                for row in min_row..=max_row {
                    polygon.insert(Corner {
                        row: row,
                        col: r1.col,
                    });
                }
            }
        });
        let mut rectangles: Vec<(Corner, Corner, isize)> = input
            .iter()
            .combinations(2)
            .map(|comb| {
                let r1 = comb[0];
                let r2 = comb[1];
                let a = r1.area(r2);
                (*r1, *r2, a)
            })
            .collect();
        //sort rectangles by area
        rectangles.sort_by(|r1, r2| r2.2.cmp(&r1.2));
        for (r1, r2, area) in rectangles {
            if is_valid_rectangle(&polygon, &r1, &r2) {
                return Ok(format!("{}", area));
            }
        }

        Ok(format!("{}", 0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_1() {
        let d9 = Day9;
        let file = "./inputs/day9.test";
        let result = d9.part_a(file).unwrap();
        let expected = String::from("50");
        assert_eq!(result, expected)
    }

    #[test]
    fn part_2() {
        let d9 = Day9;
        let file = "./inputs/day9.test";
        let result = d9.part_b(file).unwrap();
        let expected = String::from("24");
        assert_eq!(result, expected)
    }
}
