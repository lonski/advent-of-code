extern crate regex;

use std::fs::File;
use std::io::prelude::*;
use regex::Regex;
use std::collections::HashSet;
use std::collections::HashMap;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    println!("Part one: {}", part_one(&input));
    println!("Part two: {:?}", part_two(&input));
}

fn part_two(input: &String) -> Vec<u32> {
    let mut result: Vec<u32> = Vec::new();
    let tree: HashMap<String, Node> = input
        .lines()
        .map(Node::new)
        .map(|n| (n.name.clone(), n))
        .collect();
    for ref node in &tree {
        if node.1.children.is_empty() {
            continue;
        }
        let mut children_weights: Vec<(&Node, u32)> = node.1
            .children
            .iter()
            .map(|c| tree.get(c).unwrap())
            .map(|c| (c, count_weight(c, &tree)))
            .collect();
        children_weights.dedup_by_key(|t| t.1);
        if children_weights.len() > 1 {
            let heavier = children_weights.iter().max_by_key(|k| k.1).unwrap();
            let diff = heavier.1 - children_weights.iter().min_by_key(|k| k.1).unwrap().1;
            result.push(heavier.0.weight - diff);
        }
    }
    result
}

fn count_weight(node: &Node, tree: &HashMap<String, Node>) -> u32 {
    if node.children.is_empty() {
        node.weight
    } else {
        node.weight +
            node.children
                .iter()
                .map(|c| count_weight(tree.get(c).unwrap(), tree))
                .sum::<u32>()
    }
}

fn part_one(input: &String) -> String {
    let branches: Vec<Node> = input
        .lines()
        .filter(|l| l.contains("->"))
        .map(|l| Node::new(l))
        .collect();
    let mut names: HashSet<String> = branches.iter().map(|n| n.name.clone()).collect();
    for branch in branches.iter() {
        for a_branch in branches.iter() {
            if a_branch.children.contains(&branch.name) {
                names.remove(&branch.name);
                break;
            }
        }
    }
    names.iter().next().unwrap().clone()
}

struct Node {
    name: String,
    weight: u32,
    children: Vec<String>,
}

impl Node {
    fn new(s: &str) -> Node {
        let children: Vec<String> = if s.contains("->") {
            String::from(&s[s.find(">").unwrap() + 1..])
                .split(",")
                .map(|c| String::from(c.trim()))
                .collect()
        } else {
            Vec::new()
        };
        let capture = Regex::new(r"^(\w+) \((\d+)\)")
            .unwrap()
            .captures(s)
            .unwrap();
        Node {
            name: String::from(&capture[1]),
            weight: capture[2].parse::<u32>().unwrap(),
            children: children,
        }
    }
}
