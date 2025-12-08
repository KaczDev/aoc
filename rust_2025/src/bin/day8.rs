use anyhow::Result;
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    f64,
    fmt::Display,
    fs,
    str::FromStr,
    time::Instant,
};

type Input<T> = Vec<T>;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: isize,
    y: isize,
    z: isize,
}

impl Position {
    /// euclidic distance
    fn distance(&self, q: &Position) -> f64 {
        let dx = (self.x - q.x).pow(2) as f64;
        let dy = (self.y - q.y).pow(2) as f64;
        let dz = (self.z - q.z).pow(2) as f64;
        (dx + dy + dz).sqrt()
    }
}
impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {}, y: {}, z: {}", self.x, self.y, self.z)
    }
}

impl FromStr for Position {
    type Err = anyhow::Error;

    // expects s to be "x,y,z"
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let s = s.split(",").collect_vec();
        let x = s[0]
            .parse::<isize>()
            .expect(&format!("Couldn't parse {}", s[0]));
        let y = s[1]
            .parse::<isize>()
            .expect(&format!("Couldn't parse {}", s[1]));
        let z = s[2]
            .parse::<isize>()
            .expect(&format!("Couldn't parse {}", s[2]));
        Ok(Self { x, y, z })
    }
}

fn find_closest_pair(
    input: &Vec<Position>,
    connections: &HashSet<(Position, Position)>,
) -> (usize, usize) {
    let mut min_dist = f64::MAX;
    let mut p_idx = usize::MAX;
    let mut q_idx = usize::MAX;
    for i in 0..input.len() {
        let p = input.get(i).expect("Should never happen");
        for j in 0..input.len() {
            if i==j{
                continue;
            }
            let q = input.get(j).expect("Should never happen");
            let connection = if p.x < q.x { (*p, *q) } else { (*q, *p) };
            if connections.contains(&connection) {
                continue;
            }
            let d = p.distance(q);
            if d < min_dist {
                min_dist = d;
                p_idx = i;
                q_idx = j;
            }
        }
    }
    (p_idx, q_idx)
}

fn merge_circuits(
    lookup: &mut HashMap<Position, usize>,
    circuits: &mut Vec<HashSet<Position>>,
    p: &Position,
    q: &Position,
) {
    let circuit_id_q = lookup.get(&q).unwrap().clone();
    let circuit_q = circuits[circuit_id_q].clone();
    let circuit_id_p = lookup.get(&p).unwrap().clone();
    //update IDs for all positions in the circuit
    for x in &circuit_q {
        *lookup.get_mut(x).unwrap() = circuit_id_p;
    }
    //merge circuits
    circuits.get_mut(circuit_id_p).unwrap().extend(circuit_q);
    //remove merged circuit
    circuits.remove(circuit_id_q);
    //update other indexes
    lookup
        .iter_mut()
        .filter(|(_, v)| **v > circuit_id_q)
        .for_each(|(_, v)| {
            *v -= 1;
        });
}

fn connect_pairs(
    (p_idx, q_idx): (usize, usize),
    input: &Vec<Position>,
    lookup: &mut HashMap<Position, usize>,
    circuits: &mut Vec<HashSet<Position>>,
    connections: &mut HashSet<(Position, Position)>,
) {
    let p = input.get(p_idx).expect("xdd").clone();
    let q = input.get(q_idx).expect("xdd").clone();
    let connection = if p.x < q.x { (p, q) } else { (q, p) };
    connections.insert(connection);
    //merge circuits if they dont belong to the same one
    if lookup.get(&q).unwrap() != lookup.get(&p).unwrap() {
        merge_circuits(lookup, circuits, &p, &q);
    }
}

fn solve(file_name: &str, max_connections: usize) -> Result<usize> {
    let input: Input<Position> = read_input(file_name)?;
    // IT's AN UNDIRECTED GRAPH / DSU
    // Union Find

    //map to connect a Position to a certain circuit
    let mut lookup: HashMap<Position, usize> = HashMap::new();
    let mut connections: HashSet<(Position, Position)> = HashSet::new();
    let mut circuits: Vec<HashSet<Position>> = vec![];
    for x in &input {
        let mut h = HashSet::new();
        h.insert(*x);
        circuits.push(h);
        lookup.insert(*x, circuits.len() - 1);
    }
    let mut conns = 0;
    while conns < max_connections {
        //1. Find the closest pair
        let (p_idx, q_idx) = find_closest_pair(&input, &connections);
        //2. Connect the closest pair
        //  2a. If they are in the same circuit, it's still a connection
        if p_idx != usize::MAX && q_idx != usize::MAX {
            connect_pairs(
                (p_idx, q_idx),
                &input,
                &mut lookup,
                &mut circuits,
                &mut connections,
            );
            conns += 1;
        }
    }
    circuits.sort_by(|a, b| b.len().cmp(&a.len()));
    let res = circuits
        .into_iter()
        .take(3)
        .map(|c| c.len())
        .reduce(|acc, e| {
            if acc == 0 {
                return 1 * e;
            }
            acc * e
        })
        .unwrap();
    Ok(res)
}

fn solve_2(file_name: &str) -> Result<isize> {
    let input: Input<Position> = read_input(file_name)?;

    //map to connect a Position to a certain circuit
    let mut lookup: HashMap<Position, usize> = HashMap::new();
    let mut connections: HashSet<(Position, Position)> = HashSet::new();
    let mut circuits: Vec<HashSet<Position>> = vec![];
    let mut last_pair: (Position, Position) = (input[0], input[1]);
    for x in &input {
        let mut h = HashSet::new();
        h.insert(*x);
        circuits.push(h);
        lookup.insert(*x, circuits.len() - 1);
    }
    while circuits.len() != 1 {
        //1. Find the closest pair
        let (p_idx, q_idx) = find_closest_pair(&input, &connections);
        //2. Connect the closest pair
        //  2a. If they are in the same circuit, it's still a connection
        if p_idx != usize::MAX && q_idx != usize::MAX {
            last_pair = (input[p_idx], input[q_idx]);
            connect_pairs(
                (p_idx, q_idx),
                &input,
                &mut lookup,
                &mut circuits,
                &mut connections,
            );
        }
    }
    let res = last_pair.0.x * last_pair.1.x;
    Ok(res)
}

fn read_input(file_name: &str) -> Result<Input<Position>> {
    Ok(fs::read_to_string(file_name)?
        .lines()
        .map(|line| {
            line.trim()
                .parse::<Position>()
                .expect(&format!("Couldn't parse '{}'", line))
        })
        .collect())
}

fn main() -> Result<()> {
    let files = [("./inputs/day8.test", 10), ("./inputs/day8.prod", 1000)];
    println!("# Part 1");
    for (file, connections) in files {
        let now = Instant::now();
        let res = solve(file, connections)?;
        println!("{}: {} in {}ms", file, res, now.elapsed().as_millis());
    }
    println!("# Part 2");
    for (file, _) in files {
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
    fn part_1() {
        let file = "./inputs/day8.test";
        let result = solve(file, 10).unwrap();
        let expected = 40;
        assert_eq!(result, expected)
    }
    #[test]
    fn part_2() {
        let file = "./inputs/day8.test";
        let result = solve_2(file).unwrap();
        let expected = 25272;
        assert_eq!(result, expected)
    }
}
