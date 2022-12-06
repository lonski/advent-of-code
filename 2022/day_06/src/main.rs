use std::{env, fs};
use std::collections::HashSet;

fn main() {
    let filename = env::args().nth(1).unwrap_or_else(|| String::from("input"));
    let input = fs::read_to_string(filename).unwrap();
    let chars = input.chars().collect::<Vec<char>>();

    println!("Unique 4 at {}", find_unique_sequence(&chars, 4));
    println!("Unique 14 at {}", find_unique_sequence(&chars, 14));
}

fn find_unique_sequence(chars: &Vec<char>, amount: usize) -> usize {
    chars
        .windows(amount)
        .enumerate()
        .filter(|(_, chars)| chars.to_vec().iter().collect::<HashSet<_>>().len() == amount)
        .map(|(n, _)| n + amount)
        .nth(0)
        .unwrap()
}
