use std::cmp::min;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::{env, fs};

use itertools::Itertools;
use regex::{Captures, Regex};

const INF: i32 = 999_999;

struct Valve {
    name: String,
    flow_rate: i32,
    tunnels: Vec<String>,
}

impl Valve {
    fn new(c: &Captures) -> Self {
        Valve {
            name: String::from(&c[1]),
            flow_rate: c[2].parse::<i32>().unwrap(),
            tunnels: c[3].split(", ").map(String::from).collect::<Vec<String>>(),
        }
    }
}

impl PartialEq<Self> for Valve {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Valve {}

impl Hash for Valve {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state)
    }
}

impl Display for Valve {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write! {f, "{}({}) -> {:?}", self.name, self.flow_rate, self.tunnels}
    }
}

fn parse(input: String) -> Vec<Valve> {
    let regex: Regex = Regex::new(
        r"^Valve (\w{2}) has flow rate=(\d+); tunnels? leads? to valves? ((:?\w{2},?\s?)+)$",
    )
    .unwrap();
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| regex.captures(l).unwrap())
        .map(|c| Valve::new(&c))
        .collect()
}

fn calculate_distance_map(valves: &Vec<Valve>) -> HashMap<String, HashMap<String, i32>> {
    let mut valve_idx: HashMap<String, usize> = HashMap::new();
    valves.iter().enumerate().for_each(|(i, valve)| {
        valve_idx.insert(valve.name.clone(), i);
    });

    let mut valve_names: HashMap<usize, String> = HashMap::new();
    valves.iter().enumerate().for_each(|(i, valve)| {
        valve_names.insert(i, valve.name.clone());
    });

    let mut matrix = valves
        .iter()
        .enumerate()
        .map(|(i, valve)| {
            let mut row = vec![INF; valves.len()];
            for tunnel in valve.tunnels.iter() {
                row[*valve_idx.get(tunnel).unwrap()] = 1;
            }
            row[i] = 0;
            row
        })
        .collect::<Vec<Vec<i32>>>();

    calculate_distances(&mut matrix);
    to_distance_map(&matrix, &valve_names)
}

// Use Floyd-Warshall algorithm to find shortest paths between vertices
fn calculate_distances(matrix: &mut Vec<Vec<i32>>) {
    let vertices = matrix.len();
    for k in 0..vertices {
        for i in 0..vertices {
            for j in 0..vertices {
                matrix[i][j] = min(matrix[i][j], matrix[i][k] + matrix[k][j]);
            }
        }
    }
}

// Transform the distance matrix into a map ValveName -> (ValveName, Distance)
fn to_distance_map(
    matrix: &Vec<Vec<i32>>,
    names: &HashMap<usize, String>,
) -> HashMap<String, HashMap<String, i32>> {
    let vertices = matrix.len();
    let mut map: HashMap<String, HashMap<String, i32>> = HashMap::new();
    for i in 0..vertices {
        let mut row: HashMap<String, i32> = HashMap::new();
        for j in 0..vertices {
            let n = names.get(&j).unwrap().clone();
            row.insert(n, matrix[i][j]);
        }
        let n = names.get(&i).unwrap().clone();
        map.insert(n, row);
    }

    map
}

fn max_pressure(
    time: i32,
    max_time: i32,
    current_valve: &Valve,
    visited: HashSet<String>,
    valves: &HashSet<&Valve>,
    dist_map: &HashMap<String, HashMap<String, i32>>,
) -> i32 {
    if time >= max_time {
        return 0;
    }

    let this_pressure = current_valve.flow_rate * (max_time - time);
    let mut new_visited = visited.clone();
    new_visited.insert(current_valve.name.clone());

    let max_sub_pressure = valves
        .iter()
        // find all valves that are worth to visit
        .filter(|v| v.flow_rate > 0)
        .filter(|v| !new_visited.contains(&v.name[..]))
        .map(|v| {
            let dist = dist_map
                .get(&v.name[..])
                .unwrap()
                .get(&current_valve.name[..])
                .unwrap();

            (*v, dist)
        })
        // calculate the pressure if we got this path
        .map(|(v, distance)| {
            max_pressure(
                time + distance + 1,
                max_time,
                v,
                new_visited.clone(),
                &valves,
                &dist_map,
            )
        })
        // and find the best path
        .max()
        .unwrap_or(0);

    this_pressure + max_sub_pressure
}

fn main() {
    let filename = env::args().nth(1).unwrap_or_else(|| String::from("input"));
    let input = fs::read_to_string(filename).unwrap();
    let valves = parse(input);
    let valves_ref: HashSet<&Valve> = valves.iter().collect();
    let distance_map = calculate_distance_map(&valves);
    let start = valves.iter().find(|v| v.name == "AA").unwrap();

    println!(
        "Part 1: {}",
        max_pressure(
            0,
            30,
            start,
            HashSet::from(["AA".to_owned()]),
            &valves_ref,
            &distance_map
        )
    );

    let non_zero_flow_valves: HashSet<&Valve> = valves.iter().filter(|v| v.flow_rate > 0).collect();
    println!(
        "Part 2: {}",
        non_zero_flow_valves
            .iter()
            .filter(|v| v.flow_rate > 0)
            // Split the valves by two - part for me, part for the elephant and try all the combinations to find the best one
            .combinations(non_zero_flow_valves.len() / 2)
            .map(|half| {
                let half_set = half.into_iter().map(|v| *v).collect::<HashSet<&Valve>>();
                let other_half = valves_ref
                    .difference(&half_set)
                    .map(|v| *v)
                    .collect::<HashSet<&Valve>>();
                (half_set, other_half)
            })
            .map(|(a, b)| {
                let my_pressure = max_pressure(0, 26, start, HashSet::new(), &a, &distance_map);
                let elephant_pressure =
                    max_pressure(0, 26, start, HashSet::new(), &b, &distance_map);
                my_pressure + elephant_pressure
            })
            .max()
            .unwrap()
    );
}
