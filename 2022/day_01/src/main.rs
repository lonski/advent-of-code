use std::env;
use std::fs;

fn main() {
    let filename = env::args().nth(1).unwrap_or_else(|| String::from("input"));
    let input = fs::read_to_string(filename).unwrap();

    let mut supplies = input
        .split("\n\n")
        .map(|elf| elf.split("\n").map(|s| s.parse::<i32>().unwrap_or(0)).sum::<i32>())
        .collect::<Vec<i32>>();
    supplies.sort();

    println!("Most callories: {}", supplies.last().unwrap());
    println!("Sum of three most supplied: {}", &supplies[supplies.len() - 3..].iter().sum::<i32>());
}
