use std::collections::HashMap;
use std::env;
use std::fs;

fn parse_initial_occurences(template: &String) -> HashMap<char, usize> {
    let mut occurences = HashMap::new();
    template
        .chars()
        .for_each(|c| *occurences.entry(c).or_insert(0) += 1);

    occurences
}

fn parse_initial_polymers(template: &String) -> HashMap<(char, char), usize> {
    let mut polymers: HashMap<(char, char), usize> = HashMap::new();
    let mut i = 0;
    while i < template.len() - 1 {
        let poly = (
            template.chars().nth(i).unwrap(),
            template.chars().nth(i + 1).unwrap(),
        );
        *polymers.entry(poly).or_insert(0) += 1;
        i += 1;
    }
    polymers
}

fn parse_polymer_manual(input: String) -> (String, HashMap<(char, char), char>) {
    let lines = input.split("\n").collect::<Vec<_>>();
    let tempalate = String::from(lines[0]);
    let mut rules = HashMap::new();

    for rule in lines.into_iter().skip(2) {
        let rule = rule.split(" -> ").collect::<Vec<_>>();
        rules.insert(
            (
                rule[0].chars().nth(0).unwrap(),
                rule[0].chars().nth(1).unwrap(),
            ),
            rule[1].chars().nth(0).unwrap(),
        );
    }

    (tempalate, rules)
}

fn generate_next_polymer(
    rules: &HashMap<(char, char), char>,
    polymers: &HashMap<(char, char), usize>,
    counts: &mut HashMap<char, usize>,
) -> HashMap<(char, char), usize> {
    let mut new_polymers: HashMap<(char, char), usize> = HashMap::new();

    for (polymer, count) in polymers {
        let insert = *rules.get(polymer).unwrap();

        *new_polymers.entry((polymer.0, insert)).or_insert(0) += count;
        *new_polymers.entry((insert, polymer.1)).or_insert(0) += count;

        *counts.entry(insert).or_insert(0) += count;
    }

    new_polymers
}

fn main() {
    let filename = env::args().nth(1).expect("Please provide input file");
    let (template, rules) = parse_polymer_manual(fs::read_to_string(filename).unwrap());
    let mut polymers = parse_initial_polymers(&template);

    for iterations in [10, 40] {
        let mut occurences = parse_initial_occurences(&template);
        for _ in 0..iterations {
            polymers = generate_next_polymer(&rules, &polymers, &mut occurences);
        }
        println!(
            "{} iterations: {}",
            iterations,
            occurences.values().max().unwrap() - occurences.values().min().unwrap()
        );
    }
}
