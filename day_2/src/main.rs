use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

fn part_two(input: &String) -> u32 {
    input
        .lines()
        .map(|l| {
            l.split("\t")
                .map(|v| v.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .map(|values| divisable_tuple(&values))
        .map(|t| t.0 / t.1)
        .sum()
}

fn divisable_tuple(values: &Vec<u32>) -> (u32, u32) {
    for v in values {
        for v2 in values {
            if v != v2 {
                if v2 % v == 0 {
                    return (*v2, *v);
                }
                if v % v2 == 0 {
                    return (*v, *v2);
                }
            }
        }
    }
    (0, 0)
}

fn part_one(input: &String) -> u32 {
    input
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
        .sum()
}


#[cfg(test)]
mod tests {


    use super::*;

    #[test]
    fn part_one_test() {
        assert_eq!(
            part_one(&String::from("5\t1\t9\t5\n7\t5\t3\n2\t4\t6\t8")),
            18
        );
    }

    #[test]
    fn part_two_test() {
        assert_eq!(
            part_two(&String::from("5\t9\t2\t8\n9\t4\t7\t3\n3\t8\t6\t5")),
            9
        );
    }
}
