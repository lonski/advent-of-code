use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut input = String::new();
    let mut file = File::open("input.txt").unwrap();
    file.read_to_string(&mut input).unwrap();

    let firewall = parse_input(&input);

    println!("Part one: {}", part_one(&firewall));
    println!("Part two: {}", part_two(&firewall));
}

type Depth = u32;
type Layer = u32;

fn parse_input(input: &str) -> Vec<(Layer, Depth)> {
    let mut firewall: Vec<(Layer, Depth)> = input
        .lines()
        .map(|line| {
            line.split(": ")
                .filter_map(|v| v.parse::<u32>().ok())
                .collect::<Vec<u32>>()
        })
        .map(|v| (v[0], v[1]))
        .collect();
    firewall.sort_by_key(|&(_, depth)| depth);
    firewall
}

fn part_two(firewall: &Vec<(Layer, Depth)>) -> u32 {
    (1u32..)
        .filter(|&wait| {
            firewall.iter().all(|&(l, d)| sentry_pos(l + wait, d) != 0)
        })
        .next()
        .unwrap()
}

fn part_one(firewall: &Vec<(Layer, Depth)>) -> u32 {
    firewall
        .iter()
        .map(|&(l, d)| (sentry_pos(l, d), l * d))
        .filter(|&(pos, _)| pos == 0)
        .map(|(_, severity)| severity)
        .sum()
}

fn sentry_pos(turn: u32, depth: u32) -> u32 {
    let x = turn % ((depth * 2) - 2);
    if x < depth {
        x
    } else {
        (depth - 1) - (x - (depth - 1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_tests() {
        let firewall = parse_input("0: 3\n1: 2\n4: 4\n6: 4");
        assert_eq!(part_one(&firewall), 24);
        assert_eq!(part_two(&firewall), 10);
    }
}
