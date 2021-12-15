use priority_queue::PriorityQueue;
use std::collections::HashMap;
use std::env;
use std::fs;

fn parse_map(input: String) -> Vec<Vec<u32>> {
    input
        .split("\n")
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn neighbours((x, y): (usize, usize), width: usize, height: usize) -> Vec<(usize, usize)> {
    let mut nb = Vec::new();

    if x > 0 {
        nb.push((x - 1, y));
    }
    if y > 0 {
        nb.push((x, y - 1));
    }
    if x < (width - 1) {
        nb.push((x + 1, y));
    }
    if y < (height - 1) {
        nb.push((x, y + 1));
    }

    nb
}

fn print_map(map: &Vec<Vec<u32>>, path: &Vec<(usize, usize)>) {
    let height = map.len();
    let width = map[0].len();

    for y in 0..height {
        for x in 0..width {
            if path.contains(&(x, y)) {
                print!(".")
            } else {
                print!("{}", map[y][x])
            }
        }
        println!("");
    }
}

fn find_lowest_risk_path(map: &Vec<Vec<u32>>) -> u32 {
    let height = map.len();
    let width = map[0].len();
    let max = 1000000;

    let end = (width - 1, height - 1);
    let start = (0, 0);

    let mut frontier: PriorityQueue<(usize, usize), u32> = PriorityQueue::new();
    frontier.push(start, max);
    let mut come_from: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    come_from.insert(start, (0, 0));
    let mut cost: HashMap<(usize, usize), u32> = HashMap::new();
    cost.insert(start, 0);

    while frontier.len() > 0 {
        let (current, _) = frontier.pop().unwrap();

        for nb in neighbours(current, width, height) {
            let new_cost = *cost.entry(current).or_insert(0) + map[nb.1][nb.0];
            if !cost.contains_key(&nb) || new_cost < *cost.get(&nb).unwrap() {
                cost.insert(nb, new_cost);
                frontier.push(nb, max - new_cost);
                come_from.insert(nb, current);
            }
        }
    }

    let mut total_cost = 0;
    let mut pos = end;
    let mut path: Vec<(usize, usize)> = Vec::new();
    while pos != start {
        path.push(pos);
        total_cost += map[pos.1][pos.0];
        pos = *come_from.get(&pos).unwrap();
    }

    // print_map(&map, &path);

    total_cost
}

fn inc_risk(risk: u32, val: u32) -> u32 {
    if risk + val > 9 {
        risk + val - 9
    } else {
        risk + val
    }
}

fn build_full_map(map: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut full_map = Vec::new();

    for expand_y in 0..5 {
        for y in 0..map.len() {
            let mut row: Vec<u32> = Vec::new();
            for expand_x in 0..5 {
                map[y]
                    .iter()
                    .map(|risk| inc_risk(*risk, expand_x + expand_y))
                    .for_each(|risk| row.push(risk));
            }
            full_map.push(row);
        }
    }

    full_map
}

fn main() {
    let filename = env::args().nth(1).expect("Please provide input file");
    let map = parse_map(fs::read_to_string(filename).unwrap());

    println!("Part 1: {}", find_lowest_risk_path(&map));

    let full_map = build_full_map(&map);
    println!("Part 2: {}", find_lowest_risk_path(&full_map));
}
