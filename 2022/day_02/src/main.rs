extern crate core;

use std::env;
use std::fs;

#[derive(Debug, Copy, Clone)]
enum RoundResult {
    // Z
    Win,
    // X
    Lose,
    // Y
    Draw,
}

// left player, right player, result
const RESULTS_MISUNDERSTOOD: [(&'static str, &'static str, &'static str); 9] = [
    ("A", "X", "Y"),
    ("B", "Y", "Y"),
    ("C", "Z", "Y"),
    ("A", "Y", "Z"),
    ("B", "Z", "Z"),
    ("C", "X", "Z"),
    ("B", "X", "X"),
    ("C", "Y", "X"),
    ("A", "Z", "X"),
];

// left player, right player, result
const RESULTS: [(&'static str, &'static str, &'static str); 9] = [
    ("A", "A", "Y"),
    ("B", "B", "Y"),
    ("C", "C", "Y"),
    ("A", "B", "Z"),
    ("B", "C", "Z"),
    ("C", "A", "Z"),
    ("B", "A", "X"),
    ("C", "B", "X"),
    ("A", "C", "X"),
];


fn main() {
    let filename = env::args().nth(1).unwrap_or_else(|| String::from("input"));
    let input = fs::read_to_string(filename).unwrap();

    let rounds: Vec<(&str, &str)> = input
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| line.split(" ").collect::<Vec<&str>>())
        .map(|round| (round[0], round[1]))
        .collect();

    let score_first: i32 = rounds
        .iter()
        .copied()
        .map(|(left, right)| (right, play(left, right, RESULTS_MISUNDERSTOOD)))
        .map(|(shape, res)| score(shape, res))
        .sum();
    println!("Score: {:?}", score_first);

    let score_second: i32 = rounds
        .iter()
        .copied()
        .map(|(left, right)| (left, choose_shape(left, right)))
        .map(|(left, right)| (right, play(left, right, RESULTS)))
        .map(|(shape, res)| score(shape, res))
        .sum();
    println!("Score: {:?}", score_second);
}

fn choose_shape(left: &str, expected_result: &str) -> &'static str {
    RESULTS.iter()
        .filter(|(_, _, r)| **r == *expected_result)
        .filter(|(l, _, _)| **l == *left)
        .map(|(_, r, _)| *r)
        .collect::<Vec<&str>>()
        .first()
        .unwrap()
}

fn play(left: &str, right: &str, results: [(&'static str, &'static str, &'static str); 9]) -> RoundResult {
    let result = *results
        .iter()
        .filter(|(l, r, _)| **l == *left && **r == *right)
        .map(|(_, _, res)| *res)
        .collect::<Vec<&str>>()
        .first()
        .unwrap();
    parse_result(result)
}

fn parse_result(s: &str) -> RoundResult {
    match s {
        "X" => RoundResult::Lose,
        "Y" => RoundResult::Draw,
        "Z" => RoundResult::Win,
        _ => panic!("Incorrect result letter {}", s)
    }
}

fn score(shape: &str, result: RoundResult) -> i32 {
    let shape_score = match shape {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        "A" => 1,
        "B" => 2,
        "C" => 3,
        _ => panic!("Incorrect shape '{}'", shape)
    };
    let result_score = match result {
        RoundResult::Win => 6,
        RoundResult::Draw => 3,
        RoundResult::Lose => 0
    };

    shape_score + result_score
}
