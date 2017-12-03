use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    let checksum: u32 = input
        .lines()
        .map(|line| {
            (
                line.split("\t")
                    .map(|v| v.parse::<u32>().unwrap())
                    .min()
                    .unwrap(),
                line.split("\t")
                    .map(|v| v.parse::<u32>().unwrap())
                    .max()
                    .unwrap(),
            )
        })
        .map(|t| t.1 - t.0)
        .sum();

    println!("Checksum: {}", checksum);
}
