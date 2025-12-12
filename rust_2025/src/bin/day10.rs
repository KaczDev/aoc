use anyhow::Result;
use aoc_utils::reader::read_lines;
use itertools::Itertools;
use std::{
    collections::{HashSet, VecDeque},
    str::FromStr,
    time::Instant,
};

type Input<T> = Vec<T>;

#[derive(Debug)]
struct Machine {
    indicators: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<usize>,
}

fn press_buttons(current_indicators: &IndicatorState, buttons: &Vec<usize>) -> IndicatorState {
    let mut new_state = current_indicators.clone();
    for b in buttons {
        new_state.state[*b] = !current_indicators.state[*b];
    }
    new_state.presses += 1;
    new_state
}

#[derive(Debug, Clone)]
struct IndicatorState {
    state: Vec<bool>,
    presses: usize,
}

fn bfs(machine: &Machine) -> usize {
    let mut queue: VecDeque<IndicatorState> = VecDeque::new();
    let mut cache: HashSet<Vec<bool>> = HashSet::new();
    let start = IndicatorState {
        state: vec![false; machine.indicators.len()],
        presses: 0,
    };
    queue.push_back(start);
    let presses;
    'outer: loop {
        let state = queue.pop_back().unwrap();
        cache.insert(state.state.clone());
        if state.state == machine.indicators {
            presses = state.presses;
            break 'outer;
        }
        for b in &machine.buttons {
            let new_state = press_buttons(&state, b);
            if new_state.state == machine.indicators {
                presses = new_state.presses;
                break 'outer;
            }
            if !cache.contains(&new_state.state) {
                cache.insert(new_state.state.clone());
                queue.push_front(new_state.clone());
            }
        }
    }
    presses
}

fn solve(file_name: &str) -> Result<usize> {
    let mut res = 0;
    let input: Input<Machine> = read_input(file_name)?;
    for machine in &input {
        res += bfs(machine)
    }

    Ok(res)
}

fn solve_2(file_name: &str) -> Result<usize> {
    let mut res = 0;
    let input: Input<Machine> = read_input(file_name)?;
    println!("{:?}", input);

    Ok(res)
}

fn main() -> Result<()> {
    let files = ["./inputs/day10.test", "./inputs/day10.prod"];
    println!("# Part 1");
    for file in files {
        let now = Instant::now();
        let res = solve(file)?;
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

impl FromStr for Machine {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let inp = s.split(' ').collect_vec();
        let indicators = remove_first_last_char(inp[0])
            .chars()
            .map(|c| {
                if c == '.' {
                    return false;
                }
                true
            })
            .collect_vec();
        let mut buttons: Vec<Vec<usize>> = Vec::new();
        for i in 1..inp.len() - 1 {
            buttons.push(
                remove_first_last_char(inp[i])
                    .split(',')
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect_vec(),
            );
        }
        let jolt = inp[inp.len() - 1];
        let joltage = remove_first_last_char(jolt)
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect_vec();
        Ok(Self {
            indicators: indicators,
            buttons: buttons,
            joltage: joltage,
        })
    }
}

fn remove_first_last_char(s: &str) -> &str {
    let mut chars = s.chars();
    chars.next();
    chars.next_back();
    chars.as_str()
}

fn read_input(file_name: &str) -> Result<Input<Machine>> {
    Ok(read_lines(file_name)?
        .map_while(Result::ok)
        .map(|line| {
            line.parse()
                .expect(&format!("Couldn't parse line - '{}'", line))
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_1() {
        let file = "./inputs/day10.test";
        let result = solve(file).unwrap();
        let expected = 7;
        assert_eq!(result, expected)
    }

    #[test]
    fn part_2() {
        let file = "./inputs/day10.test";
        let result = solve_2(file).unwrap();
        let expected = 33;
        assert_eq!(result, expected)
    }
}
