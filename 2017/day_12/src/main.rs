use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut input = String::new();
    let mut file = File::open("input.txt").unwrap();
    file.read_to_string(&mut input).unwrap();

    let nodes: HashMap<i32, Node> = input.lines().map(Node::new).map(|n| (n.id, n)).collect();
    println!("Part one: {:?}", floodfill(0, &nodes).len());
    println!("Part two: {:?}", count_groups(&nodes));
}

fn count_groups(nodes: &HashMap<i32, Node>) -> usize {
    let mut remaining: Vec<i32> = nodes.iter().map(|n| *n.0).collect();
    let mut groups = 0;
    while !remaining.is_empty() {
        let visited = floodfill(remaining[0], &nodes);
        remaining.retain(|v| !visited.contains(v));
        groups += 1;
    }
    groups
}

fn floodfill(start: i32, nodes: &HashMap<i32, Node>) -> Vec<i32> {
    let mut frontier: Vec<&Node> = vec![nodes.get(&start).unwrap()];
    let mut visited: Vec<i32> = Vec::new();
    while !frontier.is_empty() {
        let current: &Node = frontier.pop().unwrap();
        for n in current.find_neighbours(&nodes) {
            if !visited.contains(&n.id) {
                visited.push(n.id);
                frontier.push(n);
            }
        }
    }
    visited
}

struct Node {
    id: i32,
    edges: Vec<i32>,
}

impl Node {
    fn new(line: &str) -> Self {
        let mut tokens = line.split(" <-> ");
        let id: i32 = tokens.next().unwrap().parse().unwrap();
        let edges: Vec<i32> = tokens
            .next()
            .unwrap()
            .split(", ")
            .filter_map(|i| i.parse().ok())
            .collect();
        Node {
            id: id,
            edges: edges,
        }
    }
    fn find_neighbours<'a>(&self, map: &'a HashMap<i32, Node>) -> Vec<&'a Node> {
        self.edges
            .iter()
            .map(|e| map.get(&e).expect(&format!("Missing node {}", e)))
            .collect()
    }
}
