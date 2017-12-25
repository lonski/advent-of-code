use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut input = String::new();
    let mut file = File::open("input.txt").unwrap();
    file.read_to_string(&mut input).unwrap();

    println!("Part one: {}", do_the_jumping(&input, |_| 1));
    println!(
        "Part two: {}",
        do_the_jumping(&input, |jmp| if jmp > 2 { -1 } else { 1 })
    );

}

fn do_the_jumping<F>(input: &String, modifier: F) -> u32
where
    F: Fn(i32) -> i32,
{
    let mut vec: Vec<i32> = input
        .lines()
        .filter_map(|v| v.parse::<i32>().ok())
        .collect();
    let mut pointer: usize = 0;
    let mut steps = 0;
    while pointer < vec.len() {
        let jmp = vec[pointer];
        vec[pointer] = vec[pointer] + modifier(jmp);
        pointer = (pointer as i32 + jmp) as usize;
        steps += 1;
    }
    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_test() {
        assert_eq!(do_the_jumping(&String::from("0\n3\n0\n1\n-3"), |_| 1), 5);
    }

    #[test]
    fn part_two_test() {
        assert_eq!(
            do_the_jumping(&String::from("0\n3\n0\n1\n-3"), |jmp| if jmp > 2 {
                -1
            } else {
                1
            }),
            10
        );
    }
}
