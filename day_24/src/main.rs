use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    let components: Vec<(i32, i32)> = input
        .lines()
        .map(|l| l.split("/"))
        .map(|mut s| (s.next().unwrap(), s.next().unwrap()))
        .map(|t| {
            (t.0.parse::<i32>().unwrap(), t.1.parse::<i32>().unwrap())
        })
        .collect();

    let mut bridges: Vec<(usize, i32)> = build_bridges(&components, 0)
        .iter()
        .map(|b| (b.len(), b.iter().map(|c| c.0 + c.1).sum::<i32>()))
        .collect();

    bridges.sort_by(|a, b| b.1.cmp(&a.1));
    println!("Part one: {:?}", &bridges[0].1);
    bridges.sort_by(|a, b| b.0.cmp(&a.0));
    println!("Part two: {:?}", &bridges[0].1);
}

fn build_bridges(components: &Vec<(i32, i32)>, port: i32) -> Vec<Vec<(i32, i32)>> {
    let mut bridges = Vec::new();
    for (cmp, left) in find_matching(&components, port) {
        let bridge = vec![cmp.clone()];
        if left.iter().any(|c| c.0 == cmp.1 || c.1 == cmp.1) {
            for b in build_bridges(&left, cmp.1) {
                bridges.push([&bridge[..], &b[..]].concat());
            }
        }
        bridges.push(bridge);
    }
    bridges
}

fn find_matching(components: &Vec<(i32, i32)>, port: i32) -> Vec<((i32, i32), Vec<(i32, i32)>)> {
    let mut possibilities = Vec::new();
    for (i, c) in components.iter().enumerate() {
        if c.0 == port || c.1 == port {
            let comp = if c.0 == port { (c.0, c.1) } else { (c.1, c.0) };
            let mut left = components.clone();
            left.remove(i);
            possibilities.push((comp, left));
        }
    }
    possibilities
}
