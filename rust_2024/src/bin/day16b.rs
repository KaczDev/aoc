use anyhow::Result;
use aoc_utils::grid::{find_element, get_element, Direction, Grid, Point};
use std::{
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fs,
    time::Instant,
    usize,
};

#[derive(Eq, PartialEq, Debug, Hash)]
struct State {
    p: Point,
    d: Direction,
    s: usize,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.s.cmp(&self.s).then_with(|| self.p.cmp(&other.p))
    }
}

fn solve(file_name: &str) -> Result<usize> {
    let input: Grid<char> = read_input(file_name)?;
    let start = find_element(&input, 'S');
    let res = walk(&input, start, Direction::Right);
    Ok(res)
}

fn walk(grid: &Grid<char>, cur: Point, dir: Direction) -> usize {
    let mut pq: BinaryHeap<State> = BinaryHeap::new();
    let mut visited: HashSet<(Point, Direction)> = HashSet::new();
    //add the path vec to state
    // store prev point in state and store the path in hashmap(point,vec) and we just update the
    // point to current after we get the vec from prev
    let mut best_paths: HashMap<Point, VecDeque<Point>> = HashMap::new();
    pq.push(State {
        p: cur,
        d: dir,
        s: 0,
    });
    while let Some(state) = pq.pop() {
        let cur_el = *get_element(grid, state.p);
        if visited.contains(&(state.p, state.d)) {
            continue;
        }
        visited.insert((state.p, state.d));
        if cur_el == '#' {
            continue;
        }
        if cur_el == 'E' {
            return state.s;
        }
        pq.push(State {
            p: state.d.move_point(state.p),
            d: state.d,
            s: state.s + 1,
        });
        let dright = state.d.rotate_clockwise();
        let goright = dright.move_point(state.p);
        if *get_element(grid, goright) != '#' {
            pq.push(State {
                p: goright,
                d: dright,
                s: state.s + 1001,
            });
        }
        let dleft = state.d.rotate_counter_clockwise();
        let goleft = dleft.move_point(state.p);
        if *get_element(grid, goleft) != '#' {
            pq.push(State {
                p: goleft,
                d: dleft,
                s: state.s + 1001,
            });
        }
    }
    panic!("Couldn't find the END");
}

fn read_input(file_name: &str) -> Result<Grid<char>> {
    let r = fs::read_to_string(file_name)?
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    Ok(r)
}

fn main() -> Result<()> {
    let files = [
        "./inputs/day16.test",
        "./inputs/day16.test2",
        "./inputs/day16.prod",
    ];
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
        let file = "./inputs/day16.test";
        let result = solve(file).unwrap();
        let expected = 7036;
        assert_eq!(result, expected)
    }
    #[test]
    fn example_2() {
        let file = "./inputs/day16.test2";
        let result = solve(file).unwrap();
        let expected = 11048;
        assert_eq!(result, expected)
    }
}
