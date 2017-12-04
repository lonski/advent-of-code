use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() {
    let mut input = String::new();
    let mut file = File::open("input.txt").unwrap();
    file.read_to_string(&mut input).unwrap();

    println!("Part one = {}", part_one(&input));
    println!("Part two = {}", part_two(&input));
}

fn part_one(input: &String) -> usize {
    let mut count = input.lines().count();
    for line in input.lines() {
        let mut map = HashMap::new();
        for word in line.split_whitespace() {
            if map.contains_key(&String::from(word)) {
                count -= 1;
                break;
            }
            map.insert(String::from(word), 1);
        }
    }
    count
}

fn part_two(input: &String) -> usize {
    let mut count = input.lines().count();
    for line in input.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        'outer: for (i, word) in words.iter().enumerate() {
            for j in i + 1..words.len() {
                if is_anagram(word, words[j]) {
                    count -= 1;
                    break 'outer;
                }
            }
        }
    }
    count
}

fn is_anagram(one: &str, two: &str) -> bool {
    if one.len() != two.len() {
        return false;
    }
    let mut one: Vec<char> = one.chars().collect();
    one.sort_by(|a, b| b.cmp(a));
    let mut two: Vec<char> = two.chars().collect();
    two.sort_by(|a, b| b.cmp(a));
    one == two
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn anagram_test() {
        assert_eq!(is_anagram("asd", "sda"), true);
        assert_eq!(is_anagram("aab", "bba"), false);
    }

    #[test]
    fn part_two_tests() {
        assert_eq!(part_two(&String::from("abcde fghij")), 1);
        assert_eq!(part_two(&String::from("abcde xyz ecdab")), 0);
        assert_eq!(part_two(&String::from("a ab abc abd abf abj")), 1);
        assert_eq!(part_two(&String::from("iiii oiii ooii oooi oooo")), 1);
        assert_eq!(part_two(&String::from("oiii ioii iioi iiio")), 0);
        assert_eq!(
            part_two(&String::from(
                "nyot babgr babgr kqtu kqtu kzshonp ylyk psqk",
            )),
            0
        );
    }
}
