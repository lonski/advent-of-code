use std::{env, fs};

fn main() {
    let filename = env::args().nth(1).unwrap_or_else(|| String::from("input"));
    let input = fs::read_to_string(filename).unwrap();
    let pairs = input
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(parse_pair);

    println!("Contained: {}", pairs.clone().filter(contained).count());
    println!("Overlaps: {}", pairs.clone().filter(overlaps).count());
}

fn contained(pair: &((u32, u32), (u32, u32))) -> bool {
    let (left, right) = pair;
    (left.0 <= right.0 && left.1 >= right.1) || (right.0 <= left.0 && right.1 >= left.1)
}

fn overlaps(pair: &((u32, u32), (u32, u32))) -> bool {
    let (left, right) = pair;
    !(left.0 > right.1 || right.0 > left.1)
}

fn parse_pair(line: &str) -> ((u32, u32), (u32, u32)) {
    let ranges = line
        .split(",")
        .map(|range| range
            .split("-")
            .map(|n| str::parse::<u32>(n).unwrap())
            .collect::<Vec<u32>>())
        .map(|range| (range[0], range[1]))
        .collect::<Vec<(u32, u32)>>();

    (ranges[0], ranges[1])
}
