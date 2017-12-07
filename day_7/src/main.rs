extern crate regex;

use std::fs::File;
use std::io::prelude::*;
use regex::Regex;
use std::collections::HashSet;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    println!("Part one: {}", part_one(&input));
}

fn part_one(input: &String) -> String {
    let branches: Vec<Node> = input
        .lines()
        .filter(|l| l.contains("->"))
        .map(|l| Node::new(l))
        .collect();

    let mut names: HashSet<String> = branches.iter().map(|n| n.name.clone()).collect();

    for (i, branch) in branches.iter().enumerate() {
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
    children: String,
}

impl Node {
    fn new(s: &str) -> Node {
        let regex = Regex::new(r"(\w+) \((\d+)\) -> (.*)").unwrap();
        let cap = regex.captures(s).unwrap();
        Node {
            name: String::from(&cap[1]),
            weight: cap[2].parse::<u32>().unwrap(),
            children: String::from(&cap[3]),
        }
    }
}
