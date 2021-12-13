use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;

#[derive(Debug)]
struct Cave {
    name: String,
    exits: HashSet<String>,
}

impl Cave {
    fn is_small(name: &str) -> bool {
        name.chars().all(|c| c.is_lowercase())
    }
}

fn count(vec: &Vec<&&str>, key: &str) -> usize {
    vec.iter().filter(|c| ***c == key).count()
}

fn count_small_visited_more_than_once(path: &Vec<&str>) -> usize {
    let visited_small = path
        .iter()
        .filter(|c| **c != "end" && Cave::is_small(&c))
        .collect::<Vec<_>>();

    visited_small
        .iter()
        .map(|c| count(&visited_small, c))
        .filter(|n| *n > 1)
        .count()
}

fn find_paths<'a>(
    current: &'a Cave,
    partial_paths: Vec<Vec<&'a str>>,
    caves: &'a HashMap<String, Cave>,
    max_small_visit: usize,
) -> Vec<Vec<&'a str>> {
    let mut final_paths = Vec::new();

    if partial_paths.len() == 0 {
        return Vec::new();
    }

    for exit in &current.exits {
        if exit == &current.name || exit == "start" {
            continue;
        }

        let mut new_partial_paths = Vec::new();

        for path in &partial_paths {
            if count_small_visited_more_than_once(path) > max_small_visit {
                continue;
            }

            let mut expanded = path.clone();
            expanded.push(exit);
            if exit == "end" {
                final_paths.push(expanded);
            } else {
                new_partial_paths.push(expanded);
            }
        }

        if exit != "end" {
            final_paths.append(&mut find_paths(
                caves.get(exit).unwrap(),
                new_partial_paths,
                &caves,
                max_small_visit,
            ));
        }
    }

    final_paths
}

fn parse_caves(input: String) -> HashMap<String, Cave> {
    let input = input
        .split("\n")
        .map(|line| line.split("-").map(|s| String::from(s)).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut caves: HashMap<String, Cave> = HashMap::new();
    for c in &input {
        let name = c[0].clone();
        let exit = c[1].clone();
        caves
            .entry(name.clone())
            .or_insert(Cave {
                name: name.clone(),
                exits: HashSet::new(),
            })
            .exits
            .insert(exit.clone());
        caves
            .entry(exit.clone())
            .or_insert(Cave {
                name: exit,
                exits: HashSet::new(),
            })
            .exits
            .insert(name);
    }
    caves
}

fn main() {
    let filename = env::args().nth(1).expect("Please provide input file");
    let caves = parse_caves(fs::read_to_string(filename).unwrap());

    let start: &Cave = caves.get("start").unwrap();
    println!(
        "Part 1: {}",
        find_paths(start, vec![vec![&start.name[..]]], &caves, 1).len()
    );
    println!(
        "Part 2: {}",
        find_paths(start, vec![vec![&start.name[..]]], &caves, 2).len()
    );
}
