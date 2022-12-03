use std::{env, fs};

fn main() {
    let filename = env::args().nth(1).unwrap_or_else(|| String::from("input"));
    let input = fs::read_to_string(filename).unwrap();
    let lines = input
        .split("\n")
        .filter(|line| !line.is_empty())
        .collect::<Vec<&str>>();

    println!("Part 1: {}", common_letter_in_one_rucksack_sum(lines.clone()));
    println!("Part 2: {}", common_letter_in_three_rucksacks_sum(lines.clone()));
}

fn common_letter_in_three_rucksacks_sum(lines: Vec<&str>) -> u32 {
    lines
        .chunks(3)
        .map(|c| c.into_iter().map(|v| *v).collect::<Vec<&str>>())
        .map(|chunk| common_letter(chunk))
        .map(|letter| letter_value(&letter) as u32)
        .sum::<u32>()
}

fn common_letter_in_one_rucksack_sum(lines: Vec<&str>) -> u32 {
    lines
        .iter()
        .map(|line| (&line[..line.len() / 2], &line[line.len() / 2..]))
        .map(|(left, right)| common_letter(vec![left, right]))
        .map(|l| letter_value(&l) as u32)
        .sum()
}

fn common_letter(strings: Vec<&str>) -> String {
    let mut vectors = strings
        .iter()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    vectors.iter_mut().for_each(|v| v.sort());

    let mut indices: Vec<u32> = vec![0; vectors.len()];

    loop {
        // check out of range condition
        if indices.iter().enumerate().any(|(i, val)| *val >= vectors[i].len() as u32) {
            panic!("No common letter found in {:?}", strings);
        }

        // map current values
        let values = indices
            .iter()
            .enumerate()
            .map(|(i, val)| vectors[i][*val as usize])
            .collect::<Vec<char>>();

        // check if found common letter
        if values.iter().all(|v| *v == values[0]) {
            return String::from(values[0]);
        }

        // update indices
        let min_value = values.iter().min().unwrap();
        values
            .iter()
            .enumerate()
            .filter(|(_, val)| *val == min_value)
            .map(|(i, _)| i)
            .for_each(|i| indices[i] += 1);
    }
}

fn letter_value(letter: &str) -> u8 {
    match letter.as_bytes()[0] {
        byte @ 0..=96 => byte - 64 + 26,
        byte @ _ => byte - 96
    }
}
